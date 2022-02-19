use bevy::prelude::*;
use bevy_rapier3d::physics::PhysicsSystems;

use crate::{
    components::{
        body::part::*,
        player::{Player1, Player2},
    },
    systems::{
        active_ragdoll::{baloon_system, hand_baloon_system},
        control::{angular_spring_system, keyboard_input, move_system, move_system2, touch_input},
    },
    HugSystems,
};

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .label(HugSystems::InputSystem)
                .with_system(keyboard_input.system())
                .with_system(touch_input.system()),
        )
        .add_system_set(
            SystemSet::new()
                .label(HugSystems::MoveSystem)
                .after(HugSystems::InputSystem)
                .with_system(move_system.system())
                .with_system(move_system2.system()),
        )
        .add_system_set(
            SystemSet::new()
                .before(PhysicsSystems::StepWorld)
                .with_system(baloon_system::<Player1, Head>.system())
                .with_system(baloon_system::<Player1, Hip>.system())
                .with_system(hand_baloon_system::<Player1>.system())
                .with_system(baloon_system::<Player2, Head>.system())
                .with_system(baloon_system::<Player2, Hip>.system())
                .with_system(hand_baloon_system::<Player2>.system())
                .with_system(angular_spring_system::<Player1, UpperArmLeft, ForearmLeft>.system())
                .with_system(angular_spring_system::<Player1, ForearmLeft, HandLeft>.system())
                .with_system(angular_spring_system::<Player1, UpperArmRight, ForearmRight>.system())
                .with_system(angular_spring_system::<Player1, ForearmRight, HandRight>.system())
                .with_system(angular_spring_system::<Player1, ThighLeft, ShinLeft>.system())
                .with_system(angular_spring_system::<Player1, ShinLeft, FootLeft>.system())
                .with_system(angular_spring_system::<Player1, ThighRight, ShinRight>.system())
                .with_system(angular_spring_system::<Player1, ShinRight, FootRight>.system())
                .with_system(angular_spring_system::<Player2, UpperArmLeft, ForearmLeft>.system())
                .with_system(angular_spring_system::<Player2, ForearmLeft, HandLeft>.system())
                .with_system(angular_spring_system::<Player2, UpperArmRight, ForearmRight>.system())
                .with_system(angular_spring_system::<Player2, ForearmRight, HandRight>.system())
                .with_system(angular_spring_system::<Player2, ThighLeft, ShinLeft>.system())
                .with_system(angular_spring_system::<Player2, ShinLeft, FootLeft>.system())
                .with_system(angular_spring_system::<Player2, ThighRight, ShinRight>.system())
                .with_system(angular_spring_system::<Player2, ShinRight, FootRight>.system()),
        );
    }
}
