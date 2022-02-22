use bevy::prelude::*;

pub struct Message(pub String);
pub struct Alert(pub String);
pub struct AlertTimer(pub Timer);

pub enum Device {
    Mobile,
    Desktop,
}

pub struct Messages {
    pub finding: &'static str,
    pub room_created: &'static str,
    pub ready: &'static str,
    pub finding_room: &'static str,
    pub room_notfound: &'static str,
    pub copied: &'static str,
    pub share: &'static str,
    pub share_match: &'static str,
    pub room_link: &'static str,
    pub tags: &'static str,
}
