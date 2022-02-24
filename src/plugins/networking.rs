use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBodyType, RigidBodyTypeComponent};

use crate::{
    components::{
        networking::{ElapsedTime, IsPrimary, PushTimer, Receiver, Sender, SyncTimer, WaitTimer},
        player::Player2,
        state::AppState,
        ui::Message,
    },
    systems::networking::{
        back_to_alone, elapse_time, event_handlers, handle_event::handle_events, join_room, sync,
        transport, update_name, when_connect,
    },
    HugSystems,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Sender(Vec::new()))
            .insert_resource(Receiver(Vec::new()))
            .insert_resource(PushTimer(Timer::from_seconds(1.0 / 20.0, true)))
            .insert_resource(SyncTimer(Timer::from_seconds(1.0, true)))
            .insert_resource(WaitTimer(Timer::from_seconds(30.0, false)))
            .insert_resource(ElapsedTime(Duration::default()))
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
            )
            .add_system_set(
                SystemSet::on_enter(AppState::Alone).with_system(event_handlers::cleanup.system()),
            )
            .add_system(
                elapse_time
                    .system()
                    .with_run_criteria(when_connect.system()),
            )
            .add_system(back_to_alone.system());
    }
}
