use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    adapters::share::navigator_share,
    components::{
        body::part::*,
        control::HandControl,
        networking::{HugEvent, IsPrimary, Payload, PlayerName, Receiver},
        player::{Player1, Player2},
        state::AppState,
        ui::{Alert, Message, Messages},
    },
};

pub fn handle_events(
    receiver: (Res<Receiver>, Res<Messages>),
    mut state: ResMut<State<AppState>>,
    mut control: ResMut<HandControl<Player2>>,
    mut message: ResMut<Message>,
    mut alert: ResMut<Alert>,
    mut position: Query<&mut RigidBodyPositionComponent>,
    mut is_primary_res: ResMut<IsPrimary>,
    mut name2: ResMut<PlayerName<Player2>>,
    player1_head_q: Query<Entity, (With<Player1>, With<Head>)>,
    player2_head_q: Query<Entity, (With<Player2>, With<Head>)>,
    player1_hip_q: Query<Entity, (With<Player1>, With<Hip>)>,
    player2_hip_q: Query<Entity, (With<Player2>, With<Hip>)>,
    player1_hand_left_q: Query<Entity, (With<Player1>, With<HandLeft>)>,
    player2_hand_left_q: Query<Entity, (With<Player2>, With<HandLeft>)>,
    player1_hand_right_q: Query<Entity, (With<Player1>, With<HandRight>)>,
    player2_hand_right_q: Query<Entity, (With<Player2>, With<HandRight>)>,
) {
    let (receiver, messages) = receiver;
    for event in receiver.0.iter() {
        match event {
            HugEvent::Joined { is_primary } => {
                message.0 = messages.ready.into();
                let _ = state.set(AppState::Connected);
                *is_primary_res = if *is_primary {
                    IsPrimary::Yes
                } else {
                    IsPrimary::No
                };
            }
            HugEvent::RoomCreated { key } => unsafe {
                let url = &format!("?key={}", key);
                navigator_share(messages.room_link, url, &mut alert, &messages);
                message.0 = messages.room_created.into();
            },
            HugEvent::NotFound => {
                alert.0 = messages.room_notfound.into();
            }
            HugEvent::Push { payload } => match payload {
                Payload::HandControl { left, right } => {
                    control.set_left(left.clone());
                    control.set_right(right.clone());
                }
                Payload::Sync {
                    // Swap player1 and player2
                    player1_head: player2_head,
                    player2_head: player1_head,
                    player1_hip: player2_hip,
                    player2_hip: player1_hip,
                    player1_hand_left: player2_hand_left,
                    player2_hand_left: player1_hand_left,
                    player1_hand_right: player2_hand_right,
                    player2_hand_right: player1_hand_right,
                } => {
                    position_vec(
                        &mut position.get_mut(player1_head_q.single().unwrap()).unwrap(),
                        player1_head,
                    );
                    position_vec(
                        &mut position.get_mut(player2_head_q.single().unwrap()).unwrap(),
                        player2_head,
                    );
                    position_vec(
                        &mut position.get_mut(player1_hip_q.single().unwrap()).unwrap(),
                        player1_hip,
                    );
                    position_vec(
                        &mut position.get_mut(player2_hip_q.single().unwrap()).unwrap(),
                        player2_hip,
                    );
                    position_vec(
                        &mut position
                            .get_mut(player1_hand_left_q.single().unwrap())
                            .unwrap(),
                        player1_hand_left,
                    );
                    position_vec(
                        &mut position
                            .get_mut(player2_hand_left_q.single().unwrap())
                            .unwrap(),
                        player2_hand_left,
                    );
                    position_vec(
                        &mut position
                            .get_mut(player1_hand_right_q.single().unwrap())
                            .unwrap(),
                        player1_hand_right,
                    );
                    position_vec(
                        &mut position
                            .get_mut(player2_hand_right_q.single().unwrap())
                            .unwrap(),
                        player2_hand_right,
                    );
                }
                Payload::Name(name) => {
                    name2.0 = name.clone();
                }
            },
        }
    }
}

fn position_vec(position: &mut RigidBodyPositionComponent, vec: &Vec3) {
    position.0.next_position.translation.x = vec.x;
    position.0.next_position.translation.y = vec.y;
    position.0.next_position.translation.z = vec.z;
}
