//! Development tools for the game. This plugin is only enabled in dev builds.
use avian3d::debug_render::PhysicsDebugPlugin;
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
use iyes_perf_ui::{prelude::*, entries::PerfUiBundle};

use crate::app3d::controls::KeyBindingsConfig;

pub(super) fn plugin(app: &mut App) {
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
    app.add_event::<SpawnPerfUi>();
    app.add_event::<DespawnPerfUi>();
    app.add_observer(spawn_perf_ui);
    app.add_observer(despawn_perf_ui);

    // Wireframe doesn't work on WASM
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_debug_ui);
    // #[cfg(feature = "inspect")]
    // {
    //     use bevy_inspector_egui::quick::WorldInspectorPlugin;
    //     app.add_plugins(WorldInspectorPlugin::new());
    // }
}

#[derive(Debug, Default, Resource)]
struct DebugState {
    wireframe: bool,
    physics_debug: bool,
    perf_ui: bool,
}

#[allow(dead_code)]
#[derive(Component, Default)]
struct DebugUi;

#[cfg(not(target_arch = "wasm32"))]
fn toggle_debug_ui(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut debug_state: ResMut<DebugState>,
    key_input: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindingsConfig>,
) {
    if key_input.just_pressed(key_bindings.debug_controls.toggle_wireframe) {
        debug_state.wireframe = !debug_state.wireframe;
        wireframe_config.global = !wireframe_config.global;
        warn!("wireframe: {}", debug_state.wireframe);
    }

    if key_input.just_pressed(key_bindings.debug_controls.toggle_physics_debug) {
        debug_state.physics_debug = !debug_state.physics_debug;
        warn!("physics debug: {} - not implemented", debug_state.physics_debug);
    }

    if key_input.just_pressed(key_bindings.debug_controls.toggle_perf_ui) {
        debug_state.perf_ui = !debug_state.perf_ui;
        warn!("perf ui: {}", debug_state.perf_ui);
        if debug_state.perf_ui {
            commands.trigger(SpawnPerfUi);
        } else {
            commands.trigger(DespawnPerfUi);
        }
    }
}

#[derive(Event, Default)]
struct SpawnPerfUi;

fn spawn_perf_ui(_trigger: Trigger<SpawnPerfUi>, mut commands: Commands) {
    info!("spawn_perf_ui");
    warn!("spawning perf ui DOES NOT WORK");
    commands.spawn((DebugUi,
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryClock::default(),
    ));
}

#[derive(Event, Default)]
struct DespawnPerfUi;

fn despawn_perf_ui(_trigger: Trigger<DespawnPerfUi>, mut commands: Commands, ui: Query<Entity, (With<DebugUi>, With<PerfUiRoot>)>) {
    info!("despawn_perf_ui");
    for ui in ui.iter() {
        commands.entity(ui).despawn_recursive();
    }
}
