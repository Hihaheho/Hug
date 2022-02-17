use bevy::{ecs::component::Component, prelude::*};
use bevy_rapier3d::{physics::JointHandleComponent, prelude::*};

use crate::{
    components::{
        body::{part::*, PlayerBody},
        control::HandControl,
        physics::{Joint, JointMotorParams},
        player::{Player, Player1, Player2},
    },
    WIDTH,
};

pub fn touch_input(
    mut control: ResMut<HandControl<Player1>>,
    touches: Res<Touches>,
    desc: Res<WindowDescriptor>,
) {
    for touch in touches.iter() {
        let delta = touch.delta();
        let delta = Vec2::new(delta.x / (WIDTH / 5.0), delta.y / (WIDTH / 2.0));
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
    if input.pressed(KeyCode::D) {
        control.add_left(Vec2::new(1.0, 0.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::W) {
        control.add_left(Vec2::new(0.0, 1.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::S) {
        control.add_left(Vec2::new(0.0, -1.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::J) {
        control.add_right(Vec2::new(-1.0, 0.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::L) {
        control.add_right(Vec2::new(1.0, 0.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::I) {
        control.add_right(Vec2::new(0.0, 1.0) * time.delta_seconds());
    }
    if input.pressed(KeyCode::K) {
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

pub fn angular_spring_system<T: Player, Parent: Component, Child: Component>(
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
