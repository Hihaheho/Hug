use bevy::{ecs::schedule::ShouldRun, prelude::*};
use js_sys::Function;
use parking_lot::Mutex;
use wasm_bindgen::JsValue;
use web_sys::ShareData;

use crate::components::{
    control::HandControl,
    networking::{HugCommand, HugEvent, Payload, PushTimer, Receiver, Sender},
    player::{Player1, Player2},
    state::AppState,
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
                push.call1(&JsValue::NULL, &message.into());
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

pub fn random_matching(mut sender: ResMut<Sender>) {
    sender.0.push(HugCommand::JoinRandom);
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

pub fn handle_events(
    receiver: Res<Receiver>,
    mut state: ResMut<State<AppState>>,
    mut control: ResMut<HandControl<Player2>>,
) {
    for event in receiver.0.iter() {
        match event {
            HugEvent::Joined => {
                unsafe {
                    web_sys::console::log_1(&format!("connected").into());
                }
                state.set(AppState::Connected).unwrap();
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
                if js_sys::Reflect::get(&navigator, &"share".into())
                    .unwrap()
                    .is_function()
                {
                    let _ = navigator.share(data);
                } else {
                    let mut clipboard = navigator.clipboard();
                    clipboard.write_text(&format!("Hug with Me?\n{}", url));
                    window.alert_with_message("Copied to clipboard").unwrap();
                }
                // }
            },
            HugEvent::NotFound => {
                unsafe {
                    web_sys::console::log_1(&format!("Room not found").into());
                }
                // TODO: show
            }
            HugEvent::Push { payload } => match payload {
                Payload::HandControl { left, right } => {
                    control.set_left(left.clone());
                    control.set_right(right.clone());
                }
            },
        }
    }
}

pub fn join_room(mut sender: ResMut<Sender>, mut state: ResMut<State<AppState>>) {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    if let Ok(Some(key)) = storage.get_item("key") {
        if key.len() != 0 {
            web_sys::console::log_1(&format!("key {}", key).into());
            sender.0.push(HugCommand::JoinRoom { key });
            let _ = state.set(AppState::MatchingByKey);
        }
    }
}
