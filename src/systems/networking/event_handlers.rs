use bevy::{prelude::*, render::wireframe::Wireframe};

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
    *control = HandControl::default_absent();
    time.0 = Default::default();
}

pub fn alone(
    mut commands: Commands,
    mut message: ResMut<Message>,
    mut control: ResMut<HandControl<Player2>>,
    query: Query<Entity, (With<Player2>, With<Handle<Mesh>>)>,
) {
    message.0 = "You are alone in this room. Click Random to find someone or Invite to hug with your friend.".into();
    *control = HandControl::default_absent();
    for entity in query.iter() {
        commands.entity(entity).insert(Wireframe);
    }
}

pub fn on_connected(
    mut commands: Commands,
    mut sender: ResMut<Sender>,
    name: Res<PlayerName<Player1>>,
    mut control: ResMut<HandControl<Player1>>,
    query: Query<Entity, (With<Player2>, With<Handle<Mesh>>)>,
) {
    sender.0.push(HugCommand::Push {
        payload: Payload::Name(name.0.clone()),
    });
    *control = HandControl::default();
    for entity in query.iter() {
        commands.entity(entity).remove::<Wireframe>();
    }
}
