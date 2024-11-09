//! Forces that act in the vertical axis. All forces assume a positive-up
//! coordinate frame and are reported in Newtons.
#![allow(dead_code)]

use bevy::prelude::*;
use avian3d::prelude::*;
use super::constants::{EARTH_RADIUS_M, STANDARD_G};
use super::{gas::GasVolume, atmosphere::Atmosphere};

pub struct DynamicsPlugin;

impl Plugin for DynamicsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, step);
    }
}

struct SolidBody {
    mass: Mass,
    volume: Mesh,
}

/// Acceleration (m/s²) from gravity at an altitude (m) above mean sea level.
fn g(position: Vec3) -> Vec3 {
    Vec3::NEG_Y * STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + position.y)) // [m/s²]
}

/// Weight (N) as a function of altitude (m) and mass (kg).
pub fn weight(position: Vec3, mass: f32) -> Vec3 {
    g(position) * mass // [N]
}

/// Force (N) due to air displaced by the given gas volume.
pub fn buoyancy(body: Collider, position: Vec3, atmosphere: Atmosphere) -> Vec3 {
    let v = body.volume();
    if v > 0.0 {
        let rho_atmo = atmosphere.density(position);
        let rho_lift = gas_volume.density();
        return v * (rho_lift - rho_atmo) * g(position)
    } else {
        Vec3::ZERO
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, velocity: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let direction = -velocity.normalize();
    direction * drag_coeff / 2.0 * ambient_density * f32::powf(velocity.length(), 2.0) * drag_area
}
