// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod camera;
pub mod controls;
mod dev_tools;
mod scene;

use camera::CameraPlugin;
use controls::ControlsPlugin;
use scene::ScenePlugin;

#[cfg(feature = "dev")]
use dev_tools::DevToolsPlugin;

use bevy::{
    prelude::*,
    asset::AssetMetaCheck,
};

use yahs_simulator::prelude::SimulatorPlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "🎈".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            SimulatorPlugins,
            ControlsPlugin,
            ScenePlugin,
            CameraPlugin,
            #[cfg(feature = "dev")]
            DevToolsPlugin,
        ))
        .run();
}
