//! Forces applied to rigid bodies.
mod aero;
mod body;

use avian3d::prelude::*;
use bevy::prelude::*;

// Re-export common forces
#[allow(unused_imports)]
pub use aero::Drag;
#[allow(unused_imports)]
pub use body::{Buoyancy, Weight};

use crate::{
    gas::Atmosphere,
    vehicle::Balloon,
    core::{SimState, SimulationUpdateOrder},
    thermodynamics::{Density, Volume},
};
pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        // Disable the default forces since we apply our own.
        app.insert_resource(Gravity(Vec3::ZERO));
        app.add_plugins((aero::AeroForcesPlugin, body::BodyForcesPlugin));
    }
}

/// This trait is used to identify a force vector component. All forces are
/// collected and summed to determine the net force acting on a rigid body. All
/// forces assume a right-handed Y-up coordinate frame and are reported in
/// Newtons.
pub trait Force {
    fn name(&self) -> String {
        String::from("Force")
    }
    fn force(&self) -> Vec3;
    fn direction(&self) -> Vec3 {
        self.force().normalize()
    }
    fn magnitude(&self) -> f32 {
        self.force().length()
    }
    fn point_of_application(&self) -> Vec3;
    fn torque(&self) -> Vec3 {
        Vec3::ZERO
    }
    fn color(&self) -> Option<Color> {
        None
    }
}
