use crate::components::{
    state::AppState,
    ui::{Alert, AlertTimer, Message},
};

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

pub fn update_message(message: Res<Message>) {
    if message.is_changed() {
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .query_selector("#message")
            .unwrap()
            .unwrap()
            .set_inner_html(&message.0);
    }
}

pub fn update_alert(alert: Res<Alert>, mut timer: ResMut<AlertTimer>) {
    if alert.is_changed() {
        timer.0.reset();
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .query_selector("#alert")
            .unwrap()
            .unwrap()
            .set_inner_html(&alert.0);
    }
}

pub fn remove_alert(mut timer: ResMut<AlertTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .query_selector("#alert")
            .unwrap()
            .unwrap()
            .set_inner_html("");
    }
}
