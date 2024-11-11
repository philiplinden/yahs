mod shell;

#[cfg(feature = "dev")]
mod dev_tools;

use bevy::{
    app::PluginGroupBuilder, prelude::*,
};
use bevy_egui::EguiPlugin;

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreUiPlugin)
    }
}

/// Base UI plugin. This sets up Bevy Egui and the "shell" or frame for the UI.
pub struct CoreUiPlugin;

impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin,
            shell::ShellPlugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
        ));
    }
}
