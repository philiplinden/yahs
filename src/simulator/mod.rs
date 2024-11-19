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
        app.add_systems(Update, pause_physics_time);
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    #[default]
    Running,
    Stopped,
    Anomaly,
}

fn pause_physics_time(
    sim_state: Res<State<SimState>>,
    mut physics_time: ResMut<Time<Physics>>) {
        match sim_state.as_ref().get() {
            SimState::Running => physics_time.unpause(),
            SimState::Stopped => physics_time.pause(),
            SimState::Anomaly => physics_time.pause(),
        }
}
