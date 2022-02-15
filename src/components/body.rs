pub mod part;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use bevy::{ecs::component::Component, prelude::*};

use super::player::Player;
use part::*;

pub trait BodyPart: Component + Default {}

pub const HEIGHT: f32 = 1.7;
pub const RATIO: f32 = HEIGHT / 15.0;
pub const TORSO_WIDTH: f32 = 4.0 * RATIO;
pub const TORSO_THICKNESS: f32 = 1.4 * RATIO;
// pub const TORSO_ROUNDNESS: f32 = 0.1;
pub const NECK_RADIUS: f32 = 0.8 * RATIO;
pub const HEAD_WIDTH: f32 = 2.8 * RATIO;
pub const HEAD_HEIGHT: f32 = 2.8 * RATIO;
pub const HEAD_THICKNESS: f32 = 2.0 * RATIO;
pub const ARM_RADIUS: f32 = 0.55 * RATIO;
pub const LEG_RADIUS: f32 = 0.8 * RATIO;
pub const SHIN_LENGTH: f32 = 3.5 * RATIO;
pub const THIGH_LENGTH: f32 = 3.5 * RATIO;
pub const SHOULDER_LENGTH: f32 = 2.1 * RATIO;
pub const FOREARM_LENGTH: f32 = 3.0 * RATIO;
pub const UPPER_ARM_LENGTH: f32 = 3.0 * RATIO;
pub const PELVIS_LENGTH: f32 = 1.5 * RATIO;

pub struct PlayerBody<T> {
    pub relative: Body,
    pub absolute: Body,
    data: PhantomData<T>,
}

impl<T: Player> PlayerBody<T> {
    pub fn new(body: Body, propagated: Body) -> Self {
        PlayerBody {
            relative: body,
            absolute: propagated,
            data: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Body(HashMap<TypeId, Transform>);

impl Body {
    pub fn player1() -> Self {
        Self::player(1.0)
    }
    pub fn player2() -> Self {
        Self::player(-1.0)
    }
    fn player(x: f32) -> Self {
        let mut body = Self(HashMap::new());
        body.0
            .insert(Hip.type_id(), Transform::from_xyz(0.0, 7. * RATIO, 0.0));
        body.0
            .insert(Spine.type_id(), Transform::from_xyz(0.0, 2.5 * RATIO, 0.0));
        body.0
            .insert(Chest.type_id(), Transform::from_xyz(0.0, 2.5 * RATIO, 0.0));
        body.0
            .insert(Neck.type_id(), Transform::from_xyz(0.0, 0.8 * RATIO, 0.0));
        body.0
            .insert(Head.type_id(), Transform::from_xyz(0.0, HEAD_HEIGHT, 0.0));
        body.0.insert(
            UpperArmLeft.type_id(),
            Transform::from_xyz(x * -SHOULDER_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            ForearmLeft.type_id(),
            Transform::from_xyz(x * -UPPER_ARM_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            HandLeft.type_id(),
            Transform::from_xyz(x * -FOREARM_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            UpperArmRight.type_id(),
            Transform::from_xyz(x * SHOULDER_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            ForearmRight.type_id(),
            Transform::from_xyz(x * UPPER_ARM_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            HandRight.type_id(),
            Transform::from_xyz(x * FOREARM_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            ThighLeft.type_id(),
            Transform::from_xyz(x * -PELVIS_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            ShinLeft.type_id(),
            Transform::from_xyz(0.0, -THIGH_LENGTH, 0.0),
        );
        body.0.insert(
            FootLeft.type_id(),
            Transform::from_xyz(0.0, -SHIN_LENGTH, 0.0),
        );
        body.0.insert(
            ThighRight.type_id(),
            Transform::from_xyz(x * PELVIS_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            ShinRight.type_id(),
            Transform::from_xyz(0.0, -THIGH_LENGTH, 0.0),
        );
        body.0.insert(
            FootRight.type_id(),
            Transform::from_xyz(0.0, -SHIN_LENGTH, 0.0),
        );
        body
    }
}

impl Body {
    pub fn get<T: 'static>(&self) -> &Transform {
        self.0.get(&TypeId::of::<T>()).unwrap()
    }
    pub fn get_mut<T: 'static>(&mut self) -> &mut Transform {
        self.0.get_mut(&TypeId::of::<T>()).unwrap()
    }
    pub fn propagated(&self) -> Self {
        let mut body = Self(HashMap::new());

        let hip = self.get::<Hip>().clone();
        body.0.insert(Hip.type_id(), hip.clone());

        let spine = hip * self.get::<Spine>().clone();
        body.0.insert(Spine.type_id(), spine.clone());

        let chest = spine * self.get::<Chest>().clone();
        body.0.insert(Chest.type_id(), chest.clone());

        let neck = chest * self.get::<Neck>().clone();
        body.0.insert(Neck.type_id(), neck.clone());

        let head = neck * self.get::<Head>().clone();
        body.0.insert(Head.type_id(), head.clone());

        let upper_arm_left = chest * self.get::<UpperArmLeft>().clone();
        body.0
            .insert(UpperArmLeft.type_id(), upper_arm_left.clone());

        let forearm_left = upper_arm_left * self.get::<ForearmLeft>().clone();
        body.0.insert(ForearmLeft.type_id(), forearm_left.clone());

        let hand_left = forearm_left * self.get::<HandLeft>().clone();
        body.0.insert(HandLeft.type_id(), hand_left.clone());

        let upper_arm_right = chest * self.get::<UpperArmRight>().clone();
        body.0
            .insert(UpperArmRight.type_id(), upper_arm_right.clone());

        let forearm_right = upper_arm_right * self.get::<ForearmRight>().clone();
        body.0.insert(ForearmRight.type_id(), forearm_right.clone());

        let hand_right = forearm_right * self.get::<HandRight>().clone();
        body.0.insert(HandRight.type_id(), hand_right.clone());

        let thigh_left = hip * self.get::<ThighLeft>().clone();
        body.0.insert(ThighLeft.type_id(), thigh_left.clone());

        let shin_left = thigh_left * self.get::<ShinLeft>().clone();
        body.0.insert(ShinLeft.type_id(), shin_left.clone());

        let foot_left = shin_left * self.get::<FootLeft>().clone();
        body.0.insert(FootLeft.type_id(), foot_left.clone());

        let thigh_right = hip * self.get::<ThighRight>().clone();
        body.0.insert(ThighRight.type_id(), thigh_right.clone());

        let shin_right = thigh_right * self.get::<ShinRight>().clone();
        body.0.insert(ShinRight.type_id(), shin_right.clone());

        let foot_right = shin_right * self.get::<FootRight>().clone();
        body.0.insert(FootRight.type_id(), foot_right.clone());

        body
    }
}
