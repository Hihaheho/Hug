use bevy::{
    ecs::component::Component,
    prelude::{shape as bshape, *},
};
use bevy_rapier3d::{
    na::{OPoint, Vector3},
    prelude::*,
};

use crate::components::{
    body::{part::*, *},
    control::HandControl,
    physics::{CollisionTag, Joint, JointMotorParams},
    player::{Player, Player1, Player2},
};

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_player::<Player1>(
        &mut commands,
        &mut meshes,
        &mut materials,
        0.2,
        Color::rgb(0.6, 0.4, 0.1),
    );
    create_player::<Player2>(
        &mut commands,
        &mut meshes,
        &mut materials,
        -0.2,
        Color::rgb(0.2, 0.2, 0.7),
    );
}

fn create_player<T: Player + Copy>(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    z: f32,
    color: Color,
) {
    let mut body = Body::default();
    body.get_mut::<Hip>().translation.z = z;
    body.get_mut::<Hip>().translation.y += 0.1;
    let propagated = body.propagated();

    let body = PlayerBody::<T>::new(body.clone(), propagated.clone());

    commands.insert_resource(HandControl::<T>::default());

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
            position: vector!(0.0, -0.5, 0.0).into(),
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

    let m = materials.add(color.into());
    insert_mesh::<T, Spine>(commands, meshes, m.clone(), &body, spine, torso_mesh::<T>);
    insert_mesh::<T, Chest>(commands, meshes, m.clone(), &body, chest, torso_mesh::<T>);
    insert_mesh::<T, Neck>(commands, meshes, m.clone(), &body, neck, neck_mesh::<T>);
    insert_mesh::<T, Head>(commands, meshes, m.clone(), &body, head, head_mesh::<T>);
    insert_mesh::<T, ForearmLeft>(
        commands,
        meshes,
        m.clone(),
        &body,
        forearm_left,
        arm_mesh::<T>,
    );
    insert_mesh::<T, ForearmRight>(
        commands,
        meshes,
        m.clone(),
        &body,
        forearm_right,
        arm_mesh::<T>,
    );
    insert_mesh::<T, HandLeft>(commands, meshes, m.clone(), &body, hand_left, arm_mesh::<T>);
    insert_mesh::<T, HandRight>(
        commands,
        meshes,
        m.clone(),
        &body,
        hand_right,
        arm_mesh::<T>,
    );
    insert_mesh::<T, ShinLeft>(commands, meshes, m.clone(), &body, shin_left, leg_mesh::<T>);
    insert_mesh::<T, ShinRight>(
        commands,
        meshes,
        m.clone(),
        &body,
        shin_right,
        leg_mesh::<T>,
    );
    insert_mesh::<T, FootLeft>(commands, meshes, m.clone(), &body, foot_left, leg_mesh::<T>);
    insert_mesh::<T, FootRight>(
        commands,
        meshes,
        m.clone(),
        &body,
        foot_right,
        leg_mesh::<T>,
    );

    // PlayerBody
    commands.insert_resource(body);
}

fn insert_mesh<T: Player, P: BodyPart>(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    body: &PlayerBody<T>,
    part: Entity,
    builder: fn(&Transform) -> Mesh,
) {
    let transform = body.relative.get::<P>();
    commands.entity(part).insert_bundle(PbrBundle {
        mesh: meshes.add(builder(transform)),
        material,
        ..Default::default()
    });
}

fn to_rapier_vec(vec: Vec3) -> Vector3<f32> {
    vector!(vec.x, vec.y, vec.z)
}

fn to_rapier_point(vec: Vec3) -> Point<f32> {
    point!(vec.x, vec.y, vec.z)
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
            position: to_rapier_vec(body.absolute.get::<C>().translation).into(),
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
        .local_anchor2(to_rapier_point(-child_translation))
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

fn torso_mesh<T: Player>(transform: &Transform) -> Mesh {
    let vec = transform.translation;
    Mesh::from(bshape::Box::new(TORSO_WIDTH, vec.y, TORSO_THICKNESS))
}

fn neck_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        -vec / 2.0,
        ColliderShape::cuboid(NECK_RADIUS, vec.y / 2.0, NECK_RADIUS),
    )
}

fn neck_mesh<T: Player>(transform: &Transform) -> Mesh {
    let vec = transform.translation;
    Mesh::from(bshape::Box::new(
        NECK_RADIUS * 2.0,
        vec.y,
        NECK_RADIUS * 2.0,
    ))
}

fn head_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        -vec / 2.0,
        ColliderShape::cuboid(HEAD_WIDTH / 2.0, vec.y / 2.0, HEAD_THICKNESS / 2.0),
    )
}

fn head_mesh<T: Player>(transform: &Transform) -> Mesh {
    let vec = transform.translation;
    Mesh::from(bshape::Box::new(HEAD_WIDTH, vec.y, HEAD_THICKNESS))
}

fn arm_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        (-vec / 2.0).into(),
        ColliderShape::cuboid(vec.x.abs() / 2.0, ARM_RADIUS, ARM_RADIUS),
    )
}

fn arm_mesh<T: Player>(transform: &Transform) -> Mesh {
    let vec = transform.translation;
    Mesh::from(bshape::Box::new(
        vec.x.abs(),
        ARM_RADIUS * 2.0,
        ARM_RADIUS * 2.0,
    ))
}

fn leg_collider<T: Player>(transform: &Transform) -> ColliderBundle {
    let vec = transform.translation;
    collider_bundle::<T>(
        (-vec / 2.0).into(),
        ColliderShape::cuboid(LEG_RADIUS, vec.y.abs() / 2.0, LEG_RADIUS),
    )
}

fn leg_mesh<T: Player>(transform: &Transform) -> Mesh {
    let vec = transform.translation;
    Mesh::from(bshape::Box::new(
        LEG_RADIUS * 2.0,
        vec.y.abs(),
        LEG_RADIUS * 2.0,
    ))
}

fn collider_bundle<T: Player>(position: Vec3, shape: ColliderShape) -> ColliderBundle {
    let collision_tag = T::get_collision_tag();
    ColliderBundle {
        position: to_rapier_vec(position).into(),
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
