#[cfg(feature = "dev")]
mod dev_tools;

use bevy::{app::PluginGroupBuilder, prelude::*};

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(CoreUiPlugin)
    }
}

/// Base UI plugin. This sets up the base plugins that all other ui plugins
/// need. .Placeholder for now
pub struct CoreUiPlugin;

impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // TODO: plugins that sets up the basics.

            #[cfg(feature = "dev")]
            dev_tools::plugin,
        ));
    }
}
