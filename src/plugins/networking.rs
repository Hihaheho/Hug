use bevy::prelude::*;

use crate::{
    components::{
        networking::{IsPrimary, PushTimer, Receiver, Sender, SyncTimer},
        state::AppState,
    },
    systems::networking::{
        event_handlers, handle_event::handle_events, join_room, sync, transport, update_name,
        when_connect,
    },
    HugSystems,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Sender(Vec::new()))
            .insert_resource(Receiver(Vec::new()))
            .insert_resource(PushTimer(Timer::from_seconds(0.2, true)))
            .insert_resource(SyncTimer(Timer::from_seconds(1.0, true)))
            .insert_resource(IsPrimary::No)
            .add_startup_system(join_room.system())
            .add_system_to_stage(CoreStage::PreUpdate, transport::sender.system())
            .add_system_to_stage(CoreStage::PreUpdate, transport::receiver.system())
            .add_system(handle_events.system().before(HugSystems::MoveSystem))
            .add_system_set(
                SystemSet::on_enter(AppState::MatchingRandom)
                    .with_system(event_handlers::random_matching.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::CreatingRoom)
                    .with_system(event_handlers::create_room.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::Connected)
                    .with_system(event_handlers::on_connected.system()),
            )
            .add_system(
                sync::push_hand_control
                    .system()
                    .after(HugSystems::InputSystem)
                    .with_run_criteria(when_connect.system()),
            )
            .add_system(
                sync::sync_parts
                    .system()
                    .with_run_criteria(when_connect.system()),
            )
            .add_system(
                sync::sync_name
                    .system()
                    .with_run_criteria(when_connect.system()),
            )
            .add_system(update_name.system())
            .add_system_set(
                SystemSet::on_exit(AppState::Connected)
                    .with_system(event_handlers::cleanup.system()),
            );
    }
}
