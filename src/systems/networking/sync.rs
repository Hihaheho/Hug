use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{
    body::part::*,
    control::HandControl,
    networking::{HugCommand, IsPrimary, Payload, PlayerName, PushTimer, Sender, SyncTimer},
    player::{Player1, Player2},
};

pub fn push_hand_control(
    mut sender: ResMut<Sender>,
    hand: Res<HandControl<Player1>>,
    mut timer: ResMut<PushTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let payload = Payload::HandControl {
            left: hand.left().clone(),
            right: hand.right().clone(),
        };
        sender.0.push(HugCommand::Push { payload });
    }
}

pub fn sync_parts(
    mut sender: ResMut<Sender>,
    mut timer: ResMut<SyncTimer>,
    is_primary: Res<IsPrimary>,
    time: Res<Time>,
    player1_head: Query<&RigidBodyPositionComponent, (With<Player1>, With<Head>)>,
    player2_head: Query<&RigidBodyPositionComponent, (With<Player2>, With<Head>)>,
    player1_hip: Query<&RigidBodyPositionComponent, (With<Player1>, With<Hip>)>,
    player2_hip: Query<&RigidBodyPositionComponent, (With<Player2>, With<Hip>)>,
    player1_hand_left: Query<&RigidBodyPositionComponent, (With<Player1>, With<HandLeft>)>,
    player2_hand_left: Query<&RigidBodyPositionComponent, (With<Player2>, With<HandLeft>)>,
    player1_hand_right: Query<&RigidBodyPositionComponent, (With<Player1>, With<HandRight>)>,
    player2_hand_right: Query<&RigidBodyPositionComponent, (With<Player2>, With<HandRight>)>,
) {
    if *is_primary == IsPrimary::Yes {
        if timer.0.tick(time.delta()).just_finished() {
            let player1_head = position_to_vec(&player1_head.single().unwrap().0);
            let player2_head = position_to_vec(&player2_head.single().unwrap().0);
            let player1_hip = position_to_vec(&player1_hip.single().unwrap().0);
            let player2_hip = position_to_vec(&player2_hip.single().unwrap().0);
            let player1_hand_left = position_to_vec(&player1_hand_left.single().unwrap().0);
            let player2_hand_left = position_to_vec(&player2_hand_left.single().unwrap().0);
            let player1_hand_right = position_to_vec(&player1_hand_right.single().unwrap().0);
            let player2_hand_right = position_to_vec(&player2_hand_right.single().unwrap().0);
            let payload = Payload::Sync {
                player1_head,
                player2_head,
                player1_hip,
                player2_hip,
                player1_hand_left,
                player2_hand_left,
                player1_hand_right,
                player2_hand_right,
            };
            sender.0.push(HugCommand::Push { payload });
        }
    }
}

pub fn sync_name(mut sender: ResMut<Sender>, name: Res<PlayerName<Player1>>) {
    sender.0.push(HugCommand::Push {
        payload: Payload::Name(name.0.clone()),
    });
}

fn position_to_vec(position: &RigidBodyPosition) -> Vec3 {
    // rotate y axis 180 degrees
    Vec3::new(
        -position.position.translation.x,
        position.position.translation.y,
        -position.position.translation.z,
    )
}
