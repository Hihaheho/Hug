use bevy::prelude::*;

const WIDTH: f32 = 600.0;

pub fn resize(mut windows: ResMut<Windows>) {
    let js_window = web_sys::window().unwrap();
    let window = windows.get_primary_mut().unwrap();
    let width = js_window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height = js_window.inner_height().unwrap().as_f64().unwrap() as f32;
    window.set_resolution(width * (WIDTH / width), height * (WIDTH / width));
}

#[derive(PartialEq)]
pub enum Message {
    NotDeleted,
    Deleted,
}

pub fn remove_message(time: Res<Time>, mut message: ResMut<Message>) {
    if time.seconds_since_startup() > 1.5 && *message == Message::NotDeleted {
        *message = Message::Deleted;
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector("#iphone")
            .unwrap()
            .and_then(|result| Some(result.remove()));
    }
}
