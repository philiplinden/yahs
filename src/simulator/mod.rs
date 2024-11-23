pub mod atmosphere;
pub mod balloon;
pub mod forces;
pub mod ideal_gas;
pub mod payload;
pub mod properties;

use avian3d::prelude::*;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

// Re-export the properties module at the top level.
pub use atmosphere::Atmosphere;
#[allow(unused-imports)]
pub use properties::{Density, Pressure, Temperature, Volume};

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
        app.add_plugins((
            PhysicsPlugins::default(),
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
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    #[default]
    Running,
    Stopped,
    Anomaly,
}
