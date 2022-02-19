use bevy::{prelude::*, render::wireframe::Wireframe};
use bevy_rapier3d::prelude::{RigidBodyActivation, RigidBodyType, RigidBodyTypeComponent};

use crate::components::{
    body::part::Head,
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    head: Query<&Handle<StandardMaterial>, (With<Player2>, With<Head>)>,
) {
    name.0 = "".into();
    *control = HandControl::default_absent();
    time.0 = Default::default();
    let material = materials.get_mut(head.single().unwrap()).unwrap();
    material.unlit = true;
}

pub fn on_connected(
    mut commands: Commands,
    mut sender: ResMut<Sender>,
    name: Res<PlayerName<Player1>>,
    mut control: ResMut<HandControl<Player1>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut RigidBodyTypeComponent, With<Player2>>,
    head: Query<&Handle<StandardMaterial>, (With<Player2>, With<Head>)>,
) {
    sender.0.push(HugCommand::Push {
        payload: Payload::Name(name.0.clone()),
    });
    *control = HandControl::default();
    let material = materials.get_mut(head.single().unwrap()).unwrap();
    material.unlit = false;
    for mut rigid_body_type in query.iter_mut() {
        rigid_body_type.0 = RigidBodyType::Dynamic;
    }
}
