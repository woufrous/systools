use std::process::Command;

use crate::sway;
use super::widget;

pub struct CommandWidget {
    pub command: String,
    pub args: Vec<String>,
}


impl widget::Widget for CommandWidget {
    fn render(&self) -> sway::SwaybarItem {
        let output = Command::new(&self.command)
            .arg(&self.args[0])
            .output();
        let output = match output {
            Err(_) => String::from("Error"),
            Ok(val) => String::from(std::str::from_utf8(&val.stdout).unwrap()),
        };
        return sway::SwaybarItem::new(
            String::from("cpu"),
            String::from(&output),
        );
    }
}
