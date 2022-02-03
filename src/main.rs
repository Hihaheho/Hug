mod components;
mod systems;

use bevy::prelude::{shape as bevy_shape, *};
use bevy_rapier3d::{
    na::{Normed, Vector3},
    physics::{JointHandleComponent, PhysicsSystems},
    prelude::*,
};
use components::{
    body::{part::*, PlayerBody},
    physics::{CollisionTag, Joint, JointMotorParams},
    player::{Player, Player1, Player2},
};
use systems::{
    active_ragdoll::{hand_baloon_system, head_baloon_system},
    setup_player::setup_player,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum HugSystems {}

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_player)
        .add_system_set(
            SystemSet::new()
                .before(PhysicsSystems::StepWorld)
                .with_system(head_baloon_system::<Player1>)
                .with_system(hand_baloon_system::<Player1>)
                .with_system(head_baloon_system::<Player2>)
                .with_system(hand_baloon_system::<Player2>)
                .with_system(angular_spring_system::<Player1, UpperArmLeft, ForearmLeft>)
                .with_system(angular_spring_system::<Player1, ForearmLeft, HandLeft>)
                .with_system(angular_spring_system::<Player1, UpperArmRight, ForearmRight>)
                .with_system(angular_spring_system::<Player1, ForearmRight, HandRight>)
                .with_system(angular_spring_system::<Player1, ThighLeft, ShinLeft>)
                .with_system(angular_spring_system::<Player1, ShinLeft, FootLeft>)
                .with_system(angular_spring_system::<Player1, ThighRight, ShinRight>)
                .with_system(angular_spring_system::<Player1, ShinRight, FootRight>),
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
            material: materials.add(Color::rgb(0.1, 0.2, 0.3).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, -0.1, 0.0).into(),
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
}

fn angular_spring_system<T: Player, Parent: Component, Child: Component>(
    body: Res<PlayerBody<T>>,
    joint: Query<(&JointHandleComponent, &JointMotorParams), (With<Joint<Parent, Child>>, With<T>)>,
    mut joints: ResMut<ImpulseJointSet>,
) {
    if let Ok((joint_handle, JointMotorParams { stiffness, damping })) = joint.get_single() {
        let joint = joints.get_mut(joint_handle.handle()).unwrap();
        let transform = body.relative.get::<Parent>();
        let angles = dbg!(transform.rotation.to_euler(EulerRot::XYZ));
        joint.data = joint
            .data
            .motor_position(JointAxis::AngX, angles.0, *stiffness, *damping)
            .motor_position(JointAxis::AngY, angles.1, *stiffness, *damping)
            .motor_position(JointAxis::AngZ, angles.2, *stiffness, *damping);
    }
}
