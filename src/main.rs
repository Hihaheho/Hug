mod components;

use bevy::prelude::{shape as bevy_shape, *};
use bevy_rapier3d::{
    physics::{JointHandleComponent, PhysicsSystems},
    prelude::*,
};
use components::{
    body::*,
    physics::{CollisionTag, Joint},
    player::{Player, Player1, Player2},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum MainSystems {
    /// After global transform update, sync the global transform to rigid bodies
    SyncRigidBody,
    /// After physics update, sync the transform from rigid bodies
    SyncTransform,
    /// Move the body by changing transform
    BodyMovement,
}

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .before(PhysicsSystems::StepWorld)
                .with_system(angular_spring_system::<Player1, Hip, Spine>)
                .with_system(angular_spring_system::<Player1, Spine, Chest>)
                .with_system(angular_spring_system::<Player1, Chest, Neck>)
                .with_system(angular_spring_system::<Player1, Neck, Head>),
        )
        .add_system_set(SystemSet::new().after(PhysicsSystems::StepWorld));

    // bevy_mod_debugdump::print_schedule(&mut app);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(10.0, 0.1, 10.0).into(),
            ..Default::default()
        });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-5.0, 2.0, 5.0)
            .looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
        ..Default::default()
    });

    create_player(&mut commands, Player1, 1.0);
    create_player(&mut commands, Player2, -1.0);
}

