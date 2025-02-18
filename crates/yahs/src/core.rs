use super::*;
use avian3d::prelude::{PhysicsPlugins, PhysicsSet, PhysicsInterpolationPlugin};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};

#[cfg(feature = "dev")]
use avian3d::debug_render::PhysicsDebugPlugin;

pub struct SimulatorPlugins;

impl PluginGroup for SimulatorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePhysicsPlugin)
            .add(SimStatePlugin)
            .add(dynamics::plugin)
            .add(ideal_gas::plugin)
            .add(atmosphere::plugin)
            .add(objects::balloon::plugin)
    }
}

/// The plugin that loads the physics engine. We keep this as a separate plugin
/// so that we can configure it as needed before passing it to the PluginGroup.
struct CorePhysicsPlugin;

impl Plugin for CorePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}

/// The plugin that handles the overall simulation state.
struct SimStatePlugin;

impl Plugin for SimStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimState>();
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    Stopped,
    #[default]
    Running,
    Faulted,
}
