use std::fs;
use std::path::Path;
use chrono;

use crate::sway;
use super::widget::Widget;
use super::utils;

pub struct BatteryWidget {
    //basepath: &'static str,
}

enum BatteryState {
    Charging,
    Discharging,
    Full,
    Unknown,
}

impl BatteryState {
    fn from_str(text: &str) -> Self {
        match text {
            "Full" => BatteryState::Full,
            "Discharging" => BatteryState::Discharging,
            "Charging" => BatteryState::Charging,
            _ => BatteryState::Unknown,
        }
    }
    fn to_str(&self) -> &str {
        match self {
            BatteryState::Discharging => "Discharging",
            BatteryState::Charging => "Charging",
            BatteryState::Full => "Full",
            BatteryState::Unknown => "Unknown",
        }
    }
}

impl BatteryWidget {
    fn get_charge_full(&self) -> f64 {
        let path = Path::new("/sys/class/power_supply/BAT0/charge_full");
        match utils::read_num_from_file::<f64>(path) {
            Err(_) => -1f64,
            Ok(val) => val,
        }
    }
    fn get_charge_now(&self) -> f64 {
        let path = Path::new("/sys/class/power_supply/BAT0/charge_now");
        match utils::read_num_from_file::<f64>(path) {
            Err(_) => -1f64,
            Ok(val) => val,
        }
    }

    fn get_remaining_runtime(&self) -> chrono::Duration {
        let path = Path::new("/sys/class/power_supply/BAT0/current_now");
        let current = match utils::read_num_from_file::<f64>(path) {
            Err(_) => return chrono::Duration::hours(99),
            Ok(val) => val,
        };
        let charge = self.get_charge_now();

        chrono::Duration::minutes(((charge/current)*60.).round() as i64)
    }

    fn get_state(&self) -> BatteryState {
        let path = Path::new("/sys/class/power_supply/BAT0/status");
        let state_str = match fs::read_to_string(path) {
            Err(_) => return BatteryState::Unknown,
            Ok(val) => val,
        };
        BatteryState::from_str(state_str.trim())
    }
}

impl Widget for BatteryWidget {
    fn render(&self) -> sway::SwaybarItem {
        let mut full_text = String::from(format!("{:.0}%", (100.*self.get_charge_now()/self.get_charge_full()).round()));
        if let BatteryState::Discharging = self.get_state() {
            let runtime = self.get_remaining_runtime();
            let rt_string = format!(" ({:02}:{:02})", runtime.num_hours(), runtime.num_minutes()%60);
            full_text.push_str(&rt_string);
        }

        sway::SwaybarItem::new(
            String::from("battery"),
            full_text,
        )
    }
}
