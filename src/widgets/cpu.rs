use std::io;
use std::fs;
use std::path;

use crate::sway;
use super::widget;

pub struct CpuSpeedWidget {
}

impl CpuSpeedWidget {
    fn read_num_from_file<NumT: std::str::FromStr>(fpath: &path::Path) -> io::Result<NumT> {
        let data = match fs::read_to_string(fpath) {
            Err(err) => return Err(err),
            Ok(val) => val,
        };
        match data.trim().parse::<NumT>() {
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "number can't be parsed")),
            Ok(val) => Ok(val),
        }
    }

    fn get_cpu_speed(idx: u8) -> Option<u32> {
        let fpath = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", idx);
        let fpath = path::Path::new(&fpath);

        match CpuSpeedWidget::read_num_from_file::<u32>(fpath) {
            Err(_) => None,
            Ok(val) => Some(val)
        }
    }

    fn get_cpu_speeds() -> Vec<u32> {
        let mut ret: Vec<u32> = Vec::new();

        let mut i = 0 as u8;
        loop {
            match CpuSpeedWidget::get_cpu_speed(i) {
                None => return ret,
                Some(val) => ret.push(val),
            };

            i += 1;
        }
    }
}

impl widget::Widget for CpuSpeedWidget {
    fn render(&self) -> sway::SwaybarItem {
        let speeds = CpuSpeedWidget::get_cpu_speeds();
        let filter = |vals: Vec<u32>| -> u32 {
            match vals.iter().max() {
                Some(&v) => v,
                None => 0,
            }
        };
        return sway::SwaybarItem::new(
            format!("{:.1}", filter(speeds) as f64 / 1_000.),
            String::from("cpu"),
        );
    }
}
