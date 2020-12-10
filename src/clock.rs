pub mod clock {
    extern crate chrono;
    use chrono::prelude::*;

    pub fn get_clock() -> String {
        let lt = Local::now();
        lt.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
