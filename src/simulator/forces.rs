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
        app.register_type::<WeightForce>();
        app.register_type::<BuoyantForce>();
        app.register_type::<DragForce>();

        app.add_systems(Update, (
            update_weight_force,
            update_buoyant_force,
        ));
    }
}

#[derive(Bundle)]
pub struct ForcesBundle {
    pub weight_force: WeightForce,
    pub buoyant_force: BuoyantForce,
    pub drag_force: DragForce,
}

impl Default for ForcesBundle {
    fn default() -> Self {
        Self {
            weight_force: WeightForce::ZERO,
            buoyant_force: BuoyantForce::ZERO,
            drag_force: DragForce::ZERO,
        }
    }
}

#[derive(Component, Reflect)]
pub struct WeightForce(ExternalForce);

impl WeightForce {
    pub const ZERO: Self = Self(ExternalForce::ZERO);

    pub fn new(mass: f32, position: Vec3) -> Self {
        Self(ExternalForce::new(weight(position, mass)))
    }

    pub fn update(&mut self, position: Vec3, mass: f32) {
        self.0.set_force(weight(position, mass));
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

fn update_weight_force(mut bodies: Query<(&mut WeightForce, &Position, &Mass)>) {
    for (mut weight_force, position, mass) in bodies.iter_mut() {
        weight_force.update(position.0, mass.kg());
    }
}

#[derive(Component, Reflect)]
pub struct BuoyantForce(ExternalForce);

impl BuoyantForce {
    pub const ZERO: Self = Self(ExternalForce::ZERO);

    pub fn new(position: Vec3, volume: Volume, density: Density) -> Self {
        Self(ExternalForce::new(buoyancy(position, volume, density)))
    }

    pub fn update(&mut self, position: Vec3, volume: Volume, density: Density) {
        self.0.set_force(buoyancy(position, volume, density));
    }
}

/// Upward force (N) due to atmosphere displaced by the given gas volume.
pub fn buoyancy(position: Vec3, volume: Volume, ambient_density: Density) -> Vec3 {
    Vec3::Y * (volume.cubic_meters() * ambient_density.kg_per_m3() * g(position))
}

fn update_buoyant_force(atmosphere: Res<Atmosphere>, mut bodies: Query<(&mut BuoyantForce, &Position, &Volume)>) {
    for (mut buoyant_force, position, volume) in bodies.iter_mut() {
        let density = atmosphere.density(position.0);
        buoyant_force.update(position.0, *volume, density);
    }
}

#[derive(Component, Reflect)]
pub struct DragForce(ExternalForce);

impl DragForce {
    pub const ZERO: Self = Self(ExternalForce::ZERO);

    pub fn new(force: Vec3) -> Self {
        Self(ExternalForce::new(force))
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
