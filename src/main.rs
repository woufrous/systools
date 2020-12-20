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

    let clock_widget = widgets::clock::ClockWidget{format_str: String::from("%Y-%m-%d %H:%M:%S")};
    let mut wgs: Vec<Box<dyn widgets::widget::Widget>> = Vec::new();
    wgs.push(Box::new(widgets::cpu::CpuSpeedWidget{}));
    wgs.push(Box::new(widgets::widget::prefix(String::from(""), &clock_widget)));

    loop {
        let mut items: Vec<sway::SwaybarItem> = Vec::new();
        for wg in &wgs {
            items.push(wg.render());
        }

        let jstr = serde_json::to_string(&items)?;
        println!("{},", jstr);
        thread::sleep(time::Duration::new(1, 0));
    }
}
