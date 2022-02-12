use bevy::prelude::*;

use crate::{
    components::{
        body::{part::*, PlayerBody},
        control::HandControl,
        player::{Player, Player1, Player2},
    },
    WIDTH,
};

pub fn touch_input(
    mut control: ResMut<HandControl<Player1>>,
    touches: Res<Touches>,
    desc: Res<WindowDescriptor>,
) {
    let scale = desc.scale_factor_override.unwrap() as f32;
    for touch in touches.iter() {
        let delta = touch.delta();
        let delta = Vec2::new(delta.x / (WIDTH / 5.0), delta.y / (WIDTH / 2.0));
        // web_sys::console::log_1(&format!("scale {}", scale).into());
        // web_sys::console::log_1(&format!("width {}", width).into());
        // web_sys::console::log_1(&format!("x {}", touch.start_position().x / 2.0).into());
        if touch.start_position().x < WIDTH / 2.0 {
            control.add_left(delta);
        } else {
            control.add_right(delta);
        }
    }
}

pub fn keyboard_input(
    time: Res<Time>,
    mut control: ResMut<HandControl<Player1>>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::A) {
        control.add_left(Vec2::new(-1.0, 0.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::S) {
        control.add_left(Vec2::new(1.0, 0.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::W) {
        control.add_left(Vec2::new(0.0, 1.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::R) {
        control.add_left(Vec2::new(0.0, -1.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::E) {
        control.add_right(Vec2::new(-1.0, 0.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::O) {
        control.add_right(Vec2::new(1.0, 0.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::Y) {
        control.add_right(Vec2::new(0.0, 1.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::I) {
        control.add_right(Vec2::new(0.0, -1.0) * time.delta_seconds());
    }
}

pub fn move_system(control: Res<HandControl<Player1>>, mut body: ResMut<PlayerBody<Player1>>) {
    if control.is_changed() {
        let (b, a, c) = control.left_sholder();
        body.relative.get_mut::<UpperArmLeft>().rotation = Quat::from_rotation_ypr(a, b, c);
        let (b, a, c) = control.right_sholder();
        body.relative.get_mut::<UpperArmRight>().rotation = Quat::from_rotation_ypr(a, b, c);
        let (b, a, c) = control.left_elbow();
        body.relative.get_mut::<ForearmLeft>().rotation = Quat::from_rotation_ypr(a, b, c);
        let (b, a, c) = control.right_elbow();
        body.relative.get_mut::<ForearmRight>().rotation = Quat::from_rotation_ypr(a, b, c);

        body.absolute = body.relative.propagated();
    }
}

pub fn move_system2(control: Res<HandControl<Player2>>, mut body: ResMut<PlayerBody<Player2>>) {
    if control.is_changed() {
        let (b, a, c) = control.left_sholder();
        body.relative.get_mut::<UpperArmLeft>().rotation = Quat::from_rotation_ypr(a, b, c);
        let (b, a, c) = control.right_sholder();
        body.relative.get_mut::<UpperArmRight>().rotation = Quat::from_rotation_ypr(a, b, c);
        let (b, a, c) = control.left_elbow();
        body.relative.get_mut::<ForearmLeft>().rotation = Quat::from_rotation_ypr(a, b, c);
        let (b, a, c) = control.right_elbow();
        body.relative.get_mut::<ForearmRight>().rotation = Quat::from_rotation_ypr(a, b, c);

        body.absolute = body.relative.propagated();
    }
}
