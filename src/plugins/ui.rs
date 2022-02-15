use bevy::prelude::*;

use crate::{
    components::{
        body::part::*,
        networking::PlayerName,
        player::{Player1, Player2},
        ui::{Alert, AlertTimer, Message},
    },
    systems::{
        name::{insert_name, load_font, update_name, update_name_position},
        ui::{remove_alert, update_alert, update_message, update_state},
    },
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Alert("".into()))
            .insert_resource(Message("".into()))
            .insert_resource(AlertTimer(Timer::from_seconds(3.0, false)))
            .insert_resource(PlayerName::<Player1>::default())
            .insert_resource(PlayerName::<Player2>::default())
            .add_system(update_state.system())
            .add_system(update_message.system())
            .add_system(update_alert.system())
            .add_system(remove_alert.system())
            .add_startup_system(load_font.system())
            .add_system(insert_name::<Player1>.system())
            .add_system(insert_name::<Player2>.system())
            .add_system(update_name::<Player1>.system())
            .add_system(update_name::<Player2>.system())
            .add_system(update_name_position::<Player1, UpperArmRight, true>.system())
            .add_system(update_name_position::<Player2, UpperArmLeft, false>.system());
    }
}
