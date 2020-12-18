use crate::sway;

pub trait Widget {
    fn render(&self) -> sway::SwaybarItem;
}
