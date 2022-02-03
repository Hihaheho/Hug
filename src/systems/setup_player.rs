use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{
    body::{part::*, *},
    physics::{CollisionTag, Joint, JointMotorParams},
    player::{Player, Player1, Player2},
};

pub fn setup_player(mut commands: Commands) {
    create_player::<Player1>(&mut commands, 1.0);
    create_player::<Player2>(&mut commands, -1.0);
}

fn create_player<T: Player + Copy>(commands: &mut Commands, z: f32) {
    let mut body = Body::default();
    body.get_mut::<Hip>().translation.z = z;
    body.get_mut::<Hip>().translation.y += 0.1;
    let propagated = body.propagated();

    let body = PlayerBody::<T>::new(body.clone(), propagated.clone());

    let hip = spawn_body_part::<T, Hip>(commands, &body, small_collider::<T>);
    let spine = spawn_body_part::<T, Spine>(commands, &body, torso_collider::<T>);
    let chest = spawn_body_part::<T, Chest>(commands, &body, torso_collider::<T>);
    let neck = spawn_body_part::<T, Neck>(commands, &body, neck_collider::<T>);
    let head = spawn_body_part::<T, Head>(commands, &body, head_collider::<T>);
    let upper_arm_left = spawn_body_part::<T, UpperArmLeft>(commands, &body, small_collider::<T>);
    let forearm_left = spawn_body_part::<T, ForearmLeft>(commands, &body, arm_collider::<T>);
    let hand_left = spawn_body_part::<T, HandLeft>(commands, &body, arm_collider::<T>);
    let upper_arm_right = spawn_body_part::<T, UpperArmRight>(commands, &body, small_collider::<T>);
    let forearm_right = spawn_body_part::<T, ForearmRight>(commands, &body, arm_collider::<T>);
    let hand_right = spawn_body_part::<T, HandRight>(commands, &body, arm_collider::<T>);
    let thigh_left = spawn_body_part::<T, ThighLeft>(commands, &body, small_collider::<T>);
    let shin_left = spawn_body_part::<T, ShinLeft>(commands, &body, leg_collider::<T>);
    let foot_left = spawn_body_part::<T, FootLeft>(commands, &body, leg_collider::<T>);
    let thigh_right = spawn_body_part::<T, ThighRight>(commands, &body, small_collider::<T>);
    let shin_right = spawn_body_part::<T, ShinRight>(commands, &body, leg_collider::<T>);
    let foot_right = spawn_body_part::<T, FootRight>(commands, &body, leg_collider::<T>);

    joint::<Hip, Spine, _>(commands, &body, hip, spine);
    joint::<Spine, Chest, _>(commands, &body, spine, chest);
    joint::<Chest, Neck, _>(commands, &body, chest, neck);
    joint::<Neck, Head, _>(commands, &body, neck, head);
    joint::<Hip, ThighLeft, _>(commands, &body, hip, thigh_left);
    joint::<ThighLeft, ShinLeft, _>(commands, &body, thigh_left, shin_left);
    joint::<ShinLeft, FootLeft, _>(commands, &body, shin_left, foot_left);
    joint::<Hip, ThighRight, _>(commands, &body, hip, thigh_right);
    joint::<ThighRight, ShinRight, _>(commands, &body, thigh_right, shin_right);
    joint::<ShinRight, FootRight, _>(commands, &body, shin_right, foot_right);
    joint::<Chest, UpperArmLeft, _>(commands, &body, chest, upper_arm_left);
    joint::<UpperArmLeft, ForearmLeft, _>(commands, &body, upper_arm_left, forearm_left);
    joint::<ForearmLeft, HandLeft, _>(commands, &body, forearm_left, hand_left);
    joint::<Chest, UpperArmRight, _>(commands, &body, chest, upper_arm_right);
    joint::<UpperArmRight, ForearmRight, _>(commands, &body, upper_arm_right, forearm_right);
    joint::<ForearmRight, HandRight, _>(commands, &body, forearm_right, hand_right);

    // Lock hip's rotation by connecting to the ground (locking with mass properties causes panic).
    let ground = commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, -0.5, 0.0).into(),
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.1, 0.1, 0.1).into(),
            ..Default::default()
        })
        .id();
    let joint = JointData::new(JointAxesMask::ANG_X | JointAxesMask::ANG_Y | JointAxesMask::ANG_Z);
    commands
        .spawn()
        .insert(JointBuilderComponent::new(joint, ground, hip));

    // PlayerBody
    commands.insert_resource(body);
}

