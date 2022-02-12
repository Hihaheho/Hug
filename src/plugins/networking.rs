use bevy::prelude::*;

use crate::{
    components::{
        networking::{IsPrimary, PushTimer, Receiver, Sender, SyncTimer},
        state::AppState,
    },
    systems::networking::{
        create_room, handle_events, join_room, push_hand_control, random_matching, receiver, reset,
        sender, sync_parts, when_connect,
    },
    HugSystems,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Sender(Vec::new()))
            .insert_resource(Receiver(Vec::new()))
            .insert_resource(PushTimer(Timer::from_seconds(0.1, true)))
            .insert_resource(SyncTimer(Timer::from_seconds(0.1, true)))
            .insert_resource(IsPrimary::No)
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
            .add_system_set(SystemSet::on_enter(AppState::Connected).with_system(reset.system()))
            .add_system(
                push_hand_control
                    .system()
                    .after(HugSystems::InputSystem)
                    .with_run_criteria(when_connect.system()),
            )
            .add_system(sync_parts.system().with_run_criteria(when_connect.system()));
    }
}
