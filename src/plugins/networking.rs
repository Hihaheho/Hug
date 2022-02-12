use bevy::prelude::*;

use crate::{
    components::{
        networking::{PushTimer, Receiver, Sender},
        state::AppState,
    },
    systems::networking::{
        create_room, handle_events, join_room, push_hand_control, random_matching, receiver,
        sender, when_connect,
    },
    HugSystems,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Sender(Vec::new()))
            .insert_resource(Receiver(Vec::new()))
            .insert_resource(PushTimer(Timer::from_seconds(1.0 / 10.0, true)))
            .add_startup_system(join_room.system())
            .add_system_to_stage(CoreStage::PreUpdate, sender.system())
            .add_system_to_stage(CoreStage::PreUpdate, receiver.system())
            .add_system(handle_events.system().before(HugSystems::MoveSystem))
            .add_system_set(
                SystemSet::on_enter(AppState::MatchingRandom).with_system(random_matching.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::CreatingRoom).with_system(create_room.system()),
            )
            .add_system(
                push_hand_control
                    .system()
                    .after(HugSystems::InputSystem)
                    .with_run_criteria(when_connect.system()),
            );
    }
}
