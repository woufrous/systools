use chrono::prelude::*;
use super::widget;
use crate::sway;

pub struct ClockWidget {
    pub format_str: String,
}

impl widget::Widget for ClockWidget {
    fn render(&self) -> sway::SwaybarItem {
        let lt = Local::now();
        return sway::SwaybarItem::new(
            lt.format(&self.format_str).to_string(),
            String::from("clock"),
        )
    }
}
