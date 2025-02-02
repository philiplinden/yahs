mod camera;
mod controls;
mod scene;
mod hud;
mod colors;
mod gizmos;

#[cfg(feature = "console")]
mod console;

use camera::CameraPlugin;
use controls::ControlsPlugin;
use scene::ScenePlugin;
use hud::HudPlugin;
use gizmos::KinematicsGizmos;


#[cfg(feature = "console")]
use console::DevConsolePlugin;

use bevy::{
    prelude::*,
    asset::AssetMetaCheck,
    log::LogPlugin,
};

#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use yahs::prelude::SimulatorPlugins;

pub struct AppPlugins;

impl Plugin for AppPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
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
                }).build().disable::<LogPlugin>(), // we set this elsewhere
            SimulatorPlugins,
            ControlsPlugin,
            ScenePlugin,
            CameraPlugin,
            KinematicsGizmos,
            HudPlugin,
        ));
        #[cfg(feature = "console")]
        app.add_plugins(DevConsolePlugin);
        #[cfg(not(feature = "console"))]
        app.add_plugins(LogPlugin::default());
        #[cfg(feature = "inspect")]
        app.add_plugins(WorldInspectorPlugin::new());
    }
}
