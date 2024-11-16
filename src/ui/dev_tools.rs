//! Development tools for the game. This plugin is only enabled in dev builds.
#[allow(unused_imports)]
use bevy::{
    // dev_tools::states::log_transitions,
    diagnostic::{
        FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use iyes_perf_ui::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};

use avian3d::debug_render::PhysicsDebugPlugin;

const TOGGLE_DEBUG_UI_KEY: KeyCode = KeyCode::F3;
const TOGGLE_WIREFRAME_KEY: KeyCode = KeyCode::F4;

pub(super) fn plugin(app: &mut App) {
    // Toggle the debug overlay for UI.
    app.add_plugins((
        // physics
        PhysicsDebugPlugin::default(),
        // performance
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
        PerfUiPlugin,
        // rendering
        #[cfg(not(target_arch = "wasm32"))]
        WireframePlugin,
    ));
    app.add_systems(
        Update,
        toggle_debug_ui
            .before(iyes_perf_ui::PerfUiSet::Setup)
            .run_if(input_just_pressed(TOGGLE_DEBUG_UI_KEY)),
    );
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(
        Update,
        toggle_wireframe.run_if(input_just_pressed(TOGGLE_WIREFRAME_KEY)),
    );

    #[cfg(feature = "inspect")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::new());
    }
}

/// Toggle the debug overlay
fn toggle_debug_ui(
    mut commands: Commands,
    q_root: Query<Entity, With<PerfUiRoot>>,
) {
    if let Ok(e) = q_root.get_single() {
        // despawn the existing Perf UI
        commands.entity(e).despawn_recursive();
    } else {
        // create a simple Perf UI with default settings
        // and all entries provided by the crate:
        commands.spawn((
            PerfUiRoot {
                // set a fixed width to make all the bars line up
                values_col_width: Some(160.0),
                ..Default::default()
            },
            // when we have lots of entries, we have to group them
            // into tuples, because of Bevy Rust syntax limitations
            (
                PerfUiWidgetBar::new(PerfUiEntryFPS::default()),
                PerfUiWidgetBar::new(PerfUiEntryFPSWorst::default()),
                PerfUiWidgetBar::new(PerfUiEntryFrameTime::default()),
                PerfUiWidgetBar::new(PerfUiEntryFrameTimeWorst::default()),
                PerfUiWidgetBar::new(PerfUiEntryEntityCount::default()),
            ),
            (
                PerfUiEntryRunningTime::default(),
                PerfUiEntryClock::default(),
            ),
            (
                PerfUiEntryCursorPosition::default(),
                // PerfUiEntryWindowResolution::default(),
                // PerfUiEntryWindowScaleFactor::default(),
                // PerfUiEntryWindowMode::default(),
                // PerfUiEntryWindowPresentMode::default(),
            ),
        ));
    }
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
