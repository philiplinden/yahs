use super::*;
use avian3d::prelude::*;
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};

/// A marker component for entities that are simulated.
#[derive(Component, Default)]
pub struct SimulatedBody;

pub struct SimulatorPlugins;

impl PluginGroup for SimulatorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePhysicsPlugin)
            .add(atmosphere::AtmospherePlugin)
            .add(balloon::BalloonPlugin)
    }
}

struct CorePhysicsPlugin;

impl Plugin for CorePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins((
            properties::CorePropertiesPlugin,
            ideal_gas::IdealGasPlugin,
            forces::ForcesPlugin,
        ));
        app.init_state::<SimState>();
        app.add_systems(
            OnEnter(SimState::Running),
            |mut time: ResMut<Time<Physics>>| time.as_mut().unpause(),
        );
        app.add_systems(
            OnExit(SimState::Running),
            |mut time: ResMut<Time<Physics>>| time.as_mut().pause(),
        );
        app.configure_sets(
            Update,
            (
                SimulationUpdateOrder::First,
                SimulationUpdateOrder::IdealGas,
                SimulationUpdateOrder::MeshVolumes,
                SimulationUpdateOrder::Forces,
                SimulationUpdateOrder::Last,
            ).chain()
                .before(PhysicsSet::Prepare)
                .run_if(in_state(SimState::Running)),
        );
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationUpdateOrder {
    First,
    IdealGas,
    MeshVolumes,
    Forces,
    Last,
}
