use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use bevy::prelude::*;

#[derive(Component)]
pub struct Hip;
#[derive(Component)]
pub struct Spine;
#[derive(Component)]
pub struct Chest;
#[derive(Component)]
pub struct Neck;
#[derive(Component)]
pub struct Head;
#[derive(Component)]
pub struct HeadEnd;
#[derive(Component)]
pub struct UpperArmLeft;
#[derive(Component)]
pub struct ForearmLeft;
#[derive(Component)]
pub struct HandLeft;
#[derive(Component)]
pub struct UpperArmRight;
#[derive(Component)]
pub struct ForearmRight;
#[derive(Component)]
pub struct HandRight;
#[derive(Component)]
pub struct ThighLeft;
#[derive(Component)]
pub struct ShinLeft;
#[derive(Component)]
pub struct FootLeft;
#[derive(Component)]
pub struct ThighRight;
#[derive(Component)]
pub struct ShinRight;
#[derive(Component)]
pub struct FootRight;

pub const HEIGHT: f32 = 1.7;
pub const SHIN_LENGTH: f32 = 3.5 / 15. * HEIGHT;
pub const THIGH_LENGTH: f32 = 3.5 / 15. * HEIGHT;
pub const SHOULDER_LENGTH: f32 = 2.0 / 15. * HEIGHT;
pub const FOREARM_LENGTH: f32 = 2.5 / 15. * HEIGHT;
pub const UPPER_ARM_LENGTH: f32 = 2.5 / 15. * HEIGHT;
pub const PELVIS_LENGTH: f32 = 1.5 / 15. * HEIGHT;

#[derive(Component)]
pub struct PlayerBody<T>(pub Body, PhantomData<T>);
pub struct Body(HashMap<TypeId, Transform>);

impl Default for Body {
    fn default() -> Self {
        let mut body = Self(HashMap::new());
        body.0.insert(
            Hip.type_id(),
            Transform::from_xyz(0.0, 7. / 15. * HEIGHT, 0.0),
        );
        body.0.insert(
            Spine.type_id(),
            Transform::from_xyz(0.0, 2. / 15. * HEIGHT, 0.0),
        );
        body.0.insert(
            Chest.type_id(),
            Transform::from_xyz(0.0, 2. / 15. * HEIGHT, 0.0),
        );
        body.0.insert(
            Neck.type_id(),
            Transform::from_xyz(0.0, 1. / 15. * HEIGHT, 0.0),
        );
        body.0.insert(
            Head.type_id(),
            Transform::from_xyz(0.0, 3. / 15. * HEIGHT, 0.0),
        );
        body.0.insert(
            UpperArmLeft.type_id(),
            Transform::from_xyz(-SHOULDER_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            ForearmLeft.type_id(),
            Transform::from_xyz(-UPPER_ARM_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            HandLeft.type_id(),
            Transform::from_xyz(-FOREARM_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            UpperArmRight.type_id(),
            Transform::from_xyz(SHOULDER_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            ForearmRight.type_id(),
            Transform::from_xyz(UPPER_ARM_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            HandRight.type_id(),
            Transform::from_xyz(FOREARM_LENGTH, 0.0, 0.0),
        );
        body.0.insert(
            ThighLeft.type_id(),
            Transform::from_xyz(-PELVIS_LENGTH, 0.0, 0.0),
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
            Transform::from_xyz(PELVIS_LENGTH, 0.0, 0.0),
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
