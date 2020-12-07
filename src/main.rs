use std::fs;
use std::option::Option;
use std::vec::Vec;

fn get_cpu_speed(idx: u8) -> Option<u32> {
    let fpath = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", idx);

    match fs::read_to_string(fpath) {
        Err(_) => None,
        Ok(val) => match val.trim().parse::<u32>() {
            Err(_) => None,
            Ok(val) => Some(val),
        },
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
