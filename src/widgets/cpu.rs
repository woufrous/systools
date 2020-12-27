use std::path::Path;

use crate::sway;
use super::utils;
use super::widget;

pub struct CpuSpeedWidget {
}

impl CpuSpeedWidget {
    fn get_cpu_speed(idx: u8) -> Option<u32> {
        let fpath = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", idx);
        let fpath = Path::new(&fpath);

        match utils::read_num_from_file::<u32>(fpath) {
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
            String::from("cpu"),
            format!("{:.1}", filter(speeds) as f64 / 1_000.),
        );
    }
}
