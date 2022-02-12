use crate::components::state::AppState;

use bevy::prelude::*;

pub fn update_state(mut state: ResMut<State<AppState>>) {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    if let Ok(Some(random)) = storage.get_item("random") {
        if random == "true" {
            let _ = state.set(AppState::MatchingRandom);
        }
    }
    if let Ok(Some(room)) = storage.get_item("room") {
        if room == "true" {
            let _ = state.set(AppState::CreatingRoom);
        }
    }
    storage.set_item("random", "false").unwrap();
    storage.set_item("room", "false").unwrap();
}
