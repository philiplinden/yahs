mod camera;
pub mod controls;
mod dev_tools;
mod scene;

use camera::CameraPlugin;
use controls::ControlsPlugin;
use dev_tools::DevToolsPlugin;
use scene::ScenePlugin;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};

pub struct App3dPlugins;

impl PluginGroup for App3dPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UiPlugin)
            .add(RenderedObjectsPlugin)
    }
}

/// A plugin group that includes all interface-related plugins
struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ControlsPlugin,
            #[cfg(feature = "dev")]
            DevToolsPlugin,
        ));
    }
}

struct RenderedObjectsPlugin;

impl Plugin for RenderedObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScenePlugin, CameraPlugin));
    }
}
