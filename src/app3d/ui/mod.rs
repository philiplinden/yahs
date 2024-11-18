// mod monitors;

#[cfg(feature = "dev")]
mod dev_tools;

use bevy::{app::PluginGroupBuilder, prelude::*};
use iyes_perf_ui::prelude::*;

/// A plugin group that includes all interface-related plugins
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreUiPlugin)
            .add(monitors::ForceMonitorPlugin)
    }
}

/// Base UI plugin. This sets up the base plugins that all other ui plugins
/// need. .Placeholder for now
pub struct CoreUiPlugin;

impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PerfUiPlugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
        ));
    }
}
