use super::*;
use avian3d::prelude::{PhysicsPlugins, PhysicsSet};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};

pub struct SimulatorPlugins;

impl PluginGroup for SimulatorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePhysicsPlugin)
            .add(time::TimeScalePlugin)
            .add(gas::atmosphere_plugin)
            .add(vehicle::balloon::plugin)
            .add(vehicle::payload::plugin)
    }
}

struct CorePhysicsPlugin;

impl Plugin for CorePhysicsPlugin {
    fn build(&self, app: &mut App) {
        // third party plugins
        app.add_plugins((
            PhysicsPlugins::default(),
            thermodynamics::plugin,
            gas::ideal_gas_plugin,
            forces::plugin,
            geometry::plugin,
        ));
        app.init_state::<SimState>();
    }
}

#[allow(dead_code)]
#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    Stopped,
    #[default]
    Running,
    Faulted,
}
