//! Forces that act in the vertical axis. All forces assume a positive-up
//! coordinate frame and are reported in Newtons.
#![allow(dead_code)]

use bevy::prelude::*;

use crate::simulator::thermodynamics::{Density, Volume};

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub struct DynamicsPlugin;

impl Plugin for DynamicsPlugin {
    fn build(&self, _app: &mut App) {
    }
}

/// Acceleration (m/s²) from gravity at an altitude (m) above mean sea level.
fn g(position: Vec3) -> Vec3 {
    Vec3::NEG_Y * (STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + position.y)))
}

/// Weight (N) as a function of altitude (m) and mass (kg).
pub fn weight(position: Vec3, mass: f32) -> Vec3 {
    g(position) * mass // [N]
}

/// Force (N) due to air displaced by the given gas volume.
pub fn buoyancy(position: Vec3, volume: Volume, density: Density, ambient_density: Density) -> Vec3 {
    let v = volume.0;
    if v <= 0.0 {
        // No buoyancy if the volume is zero
        return Vec3::ZERO
    }

    let rho_lift = density.0;
    let rho_atmo = ambient_density.0;
    (v * (rho_lift - rho_atmo)) * g(position)
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, velocity: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let direction = -velocity.normalize();
    direction * drag_coeff / 2.0 * ambient_density * f32::powf(velocity.length(), 2.0) * drag_area
}
