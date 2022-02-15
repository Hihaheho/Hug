use bevy::prelude::*;

use crate::components::{
    control::HandControl,
    networking::{ElapsedTime, HugCommand, Payload, PlayerName, Sender},
    player::{Player1, Player2},
    ui::Message,
};

pub fn random_matching(mut sender: ResMut<Sender>, mut message: ResMut<Message>) {
    sender.0.push(HugCommand::JoinRandom);
    message.0 = "Finding someone to hug.".into()
}

pub fn create_room(mut sender: ResMut<Sender>) {
    sender.0.push(HugCommand::CreateRoom);
}

pub fn cleanup(
    mut name: ResMut<PlayerName<Player2>>,
    mut control: ResMut<HandControl<Player2>>,
    mut time: ResMut<ElapsedTime>,
) {
    name.0 = "".into();
    *control = Default::default();
    time.0 = Default::default();
}

pub fn on_connected(
    mut sender: ResMut<Sender>,
    name: Res<PlayerName<Player1>>,
    mut control: ResMut<HandControl<Player1>>,
) {
    sender.0.push(HugCommand::Push {
        payload: Payload::Name(name.0.clone()),
    });
    *control = HandControl::default();
}
