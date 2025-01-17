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
            .add(gas::AtmospherePlugin)
            .add(vehicle::BalloonPlugin)
            .add(vehicle::PayloadPlugin)
    }
}

struct CorePhysicsPlugin;

impl Plugin for CorePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins((
            thermodynamics::CorePropertiesPlugin,
            gas::IdealGasPlugin,
            forces::ForcesPlugin,
            time::TimeScalePlugin,
        ));
        app.init_state::<SimState>();
    }
}

#[allow(dead_code)]
#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    #[default]
    Stopped,
    Running,
    Faulted,
}
