use std::f32::consts::PI;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier3d::prelude::{RigidBodyPosition, RigidBodyPositionComponent};
use js_sys::Function;
use parking_lot::Mutex;
use wasm_bindgen::JsValue;
use web_sys::ShareData;

use crate::components::{
    body::part::*,
    control::HandControl,
    networking::{
        HugCommand, HugEvent, IsPrimary, Payload, PushTimer, Receiver, Sender, SyncTimer,
    },
    player::{Player1, Player2},
    state::AppState,
    ui::{Alert, Message},
};

lazy_static::lazy_static! {
    pub static ref BUFFER: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref SEND_BUFFER: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn sender(mut sender: ResMut<Sender>) {
    let document = web_sys::window().unwrap().document().unwrap();
    let push = unsafe { js_sys::Reflect::get(&document, &"push".into()).unwrap() };
    if push.is_function() {
        let push = Function::from(push);
        if sender.0.len() != 0 {
            for message in sender
                .0
                .split_off(0)
                .into_iter()
                .map(|message| serde_json::to_string(&message).unwrap())
            {
                let _ = push.call1(&JsValue::NULL, &message.into());
            }
        }
    }
}

pub fn receiver(mut receiver: ResMut<Receiver>) {
    let buffer = BUFFER.lock().split_off(0);
    receiver.0 = buffer
        .into_iter()
        .map(|message| serde_json::from_str(&message).unwrap())
        .collect();
}

pub fn random_matching(mut sender: ResMut<Sender>, mut message: ResMut<Message>) {
    sender.0.push(HugCommand::JoinRandom);
    message.0 = "Finding someone to hug.".into()
}

pub fn create_room(mut sender: ResMut<Sender>) {
    sender.0.push(HugCommand::CreateRoom);
}

pub fn when_connect(state: Res<State<AppState>>) -> ShouldRun {
    if *state.current() == AppState::Connected {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

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
            web_sys::console::log_1(&"sync_parts".into());
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

pub fn handle_events(
    receiver: Res<Receiver>,
    mut state: ResMut<State<AppState>>,
    mut control: ResMut<HandControl<Player2>>,
    mut message: ResMut<Message>,
    mut alert: ResMut<Alert>,
    mut position: Query<&mut RigidBodyPositionComponent>,
    mut is_primary_res: ResMut<IsPrimary>,
    player1_head_q: Query<Entity, (With<Player1>, With<Head>)>,
    player2_head_q: Query<Entity, (With<Player2>, With<Head>)>,
    player1_hip_q: Query<Entity, (With<Player1>, With<Hip>)>,
    player2_hip_q: Query<Entity, (With<Player2>, With<Hip>)>,
    player1_hand_left_q: Query<Entity, (With<Player1>, With<HandLeft>)>,
    player2_hand_left_q: Query<Entity, (With<Player2>, With<HandLeft>)>,
    player1_hand_right_q: Query<Entity, (With<Player1>, With<HandRight>)>,
    player2_hand_right_q: Query<Entity, (With<Player2>, With<HandRight>)>,
) {
    for event in receiver.0.iter() {
        match event {
            HugEvent::Joined { is_primary } => {
                message.0 = "Ready to hug".into();
                let _ = state.set(AppState::Connected);
                *is_primary_res = if *is_primary {
                    IsPrimary::Yes
                } else {
                    IsPrimary::No
                };
            }
            HugEvent::RoomCreated { key } => unsafe {
                web_sys::console::log_1(&format!("Room created: {}", key).into());
                let window = web_sys::window().unwrap();
                let navigator = window.navigator();
                let mut data = ShareData::new();
                let url = &format!("https://hug.hihaheho.com?key={}", key);
                data.title("Hug");
                data.text("Hug with Me?");
                data.url(url);

                let mut clipboard = navigator.clipboard();
                let _ = clipboard.write_text(&format!("Hug with Me?\n{}", url));
                if js_sys::Reflect::get(&navigator, &"share".into())
                    .unwrap()
                    .is_function()
                {
                    let _ = navigator.share(data);
                } else {
                    alert.0 = "Copied to clipboard".into();
                }
                message.0 = "Room created share the url to your friends".into();
            },
            HugEvent::NotFound => {
                alert.0 = "Room not found".into();
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
            },
        }
    }
}

pub fn join_room(
    mut sender: ResMut<Sender>,
    mut state: ResMut<State<AppState>>,
    mut message: ResMut<Message>,
) {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    if let Ok(Some(key)) = storage.get_item("key") {
        if key.len() != 0 {
            message.0 = "Joining the room".into();
            sender.0.push(HugCommand::JoinRoom { key });
            let _ = state.set(AppState::MatchingByKey);
        }
    }
}

pub fn reset(mut control: ResMut<HandControl<Player1>>) {
    *control = HandControl::default();
}

fn position_to_vec(position: &RigidBodyPosition) -> Vec3 {
    // rotate y axis 180 degrees
    Vec3::new(
        -position.position.translation.x,
        position.position.translation.y,
        -position.position.translation.z,
    )
}

fn position_vec(position: &mut RigidBodyPositionComponent, vec: &Vec3) {
    position.0.position.translation.x = vec.x;
    position.0.position.translation.y = vec.y;
    position.0.position.translation.z = vec.z;
}