fn spawn_body_part<T: Player, C: BodyPart>(
    commands: &mut Commands,
    body: &PlayerBody<T>,
    collider_builder: fn(&Transform) -> ColliderBundle,
) -> Entity {
    commands
        .spawn()
        .insert(T::default())
        .insert(C::default())
        .insert(body.absolute.get::<C>().clone())
        .insert_bundle(RigidBodyBundle {
            position: body.absolute.get::<C>().translation.into(),
            ..Default::default()
        })
        .insert_bundle(collider_builder(body.relative.get::<C>()))
        .insert(ColliderDebugRender::with_id(1))
        .insert(ColliderPositionSync::Discrete)
        .id()
}

fn joint<Parent: Component, Child: Component, T: Player>(
    commands: &mut Commands,
    body: &PlayerBody<T>,
    parent: Entity,
    child: Entity,
) {
    let stiffness = 1.5;
    let damping = 1.5;
    let child_translation = body.relative.get::<Child>().translation;
    let joint = SphericalJoint::new()
        .local_anchor2((-child_translation).into())
        .motor_position(JointAxis::AngX, 0.0, stiffness, damping)
        .motor_position(JointAxis::AngY, 0.0, stiffness, damping)
        .motor_position(JointAxis::AngZ, 0.0, stiffness, damping)
        .motor_model(JointAxis::AngX, MotorModel::VelocityBased)
        .motor_model(JointAxis::AngY, MotorModel::VelocityBased)
        .motor_model(JointAxis::AngZ, MotorModel::VelocityBased);
    let joint = JointBuilderComponent::new(joint, parent, child);
    commands
        .spawn()
        .insert(T::default())
        .insert(Joint::<Parent, Child>::default())
        .insert(JointMotorParams { stiffness, damping })
        .insert(joint);
}

fn small_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    ColliderBundle {
        mass_properties: ColliderMassProps::Density(50.0).into(),
        ..collider_bundle::<T>(Vec3::ZERO, ColliderShape::cuboid(0.05, 0.05, 0.05))
    }
}

fn torso_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        (-vec / 2.0).into(),
        ColliderShape::cuboid(TORSO_WIDTH / 2.0, vec.y / 2.0, TORSO_THICKNESS / 2.0),
    )
}

fn neck_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        -vec / 2.0,
        ColliderShape::cuboid(NECK_RADIUS, vec.y / 2.0, NECK_RADIUS),
    )
}

fn head_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        -vec / 2.0,
        ColliderShape::cuboid(HEAD_RADIUS, vec.y / 2.0, HEAD_RADIUS),
    )
}

fn arm_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        (-vec / 2.0).into(),
        ColliderShape::cuboid(vec.x.abs() / 2.0, ARM_RADIUS, ARM_RADIUS),
    )
}

fn leg_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        (-vec / 2.0).into(),
        ColliderShape::cuboid(LEG_RADIUS, vec.y.abs() / 2.0, LEG_RADIUS),
    )
}

fn collider_bundle<T: Player>(position: Vec3, shape: ColliderShape) -> ColliderBundle {
    let collision_tag = T::get_collision_tag();
    ColliderBundle {
        position: position.into(),
        shape: shape.into(),
        flags: ColliderFlags {
            collision_groups: InteractionGroups::new(
                collision_tag.into(),
                (CollisionTag::ALL ^ collision_tag).into(),
            ),
            ..Default::default()
        }
        .into(),
        ..Default::default()
    }
}
