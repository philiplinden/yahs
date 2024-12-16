use bevy::{prelude::*, state::app::StatesPlugin};
use yahs_simulator::prelude::SimulatorPlugins;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            StatesPlugin,
            SimulatorPlugins,
        ))
        .run();
} 
