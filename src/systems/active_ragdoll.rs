use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{
    body::{part::*, BodyPart, PlayerBody},
    player::Player,
    ragdoll::BallonForceCoef,
};

pub fn baloon_system<T: Player, P: BodyPart>(
    body: Res<PlayerBody<T>>,
    mut head: Query<
        (
            &mut RigidBodyForcesComponent,
            &RigidBodyPositionComponent,
            &BallonForceCoef,
        ),
        (With<T>, With<P>),
    >,
) {
    for (mut forces, pos, coef) in head.iter_mut() {
        let t = pos.position.translation;
        let diff = body.absolute.get::<P>().translation - Vec3::new(t.x, t.y, t.z);
        let force = coef.0 * diff;
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
