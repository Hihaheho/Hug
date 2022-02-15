pub mod event_handlers;
pub mod handle_event;
pub mod sync;
pub mod transport;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use parking_lot::Mutex;

use crate::components::{
    networking::{ElapsedTime, HugCommand, PlayerName, Sender},
    player::Player1,
    state::AppState,
    ui::Message,
};

lazy_static::lazy_static! {
    pub static ref BUFFER: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref SEND_BUFFER: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref NAME: Mutex<String> = Mutex::new("nameless".into());
    pub static ref RANDOM_BUTTON: Mutex<bool> = Mutex::new(false);
    pub static ref ROOM_BUTTON: Mutex<bool> = Mutex::new(false);
    pub static ref SHARE_BUTTON: Mutex<bool> = Mutex::new(false);
}

pub fn when_connect(state: Res<State<AppState>>) -> ShouldRun {
    if *state.current() == AppState::Connected {
        ShouldRun::Yes
    } else {
        ShouldRun::No
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

pub fn update_name(mut name: ResMut<PlayerName<Player1>>) {
    let name_value = NAME.lock();
    if *name_value != name.0 {
        name.0 = name_value.clone();
    }
}

pub fn elapse_time(mut elapsed_time: ResMut<ElapsedTime>, time: Res<Time>) {
    elapsed_time.0 += time.delta();
}
