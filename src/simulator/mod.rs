pub mod aero;
pub mod atmosphere;
pub mod balloon;
pub mod forces;
pub mod ideal_gas;
pub mod payload;
pub mod properties;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use avian3d::prelude::*;

// Re-export the properties module at the top level.
pub use properties::{Temperature, Pressure, Volume, Density, Mass};
pub use atmosphere::Atmosphere;

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
    }
}
