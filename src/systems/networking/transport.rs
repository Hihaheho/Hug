use bevy::prelude::*;
use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::components::networking::{Receiver, Sender};

use super::BUFFER;

pub fn sender(mut sender: ResMut<Sender>) {
    let document = web_sys::window().unwrap().document().unwrap();
    let push = unsafe { js_sys::Reflect::get(&document, &"push".into()).unwrap() };
    if push.is_function() {
        let push = Function::from(push);
        if sender.0.len() != 0 {
            for message in sender
                .0
                .split_off(0)
                .into_iter()
                .map(|message| serde_json::to_string(&message).unwrap())
            {
                let _ = push.call1(&JsValue::NULL, &message.into());
            }
        }
    }
}

pub fn receiver(mut receiver: ResMut<Receiver>) {
    let buffer = BUFFER.lock().split_off(0);
    receiver.0 = buffer
        .into_iter()
        .map(|message| serde_json::from_str(&message).unwrap())
        .collect();
}
