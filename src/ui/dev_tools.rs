//! Development tools for the game. This plugin is only enabled in dev builds.
#[allow(unused_imports)]
use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};

use avian3d::debug_render::PhysicsDebugPlugin;

const TOGGLE_DEBUG_UI_KEY: KeyCode = KeyCode::F3;
const TOGGLE_WIREFRAME_KEY: KeyCode = KeyCode::F4;

pub(super) fn plugin(app: &mut App) {
    // Toggle the debug overlay for UI.
    app.add_plugins((
        DebugUiPlugin,
        #[cfg(not(target_arch = "wasm32"))]
        WireframePlugin,
        WorldInspectorPlugin::new(),
        PhysicsDebugPlugin::default(),
    ));
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_DEBUG_UI_KEY)),
    );
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(
        Update,
        toggle_wireframe.run_if(input_just_pressed(TOGGLE_WIREFRAME_KEY)),
    );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(TOGGLE_WIREFRAME_KEY) {
        wireframe_config.global = !wireframe_config.global;
    }
}
