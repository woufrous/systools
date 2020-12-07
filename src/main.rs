use std::fs;
use std::option::Option;
use std::vec::Vec;
use std::path;
use std::io;

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

    match read_num_from_file::<u32>(fpath) {
        Err(_) => None,
        Ok(val) => Some(val)
    }
}

fn get_cpu_speeds() -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();

    let mut i = 0 as u8;
    loop {
        match get_cpu_speed(i) {
            None => return ret,
            Some(val) => ret.push(val),
        };

        i += 1;
    }
}

fn main() {
    for speed in get_cpu_speeds() {
        println!("{:>4.0} MHz", speed as f64 / 1_000.);
    }
}
