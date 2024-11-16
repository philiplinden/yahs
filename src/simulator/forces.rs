//! Forces that act in the vertical axis. All forces assume a positive-up
//! coordinate frame and are reported in Newtons.
#![allow(dead_code)]

use avian3d::prelude::*;
use bevy::prelude::*;

use super::{Atmosphere, Density, Mass, Volume};

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WeightForce>();
        app.register_type::<BuoyantForce>();

        // Disable the default forces since we apply our own.
        app.insert_resource(Gravity(Vec3::ZERO));

        // Update forces before solving physics.
        app.add_systems(
            Update,
            (update_weight_force, update_buoyant_force, update_drag_force)
                .before(update_total_force)
                .in_set(PhysicsStepSet::First),
        );
    }
}

/// All the forces that act on a rigid body.
#[derive(Bundle)]
pub struct ForcesBundle {
    weight: WeightForce,
    buoyancy: BuoyantForce,
    drag: DragForce,
}

impl Default for ForcesBundle {
    fn default() -> Self {
        ForcesBundle {
            weight: WeightForce(Vec3::ZERO),
            buoyancy: BuoyantForce(Vec3::ZERO),
            drag: DragForce(Vec3::ZERO),
        }
    }
}

#[derive(Component, Reflect)]
pub struct WeightForce(Vec3);

/// Force (N) from gravity at an altitude (m) above mean sea level.
fn g(position: Vec3) -> f32 {
    let altitude = position.y; // [m]
    STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude))
}

/// Downward force (N) vector due to gravity as a function of altitude (m) and
/// mass (kg). The direction of this force is always world-space down.
pub fn weight(position: Vec3, mass: f32) -> Vec3 {
    Vec3::NEG_Y * g(position) * mass // [N]
}

fn update_weight_force(mut bodies: Query<(&mut WeightForce, &Position, &Mass), With<RigidBody>>) {
    for (mut force, position, mass) in bodies.iter_mut() {
        *force = WeightForce(weight(position.0, mass.kg()));
    }
}

#[derive(Component, Reflect)]
pub struct BuoyantForce(Vec3);

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up.
pub fn buoyancy(position: Vec3, volume: Volume, ambient_density: Density) -> Vec3 {
    Vec3::Y * (volume.cubic_meters() * ambient_density.kg_per_m3() * g(position))
}

fn update_buoyant_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut BuoyantForce, &Position, &Volume), With<RigidBody>>,
) {
    for (mut force, position, volume) in bodies.iter_mut() {
        let density = atmosphere.density(position.0);
        *force = BuoyantForce(buoyancy(position.0, *volume, density));
    }
}

#[derive(Component, Reflect)]
pub struct DragForce(Vec3);

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, velocity: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let direction = -velocity.normalize();
    direction * drag_coeff / 2.0 * ambient_density * f32::powf(velocity.length(), 2.0) * drag_area
}

fn update_drag_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<
        (
            &mut DragForce,
            &Position,
            &LinearVelocity,
            // &DragArea,
            // &DragCoeff,
        ),
        With<RigidBody>,
    >,
) {
    // Todo: update drag force
}

fn update_total_force(
    mut forces: Query<
        (&mut ExternalForce, &WeightForce, &BuoyantForce, &DragForce),
        With<RigidBody>,
    >,
) {
    for (mut total_force, weight, buoyancy, drag) in forces.iter_mut() {
        total_force.set_force(weight.0 + buoyancy.0 + drag.0);
    }
}
