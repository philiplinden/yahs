use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
};
use bevy_egui::EguiPlugin;
mod shell;
// mod designer;

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreUiPlugin)
            // TODO: Add other ui plugins here
    }
}

/// Base UI plugin. This sets up Bevy Egui and the "shell" or frame for the UI.
pub struct CoreUiPlugin;

impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, shell::ShellPlugin));
    }
}
