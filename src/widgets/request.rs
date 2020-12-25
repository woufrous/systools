use super::widget;
use crate::sway::SwaybarItem;
use reqwest;

pub struct RequestWidget {
    pub url: String,
}

impl widget::Widget for RequestWidget {
    fn render(&self) -> SwaybarItem {
        let data = reqwest::blocking::get(&self.url).unwrap().text().unwrap();
        SwaybarItem::new(
            String::from("request"),
            String::from(data.trim()),
        )
    }
}
