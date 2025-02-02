use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gas::Atmosphere,
    vehicle::balloon::Balloon,
    forces::{ForceVector, ForceType, Forces},
    geometry::Volume,
    thermodynamics::Density,
};

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up (it opposes gravity).
pub fn buoyancy(position: Vec3, displaced_volume: Volume, ambient_density: Density) -> Vec3 {
    use super::weight::gravity;
    Vec3::Y * (displaced_volume.m3() * ambient_density.kg_per_m3() * gravity(position).length())
}

#[derive(Component, Default)]
#[require(Position, Volume, Forces)]
pub struct BuoyancyForce;

pub(super) fn update_buoyancy_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Forces, &Position, &Volume), With<BuoyancyForce>>,
) {
    for (mut forces, position, volume) in bodies.iter_mut() {
        let ambient_density = atmosphere.density(position.0);
        let buoyancy_force = buoyancy(position.0, *volume, ambient_density);

        if let Some(force) = forces.vectors.iter_mut().find(|f| f.force_type == ForceType::Buoyancy) {
            force.force = buoyancy_force;
            force.point = position.0;
        } else {
            let force = ForceVector {
                name: "Buoyancy".to_string(),
                force: buoyancy_force,
                point: position.0,
                color: Some(Color::srgb(0.0, 0.0, 1.0)),
                force_type: ForceType::Buoyancy,
            };
            forces.add(force);
        }
    }
}
