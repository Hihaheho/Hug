mod components;
mod plugins;
mod systems;
mod adapters;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use plugins::body::BodyPlugin;
use wasm_bindgen::prelude::*;

use components::state::AppState;
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
    win.scale_factor_override = Some(1.0);
    #[cfg(target_arch = "wasm32")]
    {
        let js_window = web_sys::window().unwrap();
        let width = js_window.inner_width().unwrap().as_f64().unwrap() as f32;
        let height = js_window.inner_height().unwrap().as_f64().unwrap() as f32;
        let ratio = WIDTH / width;
        win.scale_factor_override = Some(ratio as f64);
    }

    app.insert_resource(win)
        .add_plugins(DefaultPlugins)
        .add_plugin(BodyPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(setup_player.system())
        .add_state(AppState::Alone);

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
