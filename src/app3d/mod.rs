mod camera;
pub mod controls;
mod dev_tools;
mod monitors;
mod scene;

use camera::CameraPlugin;
use controls::ControlsPlugin;
use dev_tools::DevToolsPlugin;
use monitors::MonitorsPlugin;
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
            .add(ControlsPlugin)
            .add(ScenePlugin)
            .add(CameraPlugin)
    }
}

/// A plugin group that includes all interface-related plugins
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonitorsPlugin,
            #[cfg(feature = "dev")]
            DevToolsPlugin,
        ));
    }
}
