use std::vec::Vec;
use std::io;
use std::time;
use std::thread;

mod widgets;
mod sway;

fn main() -> io::Result<()> {
    let header = sway::SwaybarHeader::new(false);
    let jhdr = serde_json::to_string(&header)?;
    println!("{}\n[", jhdr);
    loop {

        let mut items: Vec<sway::SwaybarItem> = Vec::new();
        for speed in widgets::cpu::get_cpu_speeds() {
            items.push(sway::SwaybarItem::new(
                String::from("cpu_speed"),
                format!("{:.1}", speed as f64 / 1_000.),
            ));
        }

        items.push(sway::SwaybarItem::new(
            String::from("clock"),
            widgets::clock::get_clock(),
        ));

        let jstr = serde_json::to_string(&items)?;
        println!("{},", jstr);
        thread::sleep(time::Duration::new(1, 0));
    }
}
