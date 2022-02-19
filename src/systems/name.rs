use bevy::{
    prelude::*,
    render::camera::{Camera, PerspectiveProjection},
};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::components::{
    body::BodyPart,
    networking::PlayerName,
    player::{NameText, Player},
};

pub fn insert_name<P: Player, const LEFT: bool>(
    mut commands: Commands,
    name: Res<PlayerName<P>>,
    asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                name.0.clone(),
                TextStyle {
                    font: asset_server.load("DotGothic16-Regular.ttf"),
                    font_size: 70.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(NameText)
        .insert(P::default());
}

pub fn update_name<P: Player>(
    name: Res<PlayerName<P>>,
    mut query: Query<&mut Text, (With<P>, With<NameText>)>,
) {
    if name.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            text.sections[0].value = name.0.clone();
        }
    }
}

pub fn update_name_position<P: Player, Part: BodyPart, const LEFT: bool>(
    mut name: Query<&mut Style, (With<P>, With<NameText>)>,
    head: Query<&RigidBodyPositionComponent, (With<P>, With<Part>)>,
    camera: Query<(&Camera, &GlobalTransform), With<PerspectiveProjection>>,
    windows: Res<Windows>,
) {
    if let Ok((camera, transform)) = camera.single() {
        if let Ok(mut name) = name.single_mut() {
            if let Ok(head) = head.single() {
                let vec = Vec3::new(
                    head.0.position.translation.x,
                    head.0.position.translation.y,
                    head.0.position.translation.z,
                );
                if let Some(vec) = camera.world_to_screen(&windows, &transform, vec) {
                    name.position.bottom = Val::Px(vec.y);
                    if LEFT {
                        name.position.left = Val::Px(vec.x);
                    } else {
                        let window = windows.get_primary().unwrap();
                        name.position.left = Val::Px(vec.x - window.width() * 0.3);
                    }
                }
            }
        }
    }
}
