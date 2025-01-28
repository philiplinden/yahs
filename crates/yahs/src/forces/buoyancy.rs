use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gas::Atmosphere,
    vehicle::balloon::Balloon,
    forces::{ForceVector, Forces},
    geometry::Volume,
    thermodynamics::Density,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_buoyant_force);
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up (it opposes gravity).
pub fn buoyancy(position: Vec3, displaced_volume: Volume, ambient_density: Density) -> Vec3 {
    use super::weight::gravity;
    Vec3::Y * (displaced_volume.m3() * ambient_density.kg_per_m3() * gravity(position).length())
}

#[derive(Component, Default)]
#[require(Position, Volume, Forces)]
pub struct BuoyancyForce;

fn update_buoyant_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Forces, &Position, &Balloon), With<BuoyancyForce>>,
) {
    for (mut forces, position, balloon) in bodies.iter_mut() {
        let ambient_density = atmosphere.density(position.0);
        let displaced_volume = Volume(balloon.shape.volume());
        let force = ForceVector {
            name: "Buoyancy".to_string(),
            force: buoyancy(position.0, displaced_volume, ambient_density),
            point: position.0,
            color: Some(Color::srgb(0.0, 0.0, 1.0)),
        };
        forces.add(force);
    }
}
