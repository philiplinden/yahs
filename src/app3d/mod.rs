mod scene;
mod camera;
mod ui;
pub mod controls;

use scene::ScenePlugin;
use ui::InterfacePlugin;
use controls::ControlsPlugin;
use camera::CameraPlugin;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct App3dPlugins;

impl PluginGroup for App3dPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InterfacePlugin)
            .add(ControlsPlugin)
            .add(ScenePlugin)
            .add(CameraPlugin)
    }
}
