mod components;
mod systems;

use std::f32::consts::PI;

use bevy::{
    ecs::component::Component,
    prelude::{shape as bevy_shape, *},
};
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
    active_ragdoll::{hand_baloon_system, head_baloon_system, hip_baloon_system},
    control::{keyboard_input, move_system, touch_input},
    setup_player::setup_player,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum HugSystems {
    InputSystem,
    MoveSystem,
    ProgagateTransformSystem,
}

#[bevy_main]
fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::build();

    let mut win = WindowDescriptor::default();
    // win.scale_factor_override = Some(0.25);

    app.insert_resource(win)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(setup_player.system())
        .add_system_set(
            SystemSet::new()
                .label(HugSystems::InputSystem)
                .with_system(keyboard_input.system())
                .with_system(touch_input.system()),
        )
        .add_system_set(
            SystemSet::new()
                .label(HugSystems::MoveSystem)
                .after(HugSystems::InputSystem)
                .with_system(move_system::<Player1>.system())
                .with_system(move_system::<Player2>.system()),
        )
        .add_system_set(
            SystemSet::new()
                .before(PhysicsSystems::StepWorld)
                .with_system(head_baloon_system::<Player1>.system())
                .with_system(hand_baloon_system::<Player1>.system())
                .with_system(hip_baloon_system::<Player1>.system())
                .with_system(head_baloon_system::<Player2>.system())
                .with_system(hand_baloon_system::<Player2>.system())
                .with_system(hip_baloon_system::<Player2>.system())
                .with_system(angular_spring_system::<Player1, UpperArmLeft, ForearmLeft>.system())
                .with_system(angular_spring_system::<Player1, ForearmLeft, HandLeft>.system())
                .with_system(angular_spring_system::<Player1, UpperArmRight, ForearmRight>.system())
                .with_system(angular_spring_system::<Player1, ForearmRight, HandRight>.system())
                .with_system(angular_spring_system::<Player1, ThighLeft, ShinLeft>.system())
                .with_system(angular_spring_system::<Player1, ShinLeft, FootLeft>.system())
                .with_system(angular_spring_system::<Player1, ThighRight, ShinRight>.system())
                .with_system(angular_spring_system::<Player1, ShinRight, FootRight>.system()),
        );

    // bevy_mod_debugdump::print_schedule(&mut app);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin)
        .insert_resource(systems::wasm::Message::NotDeleted)
        .add_system(systems::wasm::resize.system())
        .add_system(systems::wasm::remove_message.system());

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut integration: ResMut<IntegrationParameters>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 7.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.3).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: vector!(0.0, -0.1, 0.0).into(),
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(10.0, 0.1, 10.0).into(),
            ..Default::default()
        });

    // wall left
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.1, -4.0))
            * Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.1, 0.2, 0.3).into()),
        ..Default::default()
    });

    // wall right
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 0.1, 0.0))
            * Transform::from_rotation(Quat::from_rotation_z(PI / 2.0)),
        mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.1, 0.2, 0.3).into()),
        ..Default::default()
    });

    // // light
    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 800.0,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(2.0, 3.0, 2.0),
    //     ..Default::default()
    // });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(2.0, 3.0, 2.0),
        ..Default::default()
    });
    // // light
    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 800.0,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(-2.0, 3.0, 2.0),
    //     ..Default::default()
    // });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(-2.0, 3.0, 2.0),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-1.5, 1.5, 2.0)
            .looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn angular_spring_system<T: Player, Parent: Component, Child: Component>(
    body: Res<PlayerBody<T>>,
    joint: Query<(&JointHandleComponent, &JointMotorParams), (With<Joint<Parent, Child>>, With<T>)>,
    mut joints: ResMut<ImpulseJointSet>,
) {
    if let Ok((joint_handle, JointMotorParams { stiffness, damping })) = joint.single() {
        let joint = joints.get_mut(joint_handle.handle()).unwrap();
        let transform = body.relative.get::<Parent>();
        let (x, y, z, w) = (
            transform.rotation.x,
            transform.rotation.y,
            transform.rotation.z,
            transform.rotation.w,
        );
        let (x2, y2, z2) = (x.powi(2), y.powi(2), z.powi(2));
        let yaw = (2.0 * (y * w - z * x)).atan2(1.0 - 2.0 * (y2 + x2));
        let pitch = -(2.0 * (z * y + x * w)).asin();
        let roll = (2.0 * (z * w - y * x)).atan2(1.0 - 2.0 * (z2 + x2));

        joint.data = joint
            .data
            .motor_position(JointAxis::AngX, pitch, *stiffness, *damping)
            .motor_position(JointAxis::AngY, yaw, *stiffness, *damping)
            .motor_position(JointAxis::AngZ, roll, *stiffness, *damping);
    }
}
