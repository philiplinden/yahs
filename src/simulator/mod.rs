pub mod balloon;
// pub mod bus;
pub mod constants;
pub mod forces;
pub mod gas;
// pub mod heat;
pub mod schedule;

use bevy::prelude::*;

pub trait SolidBody {
    fn drag_area(&self) -> f32;
    fn drag_coeff(&self) -> f32;
    fn total_mass(&self) -> f32;
}

pub struct SimulatorPlugin;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut App) {
        // Add systems, resources, and plugins to your app here
    }
}
