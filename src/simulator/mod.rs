pub mod balloon;
// pub mod bus;
pub mod constants;
pub mod forces;
pub mod gas;
pub mod atmosphere;
pub mod units;
// pub mod heat;

use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

pub trait SolidBody {
    fn drag_area(&self) -> f32;
    fn drag_coeff(&self) -> f32;
}

pub struct SimulatorPlugins;

impl PluginGroup for SimulatorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(atmosphere::AtmospherePlugin)
            .add(balloon::BalloonPlugin)
    }
}

// pub fn step() {

//     let total_dry_mass = body.total_mass() + parachute.total_mass();
//     let weight_force = forces::weight(altitude, total_dry_mass);
//     let buoyancy_force = forces::buoyancy(altitude, atmosphere, balloon.lift_gas);

//     let total_drag_force = forces::drag(atmosphere, ascent_rate, balloon)
//         + forces::drag(atmosphere, ascent_rate, body)
//         + forces::drag(atmosphere, ascent_rate, parachute.main)
//         + forces::drag(atmosphere, ascent_rate, parachute.drogue);
//     debug!(
//         "weight: {:?} buoyancy: {:?} drag: {:?}",
//         weight_force, buoyancy_force, total_drag_force
//     );

//     // calculate the net force
//     let net_force = weight_force + buoyancy_force + total_drag_force;
//     let acceleration = net_force / total_dry_mass;
//     let ascent_rate = ascent_rate + acceleration * delta_t;
//     let altitude = altitude + ascent_rate * delta_t;
// }
