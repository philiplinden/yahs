mod camera;
mod controls;
mod scene;
mod colors;

use camera::CameraPlugin;
use controls::ControlsPlugin;
use scene::ScenePlugin;

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
        ));

        #[cfg(feature = "inspect")]
        app.add_plugins(WorldInspectorPlugin::new());
    }
}
