use avian3d::prelude::*;
use bevy::{prelude::*, render::mesh::Mesh};

use crate::{
    forces::{ForceType, ForceVector, Forces},
    gas::Atmosphere,
    geometry::{HasMeshVolume, MeshVolume},
    units::{DensityUnit, VolumeUnit},
    vehicle::balloon::Balloon,
};


/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up (it opposes gravity).
pub fn buoyancy(
    position: Vec3,
    displaced_volume: VolumeUnit,
    ambient_density: DensityUnit,
) -> Vec3 {
    use super::weight::gravity;
    Vec3::Y * (displaced_volume.m3() * ambient_density.kg_per_m3() * gravity(position).length())
}

#[derive(Component, Default)]
#[require(Position, Forces, HasMeshVolume)]
pub struct BuoyancyForce;

pub(super) fn update_buoyancy_force(
    atmosphere: Res<Atmosphere>,
    meshes: Res<Assets<Mesh>>,
    mut bodies: Query<(&mut Forces, &Position, &HasMeshVolume), With<BuoyancyForce>>,
) {
    for (mut forces, position, mesh_volume) in bodies.iter_mut() {
        let ambient_density = atmosphere.density(position.0);

        // Get the mesh and calculate its volume
        let Some(mesh) = meshes.get(&mesh_volume.handle) else {
            continue;
        };

        let volume = mesh.volume();


        let buoyancy_force = buoyancy(position.0, volume, ambient_density);

        if let Some(force) = forces
            .vectors
            .iter_mut()
            .find(|f| f.force_type == ForceType::Buoyancy)
        {
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
