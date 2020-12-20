use crate::sway;

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
