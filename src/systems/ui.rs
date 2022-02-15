use crate::{
    adapters::share::navigator_share,
    components::{
        networking::{ElapsedTime, PlayerName},
        player::Player2,
        state::AppState,
        ui::{Alert, AlertTimer, Message},
    },
};

use bevy::prelude::*;

use super::networking::{RANDOM_BUTTON, ROOM_BUTTON, SHARE_BUTTON};

pub fn update_state_by_button(mut state: ResMut<State<AppState>>) {
    let mut random = RANDOM_BUTTON.lock();
    if *random == true {
        let _ = state.set(AppState::MatchingRandom);
        *random = false;
    }
    let mut room = ROOM_BUTTON.lock();
    if *room == true {
        let _ = state.set(AppState::CreatingRoom);
        *room = false;
    }
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

pub fn share(
    state: Res<State<AppState>>,
    name: Res<PlayerName<Player2>>,
    mut alert: ResMut<Alert>,
    elapsed: Res<ElapsedTime>,
) {
    let mut button = SHARE_BUTTON.lock();
    if *button == true {
        *button = false;

        let text;
        if *state.current() == AppState::Connected {
            let seconds = elapsed.0.as_secs();
            let minutes = seconds / 60;
            let seconds = seconds % 60;
            text = "With {name}, we've hugged for {minute} minutes and {second} seconds."
                .to_string()
                .replace("{name}", &name.0)
                .replace("{minute}", &format!("{}", minutes))
                .replace("{second}", &format!("{}", seconds));
        } else {
            text = "The Hug game is amazing! Try it out!".to_string();
        }
        navigator_share(&text, "", &mut alert);
    }
}
