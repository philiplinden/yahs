use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    vehicle::balloon::Balloon,
    forces::{ForceVector, Forces, Mass},
    thermodynamics::{EARTH_RADIUS_M, STANDARD_G},
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec3::ZERO));
    app.add_systems(FixedUpdate, update_weight_force);
}

/// Force (N) from gravity at an altitude (m) above mean sea level.
pub fn gravity(position: Vec3) -> Vec3 {
    let altitude = position.y; // [m]
    Vec3::NEG_Y * STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude))
}

/// Downward force (N) vector due to gravity as a function of altitude (m) and
/// mass (kg). The direction of this force is always world-space down.
pub fn weight(position: Vec3, mass: f32) -> Vec3 {
    gravity(position) * mass // [N]
}

#[derive(Component, Default)]
#[require(Mass, Position, Forces)]
pub struct WeightForce;


fn update_weight_force(mut bodies: Query<(&mut Forces, &Position, &Mass), With<WeightForce>>) {
    for (mut forces, position, mass) in bodies.iter_mut() {
        let force = ForceVector {
            name: "Weight".to_string(),
            force: weight(position.0, mass.0),
            point: position.0,
            color: Some(Color::srgb(0.0, 1.0, 0.0)),
        };
        forces.add(force);
    }
}
