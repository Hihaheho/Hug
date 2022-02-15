use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::player::Player;

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
    Joined { is_primary: bool },
    RoomCreated { key: String },
    NotFound,
    Push { payload: Payload },
}

#[derive(Serialize, Deserialize)]
pub enum Payload {
    HandControl {
        left: Vec2,
        right: Vec2,
    },
    Sync {
        player1_head: Vec3,
        player2_head: Vec3,
        player1_hip: Vec3,
        player2_hip: Vec3,
        player1_hand_left: Vec3,
        player2_hand_left: Vec3,
        player1_hand_right: Vec3,
        player2_hand_right: Vec3,
    },
    Name(String),
}

pub struct Receiver(pub Vec<HugEvent>);
pub struct Sender(pub Vec<HugCommand>);

pub struct PushTimer(pub Timer);
pub struct SyncTimer(pub Timer);

#[derive(PartialEq, Eq)]
pub enum IsPrimary {
    Yes,
    No,
}

#[derive(Default)]
pub struct PlayerName<P: Player>(pub String, PhantomData<P>);

pub struct ElapsedTime(pub Duration);
