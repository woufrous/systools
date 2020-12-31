use super::widget;
use crate::sway::SwaybarItem;
use reqwest;

pub struct RequestWidget<'a> {
    url: &'a str,
}
impl<'a> RequestWidget<'a> {
    pub fn new(url: &'a str) -> Self {
        Self {
            url,
        }
    }
}

impl<'a> widget::Widget for RequestWidget<'a> {
    fn render(&self) -> SwaybarItem {
        let data = match reqwest::blocking::get(self.url) {
            reqwest::Result::Err(_) => String::from(""),
            reqwest::Result::Ok(resp) => resp.text().unwrap_or(String::from("")),
        };
        SwaybarItem::new(
            String::from("request"),
            String::from(data.trim()),
        )
    }
}