fn create_player<T: Player + Copy>(commands: &mut Commands, tag: T, z: f32) {
    let mut body = Body::default();
    body.get_mut::<Hip>().translation.z = z;
    body.get_mut::<Hip>().translation.y += 0.5;
    let propagated = body.propagated();

    let hip = commands
        .spawn()
        .insert(tag)
        .insert(Hip)
        .insert(propagated.get::<Hip>().clone())
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..rigid_body_bundle(propagated.get::<Hip>())
        })
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let spine = commands
        .spawn()
        .insert(tag)
        .insert(Spine)
        .insert(propagated.get::<Spine>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<Spine>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let chest = commands
        .spawn()
        .insert(tag)
        .insert(Chest)
        .insert(propagated.get::<Chest>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<Chest>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let neck = commands
        .spawn()
        .insert(tag)
        .insert(Neck)
        .insert(propagated.get::<Neck>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<Neck>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let head = commands
        .spawn()
        .insert(tag)
        .insert(Head)
        .insert(propagated.get::<Head>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<Head>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let thigh_left = commands
        .spawn()
        .insert(tag)
        .insert(ThighLeft)
        .insert(propagated.get::<ThighLeft>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<ThighLeft>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let shin_left = commands
        .spawn()
        .insert(tag)
        .insert(ShinLeft)
        .insert(propagated.get::<ShinLeft>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<ShinLeft>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let foot_left = commands
        .spawn()
        .insert(tag)
        .insert(FootLeft)
        .insert(propagated.get::<FootLeft>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<FootLeft>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let thigh_right = commands
        .spawn()
        .insert(tag)
        .insert(ThighRight)
        .insert(propagated.get::<ThighRight>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<ThighRight>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let shin_right = commands
        .spawn()
        .insert(tag)
        .insert(ShinRight)
        .insert(propagated.get::<ShinRight>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<ShinRight>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let foot_right = commands
        .spawn()
        .insert(tag)
        .insert(FootRight)
        .insert(propagated.get::<FootRight>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<FootRight>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let upper_arm_left = commands
        .spawn()
        .insert(tag)
        .insert(UpperArmLeft)
        .insert(propagated.get::<UpperArmLeft>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<UpperArmLeft>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let forearm_left = commands
        .spawn()
        .insert(tag)
        .insert(ForearmLeft)
        .insert(propagated.get::<ForearmLeft>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<ForearmLeft>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let hand_left = commands
        .spawn()
        .insert(tag)
        .insert(HandLeft)
        .insert(propagated.get::<HandLeft>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<HandLeft>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let upper_arm_right = commands
        .spawn()
        .insert(tag)
        .insert(UpperArmRight)
        .insert(propagated.get::<UpperArmRight>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<UpperArmRight>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let forearm_right = commands
        .spawn()
        .insert(tag)
        .insert(ForearmRight)
        .insert(propagated.get::<ForearmRight>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<ForearmRight>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    let hand_right = commands
        .spawn()
        .insert(tag)
        .insert(HandRight)
        .insert(propagated.get::<HandRight>().clone())
        .insert_bundle(rigid_body_bundle(propagated.get::<HandRight>()))
        .insert_bundle(collider_bundle::<T>(ColliderShape::ball(0.1)))
        .insert(ColliderDebugRender::with_id(1))
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    insert_joint::<Hip, Spine>(commands, &body, hip, spine);
    insert_joint::<Spine, Chest>(commands, &body, spine, chest);
    insert_joint::<Chest, Neck>(commands, &body, chest, neck);
    insert_joint::<Neck, Head>(commands, &body, neck, head);
    insert_joint::<Hip, ThighLeft>(commands, &body, hip, thigh_left);
    insert_joint::<ThighLeft, ShinLeft>(commands, &body, thigh_left, shin_left);
    insert_joint::<ShinLeft, FootLeft>(commands, &body, shin_left, foot_left);
    insert_joint::<Hip, ThighRight>(commands, &body, hip, thigh_right);
    insert_joint::<ThighRight, ShinRight>(commands, &body, thigh_right, shin_right);
    insert_joint::<ShinRight, FootRight>(commands, &body, shin_right, foot_right);
    insert_joint::<Chest, UpperArmLeft>(commands, &body, chest, upper_arm_left);
    insert_joint::<UpperArmLeft, ForearmLeft>(commands, &body, upper_arm_left, forearm_left);
    insert_joint::<ForearmLeft, HandLeft>(commands, &body, forearm_left, hand_left);
    insert_joint::<Chest, UpperArmRight>(commands, &body, chest, upper_arm_right);
    insert_joint::<UpperArmRight, ForearmRight>(commands, &body, upper_arm_right, forearm_right);
    insert_joint::<ForearmRight, HandRight>(commands, &body, forearm_right, hand_right);

    let hip_translation = propagated.get::<Hip>().translation;
    let ground = commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec3::new(hip_translation.x, 0.0, hip_translation.z).into(),
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(0.1).into(),
            ..Default::default()
        })
        .id();

    // let joint = SphericalJoint::new()
    //     .local_anchor2(Vec3::new(0.0, -hip_translation.y, 0.0).into())
    //     .motor_position(JointAxis::AngX, 0.0, 10., 0.0)
    //     .motor_position(JointAxis::AngY, 0.0, 10., 0.0)
    //     .motor_position(JointAxis::AngZ, 0.0, 10., 0.0)
    //     .motor_model(JointAxis::AngX, MotorModel::AccelerationBased)
    //     .motor_model(JointAxis::AngY, MotorModel::AccelerationBased)
    //     .motor_model(JointAxis::AngZ, MotorModel::AccelerationBased);
    // let joint = JointBuilderComponent::new(joint, ground, hip);

    // commands.spawn().insert(joint);
}

fn insert_joint<Parent: Component, Child: Component>(
    commands: &mut Commands,
    body: &Body,
    parent: Entity,
    child: Entity,
) {
    let child_translation = body.get::<Child>().translation;
    let joint = SphericalJoint::new()
        .local_anchor2((-child_translation).into())
        .motor_position(JointAxis::AngX, 0.0, 10., 0.0)
        .motor_position(JointAxis::AngY, 0.0, 10., 0.0)
        .motor_position(JointAxis::AngZ, 0.0, 10., 0.0)
        .motor_model(JointAxis::AngX, MotorModel::AccelerationBased)
        .motor_model(JointAxis::AngY, MotorModel::AccelerationBased)
        .motor_model(JointAxis::AngZ, MotorModel::AccelerationBased);
    let joint = JointBuilderComponent::new(joint, parent, child);
    commands
        .spawn()
        .insert(Joint::<Parent, Child>::default())
        .insert(joint);
}

fn insert_fixed_joint<Parent: Component, Child: Component>(
    commands: &mut Commands,
    body: &Body,
    parent: Entity,
    child: Entity,
) {
    let joint = JointBuilderComponent::new(
        FixedJoint::new().local_anchor2((-body.get::<Child>().translation).into()),
        parent,
        child,
    );
    commands
        .spawn()
        .insert(Joint::<Parent, Child>::default())
        .insert(joint);
}

fn head_baloon_system<T: Player>(
    mut head: Query<&RigidBodyForcesComponent, (With<T>, With<Head>)>,
) {
}

fn angular_spring_system<T: Player, Parent: Component, Child: Component>(
    mut parent: Query<&Transform, (With<Parent>, With<T>)>,
    mut child: Query<&Transform, (With<Child>, With<T>)>,
    mut joint: Query<&JointHandleComponent, (With<Joint<Parent, Child>>, With<T>)>,
    mut joints: ResMut<ImpulseJointSet>,
) {
    if let Ok((parent, child, joint_handle)) = parent
        .get_single()
        .and_then(|parent| Ok((parent, child.get_single()?)))
        .and_then(|(parent, child)| Ok((parent, child, joint.get_single()?)))
    {
        let joint = joints.get_mut(joint_handle.handle()).unwrap();
    }
}

fn collider_flags(collision_tag: CollisionTag) -> ColliderFlags {
    ColliderFlags {
        collision_groups: InteractionGroups::new(
            collision_tag.into(),
            (CollisionTag::ALL ^ collision_tag).into(),
        ),
        ..Default::default()
    }
    .into()
}

fn rigid_body_bundle(transform: &Transform) -> RigidBodyBundle {
    RigidBodyBundle {
        position: transform.translation.into(),
        ..Default::default()
    }
}

fn collider_bundle<T: Player>(shape: ColliderShape) -> ColliderBundle {
    ColliderBundle {
        shape: shape.into(),
        flags: collider_flags(T::get_collision_tag()).into(),
        ..Default::default()
    }
}
