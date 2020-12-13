use std::vec::Vec;
use std::io;
use std::time;
use std::thread;

use serde::Serialize;

mod widgets;

#[derive(Serialize)]
struct SwaybarHeader {
    version: i32,
    click_events: bool,
}

#[derive(Serialize)]
struct SwaybarItem {
    name: String,
    full_text: String,
}

fn main() -> io::Result<()> {

    let header = SwaybarHeader {
        version: 1,
        click_events: false,
    };
    let jhdr = serde_json::to_string(&header)?;
    println!("{}\n[", jhdr);
    loop {

        let mut items: Vec<SwaybarItem> = Vec::new();
        for speed in widgets::cpu::get_cpu_speeds() {
            items.push(SwaybarItem{
                name: String::from("cpu_speed"),
                full_text: format!("{:.1}", speed as f64 / 1_000.),
            });
        }

        items.push(SwaybarItem{
            name: String::from("clock"),
            full_text: widgets::clock::get_clock(),
        });

        let jstr = serde_json::to_string(&items)?;
        println!("{},", jstr);
        thread::sleep(time::Duration::new(1, 0));
    }
}
