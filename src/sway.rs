use serde::Serialize;

#[derive(Serialize)]
pub struct SwaybarHeader {
    pub version: i32,
    pub click_events: bool,
}

impl SwaybarHeader {
    pub fn new(click_events: bool) -> SwaybarHeader {
        SwaybarHeader{
            version: 1,
            click_events: click_events,
        }
    }
}

#[derive(Serialize)]
pub struct SwaybarItem {
    pub name: String,
    pub full_text: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,
}

impl SwaybarItem {
    pub fn new(name: String, full_text: String) -> SwaybarItem {
        SwaybarItem {
            name: name,
            full_text: full_text,
            color: None,
        }
    }
}
