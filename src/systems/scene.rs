use std::f32::consts::PI;

use bevy::prelude::{shape as bevy_shape, *};
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // asset_server: Res<AssetServer>,
    mut integration: ResMut<IntegrationParameters>,
) {
    integration.max_velocity_iterations = 5;
    integration.max_stabilization_iterations = 2;

    let wall_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.78, 0.73, 0.75),
        roughness: 0.3,
        metallic: 0.0,
        reflectance: 0.5,
        ..Default::default()
    });
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 7.0 })),
            material: wall_material.clone(),
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
        material: wall_material.clone(),
        ..Default::default()
    });

    // wall right
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 0.1, 0.0))
            * Transform::from_rotation(Quat::from_rotation_z(PI / 2.0)),
        mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 15.0 })),
        material: wall_material.clone(),
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
    // commands.spawn_bundle(LightBundle {
    //     transform: Transform::from_xyz(2.0, 3.0, 2.0),
    //     ..Default::default()
    // });
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
        transform: Transform::from_xyz(-3.5, 3.0, -7.0),
        light: Light {
            intensity: 300.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(1.0, 3.0, 3.0),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.4, 1.6, 2.6)
            .looking_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y),
        ..Default::default()
    });

    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
}
