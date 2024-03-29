mod adapters;
mod components;
mod plugins;
mod systems;

use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use js_sys::Function;
use plugins::body::BodyPlugin;
use wasm_bindgen::prelude::*;

use components::{
    state::AppState,
    ui::{Device, Messages},
};
#[cfg(target_arch = "wasm32")]
use plugins::{networking::NetworkPlugin, ui::UiPlugin};
use systems::{scene::setup, setup_player::setup_player};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum HugSystems {
    InputSystem,
    MoveSystem,
    ProgagateTransformSystem,
}

pub const WIDTH: f32 = 300.0;

#[bevy_main]
fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::build();

    let mut win = WindowDescriptor::default();
    #[cfg(target_arch = "wasm32")]
    {
        win.scale_factor_override = Some(1.0);
    }
    let mut device = Device::Desktop;
    let mut messages = Messages {
        finding: "Finding someone to hug.",
        room_created: "Room created share the url to your friends",
        ready: "Ready to hug",
        finding_room: "Joining the room",
        room_notfound: "Room key it used or closed yet.",
        copied: "Copied to clipboard",
        share: "The Hug game is amazing! Try it out!",
        share_match: "With {name}, we've hugged for {minute} minutes and {second} seconds.",
        room_link: "Hug with Me?",
        tags: "#metaverse #thehuggame",
    };
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let on_load =
            Function::from(unsafe { js_sys::Reflect::get(&document, &"on_load".into()).unwrap() });
        on_load.call0(&JsValue::NULL);

        let is_iphone = unsafe { js_sys::Reflect::get(&document, &"is_iphone".into()).unwrap() };
        if !is_iphone.is_falsy() {
            let js_window = web_sys::window().unwrap();
            let width = js_window.inner_width().unwrap().as_f64().unwrap() as f32;
            let height = js_window.inner_height().unwrap().as_f64().unwrap() as f32;
            let ratio = WIDTH / width;
            win.scale_factor_override = Some(ratio as f64);
        }
        let is_mobile = unsafe { js_sys::Reflect::get(&document, &"is_mobile".into()).unwrap() };
        if !is_mobile.is_falsy() {
            device = Device::Mobile;
        }
        let navigator = window.navigator();
        if let Some(language) = navigator.language() {
            if language.starts_with("ja") {
                messages = Messages {
                    finding: "ハグの相手を探しています",
                    room_created: "ルームを作成しました。リンクを友達に送ってください",
                    ready: "ハグ開始！",
                    finding_room: "ルームに参加中です",
                    room_notfound: "ルームに入れませんでした",
                    copied: "コピーしました！",
                    share: "ハグゲームは最高です！みんなでハグしよう！",
                    share_match:
                        "{name}さんと{minute}分{second}秒ハグしました",
                    room_link: "わたしとハグしませんか？",
                    tags: "#メタバース #ハグゲーム",
                };
            }
        }
    }

    app.insert_resource(win)
        .insert_resource(device)
        .insert_resource(messages)
        .add_plugins(DefaultPlugins)
        .add_plugin(BodyPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(setup_player.system())
        .add_state(AppState::Alone);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugin(WorldInspectorPlugin::new());

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(NetworkPlugin).add_plugin(UiPlugin);

    // bevy_mod_debugdump::print_schedule(&mut app);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin)
        .insert_resource(systems::wasm::IphoneMessage::NotDeleted)
        .add_system(systems::wasm::resize.system())
        .add_system(systems::wasm::remove_message.system());

    app.run();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn on_output(output: &str) {
    systems::networking::BUFFER.lock().push(output.into());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn on_name_change(name: &str) {
    *systems::networking::NAME.lock() = name.into();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn on_click_random() {
    *systems::networking::RANDOM_BUTTON.lock() = true;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn on_click_room() {
    *systems::networking::ROOM_BUTTON.lock() = true;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn on_click_share() {
    *systems::networking::SHARE_BUTTON.lock() = true;
}
