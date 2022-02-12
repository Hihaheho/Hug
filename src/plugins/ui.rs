use bevy::prelude::*;

use crate::{
    components::ui::{Alert, AlertTimer, Message},
    systems::ui::{remove_alert, update_alert, update_message, update_state},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Alert("".into()))
            .insert_resource(Message("".into()))
            .insert_resource(AlertTimer(Timer::from_seconds(3.0, false)))
            .add_system(update_state.system())
            .add_system(update_message.system())
            .add_system(update_alert.system())
            .add_system(remove_alert.system());
    }
}
