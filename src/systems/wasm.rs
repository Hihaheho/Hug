use bevy::prelude::*;

const WIDTH: f32 = 600.0;

pub fn resize(mut windows: ResMut<Windows>) {
    let js_window = web_sys::window().unwrap();
    let window = windows.get_primary_mut().unwrap();
    let width = js_window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height = js_window.inner_height().unwrap().as_f64().unwrap() as f32;
    if window.requested_width() != width || window.requested_height() != height {
        window.set_resolution(width, height);
    }
}

#[derive(PartialEq)]
pub enum IphoneMessage {
    NotDeleted,
    Deleted,
}

pub fn remove_message(time: Res<Time>, mut message: ResMut<IphoneMessage>) {
    if time.seconds_since_startup() > 0.3 && *message == IphoneMessage::NotDeleted {
        *message = IphoneMessage::Deleted;
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector("#iphone")
            .unwrap()
            .and_then(|result| Some(result.remove()));
    }
}
