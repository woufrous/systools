use std::path::Path;

use crate::sway;
use super::widget::Widget;
use super::utils;

pub struct BatteryWidget {
    //basepath: &'static str,
}

impl BatteryWidget {
    //pub fn new() -> Self {
    //    BatteryWidget {
    //        basepath: "/sys/class/power_supply/BAT0",
    //    }
    //}
    fn get_charge_full(&self) -> f64 {
        let path = Path::new("/sys/class/power_supply/BAT0/charge_full");
        match utils::read_num_from_file::<u32>(path) {
            Err(_) => 0f64,
            Ok(val) => val as f64,
        }
    }
    fn get_charge_now(&self) -> f64 {
        let path = Path::new("/sys/class/power_supply/BAT0/charge_now");
        match utils::read_num_from_file::<u32>(path) {
            Err(_) => 0f64,
            Ok(val) => val as f64,
        }
    }
}

impl Widget for BatteryWidget {
    fn render(&self) -> sway::SwaybarItem {
        sway::SwaybarItem::new(
            String::from("battery"),
            String::from(format!("{:.0}%", (100.*self.get_charge_now()/self.get_charge_full()).round())),
        )
    }
}
