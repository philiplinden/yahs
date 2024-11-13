//! Forces that act in the vertical axis. All forces assume a positive-up
//! coordinate frame and are reported in Newtons.
#![allow(dead_code)]

use bevy::prelude::*;
use avian3d::prelude::*;

use super::{Density, Volume, Atmosphere, Mass};

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        // Disable the default gravity since we apply our own.
        app.insert_resource(Gravity(Vec3::ZERO));
        // Update forces before solving physics.
        app.add_systems(Update, (
            apply_weight_force,
            apply_buoyant_force,
        ).in_set(PhysicsStepSet::First));
    }
}

/// Force (N) from gravity at an altitude (m) above mean sea level.
fn g(position: Vec3) -> f32 {
    let altitude = position.y; // [m]
    STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude))
}

/// Downward force (N) due to gravity as a function of altitude (m) and mass (kg).
pub fn weight(position: Vec3, mass: f32) -> Vec3 {
    Vec3::NEG_Y * g(position) * mass // [N]
}

fn apply_weight_force(mut bodies: Query<(&mut ExternalForce, &Position, &Mass), With<RigidBody>>) {
    for (mut total_force, position, mass) in bodies.iter_mut() {
        total_force.apply_force(weight(position.0, mass.kg()));
    }
}

/// Upward force (N) due to atmosphere displaced by the given gas volume.
pub fn buoyancy(position: Vec3, volume: Volume, ambient_density: Density) -> Vec3 {
    Vec3::Y * (volume.cubic_meters() * ambient_density.kg_per_m3() * g(position))
}

fn apply_buoyant_force(atmosphere: Res<Atmosphere>, mut bodies: Query<(&mut ExternalForce, &Position, &Volume), With<RigidBody>>) {
    for (mut total_force, position, volume) in bodies.iter_mut() {
        let density = atmosphere.density(position.0);
        total_force.apply_force(buoyancy(position.0, *volume, density));
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, velocity: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let direction = -velocity.normalize();
    direction * drag_coeff / 2.0 * ambient_density * f32::powf(velocity.length(), 2.0) * drag_area
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
