// mod assets;
mod simulator;
mod ui;

#[cfg(feature = "dev")]
mod dev_tools;

use bevy::{asset::AssetMetaCheck, prelude::*};

pub struct AppCorePlugin;

impl Plugin for AppCorePlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist)
                    // if this isn't set. This causes errors and even panics on
                    // web build on itch. See
                    // https://github.com/bevyengine/bevy_github_ci_template/issues/48.
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
        );

        // Add other plugins.
        app.add_plugins((
            simulator::SimulatorPlugins,
        ));

        #[cfg(feature = "gui")]
        app.add_plugins(ui::InterfacePlugins);

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}
