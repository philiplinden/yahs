//! Development tools for the game. This plugin is only enabled in dev builds.
use avian3d::debug_render::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
#[allow(unused_imports)]
use bevy::{
    color::palettes::basic::*,
    dev_tools::states::log_transitions,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use crate::{app3d::controls::KeyBindingsConfig, simulator::SimState};

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        // Toggle the debug overlay for UI.
        app.add_plugins((
        // physics
        PhysicsDebugPlugin::default(),
        // performance
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        // rendering
        #[cfg(not(target_arch = "wasm32"))]
        WireframePlugin,
    ));

    app.init_resource::<DebugState>();

    app.add_systems(Update, log_transitions::<SimState>);
    app.add_systems(Update, show_physics_gizmos);

    // Wireframe doesn't work on WASM
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_debug_ui);
    // #[cfg(feature = "inspect")]
    // {
    //     use bevy_inspector_egui::quick::WorldInspectorPlugin;
    //     app.add_plugins(WorldInspectorPlugin::new());
    // }
    }
}

#[derive(Debug, Resource)]
struct DebugState {
    wireframe: bool,
    forces: bool,
    physics: bool,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            wireframe: false,
            forces: true,
            physics: false,
        }
    }
}

impl DebugState {
    fn toggle_wireframe(&mut self) {
        self.wireframe = !self.wireframe;
        warn!("wireframe debug: {}", self.wireframe);
    }
    fn toggle_forces(&mut self) {
        self.forces = !self.forces;
        warn!("forces debug: {}", self.forces);
    }
    fn toggle_physics(&mut self) {
        self.physics = !self.physics;
        warn!("physics debug: {}", self.physics);
    }
}

#[allow(dead_code)]
#[derive(Component, Default)]
struct DebugUi;

#[cfg(not(target_arch = "wasm32"))]
fn toggle_debug_ui(
    mut wireframe_config: ResMut<WireframeConfig>,
    mut debug_state: ResMut<DebugState>,
    key_input: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindingsConfig>,
) {
    if key_input.just_pressed(key_bindings.debug_controls.toggle_1) {
        debug_state.toggle_wireframe();
        wireframe_config.global = !wireframe_config.global;
    }
    if key_input.just_pressed(key_bindings.debug_controls.toggle_2) {
        debug_state.toggle_forces();
    }
    if key_input.just_pressed(key_bindings.debug_controls.toggle_3) {
        debug_state.toggle_physics();
    }
}

fn show_physics_gizmos(
    debug_state: Res<DebugState>,
    mut gizmo_store: ResMut<GizmoConfigStore>
) {
    if gizmo_store.is_changed() {
        let (_, physics_config) = gizmo_store.config_mut::<PhysicsGizmos>();
        if debug_state.physics {
            *physics_config = PhysicsGizmos::all();
        } else {
            *physics_config = PhysicsGizmos::none();
        }
    }
}
