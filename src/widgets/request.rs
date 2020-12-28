use super::widget;
use crate::sway::SwaybarItem;
use reqwest;

pub struct RequestWidget {
    pub url: String,
}

impl widget::Widget for RequestWidget {
    fn render(&self) -> SwaybarItem {
        let data = match reqwest::blocking::get(&self.url) {
            reqwest::Result::Err(_) => String::from(""),
            reqwest::Result::Ok(resp) => resp.text().unwrap_or(String::from("")),
        };
        SwaybarItem::new(
            String::from("request"),
            String::from(data.trim()),
        )
    }
}
