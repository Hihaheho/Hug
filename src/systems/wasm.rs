use bevy::prelude::*;

pub fn resize(mut windows: ResMut<Windows>) {
    let js_window = web_sys::window().unwrap();
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(
        js_window.inner_width().unwrap().as_f64().unwrap() as f32,
        js_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
}
