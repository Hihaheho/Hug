use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HugCommand {
    HeartBeat,
    JoinRoom { key: String },
    JoinRandom,
    CreateRoom,
    Leave,
    Push { payload: Payload },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HugEvent {
    Joined,
    RoomCreated { key: String },
    NotFound,
    Push { payload: Payload },
}

#[derive(Serialize, Deserialize)]
pub enum Payload {
    HandControl { left: Vec2, right: Vec2 },
}

pub struct Receiver(pub Vec<HugEvent>);
pub struct Sender(pub Vec<HugCommand>);

pub struct PushTimer(pub Timer);
