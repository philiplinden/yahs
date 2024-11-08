use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod designer;
mod shell;
mod splash;

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EguiPlugin)
            .add(shell::ShellPlugin)
            .add(designer::BalloonDesignerPlugin)
            .add(splash::SplashPlugin)
    }
}
