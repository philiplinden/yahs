// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod camera;
pub mod controls;
mod scene;
mod hud;
mod colors;
mod gizmos;

#[cfg(feature = "dev")]
mod dev_tools;

use camera::CameraPlugin;
use controls::ControlsPlugin;
use scene::ScenePlugin;
use hud::HudPlugin;

use bevy::{
    prelude::*,
    asset::AssetMetaCheck,
    log::{LogPlugin, self},
};

use yahs::prelude::SimulatorPlugins;

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
                        title: "yet another hab simulator 🎈".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(LogPlugin {
                    level: log::Level::INFO,
                    filter: "info,capture_bevy_logs=info".to_owned(),
                    custom_layer: bevy_console::make_layer,
                }),
            SimulatorPlugins,
            ControlsPlugin,
            ScenePlugin,
            CameraPlugin,
            gizmos::KinematicsGizmos,
            HudPlugin,
            #[cfg(feature = "dev")]
            dev_tools::DevToolsPlugins,
        ))
        .run();
}
