use bevy::prelude::*;
use bevy_rapier3d::{prelude::*};

use crate::components::{
    body::{part::*, PlayerBody},
    player::Player,
};

pub fn head_baloon_system<T: Player>(
    body: Res<PlayerBody<T>>,
    mut head: Query<
        (&mut RigidBodyForcesComponent, &RigidBodyPositionComponent),
        (With<T>, With<Head>),
    >,
) {
    for (mut forces, pos) in head.iter_mut() {
        let t = pos.position.translation;
        let diff = body.absolute.get::<Head>().translation - Vec3::new(t.x, t.y, t.z);
		let force = 3.0 * diff;
        forces.force += vector!(force.x, force.y, force.z);
    }
}

pub fn hip_baloon_system<T: Player>(
    body: Res<PlayerBody<T>>,
    mut head: Query<
        (&mut RigidBodyForcesComponent, &RigidBodyPositionComponent),
        (With<T>, With<Hip>),
    >,
) {
    for (mut forces, pos) in head.iter_mut() {
        let t = pos.position.translation;
        let diff = body.absolute.get::<Hip>().translation - Vec3::new(t.x, t.y, t.z);
		let force = 2.0 * diff;
        forces.force += vector!(force.x, force.y, force.z);
    }
}

pub fn hand_baloon_system<T: Player>(
    body: Res<PlayerBody<T>>,
    mut left: Query<
        (&mut RigidBodyForcesComponent, &RigidBodyPositionComponent),
        (With<T>, With<HandLeft>, Without<HandRight>),
    >,
    mut right: Query<
        (&mut RigidBodyForcesComponent, &RigidBodyPositionComponent),
        (With<T>, With<HandRight>, Without<HandLeft>),
    >,
) {
    for (mut forces, pos) in left.iter_mut() {
        let diff = body.absolute.get::<HandLeft>().translation.y - pos.position.translation.y;
        forces.force += vector!(0.0, diff * 1.0, 0.0);
    }
    for (mut forces, pos) in right.iter_mut() {
        let diff = body.absolute.get::<HandRight>().translation.y - pos.position.translation.y;
        forces.force += vector!(0.0, diff * 1.0, 0.0);
    }
}
