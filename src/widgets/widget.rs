use crate::sway;
use chrono;
use std::cell::RefCell;

pub trait Widget {
    fn render(&self) -> sway::SwaybarItem;
}

pub struct PrefixedWidget<'a, W>
where
    W: Widget
{
    prefix: String,
    widget: &'a W,
}

impl<'a, W: Widget> Widget for PrefixedWidget<'a, W> {
    fn render(&self) -> sway::SwaybarItem {
        let item = self.widget.render();
        sway::SwaybarItem {
            full_text: format!("{} {}", self.prefix, item.full_text),
            ..item
        }
    }
}

pub fn prefix<'a, W: Widget>(prefix: String, widget: &'a W) -> PrefixedWidget<'a, W> {
    PrefixedWidget {
        prefix,
        widget,
    }
}

pub struct CachedWidget<'a, W>
where
    W: Widget
{
    timeout: chrono::Duration,
    widget: &'a W,

    cache: RefCell<sway::SwaybarItem>,
    last_updated: RefCell<chrono::DateTime<chrono::Utc>>,
}

impl<'a, W: Widget> Widget for CachedWidget<'a, W> {
    fn render(&self) -> sway::SwaybarItem {
        if chrono::Utc::now() >= *self.last_updated.borrow() + self.timeout {
            let mut last_updated_ = self.last_updated.borrow_mut();
            *last_updated_ = chrono::Utc::now();
            //self.cache.replace_with(|mut& old| self.widget.render().clone());
            let mut cache_ = self.cache.borrow_mut();
            *cache_ = self.widget.render().clone();
        }
        self.cache.borrow().clone()
    }
}

pub fn cached<'a, W: Widget>(timeout: chrono::Duration, widget: &'a W) -> CachedWidget<'a, W> {
    CachedWidget {
        timeout,
        widget,
        cache: RefCell::new(widget.render()),
        last_updated: RefCell::new(chrono::Utc::now()),
    }
}
