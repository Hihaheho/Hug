use bevy::{
    prelude::*,
    render::camera::{Camera, PerspectiveProjection},
};
use bevy_rapier3d::prelude::{RigidBodyPosition, RigidBodyPositionComponent};

use crate::components::{
    body::{
        part::{HandRight, Head},
        BodyPart,
    },
    networking::PlayerName,
    player::{NameText, Player},
};

pub fn load_font(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    let font = fonts.add(
        Font::try_from_bytes(
            include_bytes!("../../assets/NotoSansJP-Black.otf")
                .into_iter()
                .cloned()
                .collect(),
        )
        .unwrap(),
    );
    commands.insert_resource(font);
}

pub fn insert_name<P: Player>(
    mut commands: Commands,
    name: Res<PlayerName<P>>,
    font: Res<Handle<Font>>,
    asset_server: ResMut<AssetServer>,
) {
    if font.is_added() {
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(5.0),
                        right: Val::Px(15.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    name.0.clone(),
                    TextStyle {
                        font: asset_server.load("NotoSansJP-Black.otf"),
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

pub fn update_name_position<P: Player, Part: BodyPart, const left: bool>(
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
                    0.0,
                );
                let vec = camera.world_to_screen(&windows, &transform, vec).unwrap();
                name.position.bottom = Val::Px(vec.y);
                if left {
                    name.position.left = Val::Px(vec.x - 10.0);
                } else {
                    name.position.right = Val::Px(vec.x - 10.0);
                }
            }
        }
    }
}
