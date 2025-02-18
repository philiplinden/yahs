use bevy::{prelude::*, render::mesh::Mesh};
use avian3d::math::Scalar;

use crate::constants::PI;
use crate::units::VolumeUnit;

// pub(super) fn plugin(app: &mut App) {
//     app.register_type::<HasMeshVolume>();
// }

pub(crate) fn sphere_volume(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * f32::powf(radius, 3.0)
}

pub(crate) fn sphere_radius_from_volume(volume: f32) -> f32 {
    f32::powf(volume * 3.0 / (4.0 * PI), 1.0 / 3.0)
}

pub(crate) fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
    let external_radius = internal_radius + thickness;
    let internal_volume = sphere_volume(internal_radius);
    let external_volume = sphere_volume(external_radius);
    external_volume - internal_volume
}

pub(crate) fn sphere_surface_area(radius: f32) -> f32 {
    4.0 * PI * f32::powf(radius, 2.0)
}

#[derive(Component, Default, Debug, Reflect)]
pub struct HasMeshVolume {
    pub handle: Handle<Mesh>,
}

pub trait MeshVolume {
    fn volume(&self) -> VolumeUnit;
}

impl MeshVolume for Mesh {
    fn volume(&self) -> VolumeUnit {
        let positions = match self
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|v| v.as_float3())
        {
            Some(p) => p,
            None => return VolumeUnit(0.0),
        };
        let indices = match self
            .indices()
            .map(|i| i.iter().map(|i| i as usize).collect::<Vec<_>>())
        {
            Some(i) => i,
            None => return VolumeUnit(0.0),
        };

        let mut volume = 0.0;

        // Calculate volume using signed tetrahedron method
        for triangle in indices.chunks(3) {
            if triangle.len() != 3 {
                continue;
            }

            let v1 = Vec3::from(positions[triangle[0]]);
            let v2 = Vec3::from(positions[triangle[1]]);
            let v3 = Vec3::from(positions[triangle[2]]);

            // Calculate signed volume of tetrahedron formed with origin
            volume += v1.dot(v2.cross(v3)) / 6.0;
        }

        VolumeUnit(volume.abs())
    }
}

/// https://docs.rs/bevy/latest/bevy/math/bounding/struct.RayCast3d.html
/// https://github.com/bevyengine/bevy/blob/latest/examples/math/sampling_primitives.rs
/// https://github.com/aevyrie/bevy_mod_raycast
#[allow(dead_code)]
trait ProjectedArea {
    fn projected_area(&self, direction: Vec3) -> f32;
}

#[allow(dead_code)]
fn generate_projection_plane(
    center: Vec3,
    normal: Vec3,
    width: f32,
    height: f32,
    resolution: usize,
) -> Vec<Vec3> {
    let mut points = Vec::new();
    let right = Vec3::new(1.0, 0.0, 0.0).cross(normal).normalize();
    let up = normal.cross(right).normalize();

    let step_x = width / resolution as f32;
    let step_y = height / resolution as f32;

    points.extend((0..resolution).flat_map(|i| {
        (0..resolution).map(move |j| {
            let x = (i as f32 - resolution as f32 / 2.0) * step_x;
            let y = (j as f32 - resolution as f32 / 2.0) * step_y;
            center + x * right + y * up
        })
    }));

    points
}

pub struct DragCoefficient(pub Scalar);

impl Default for DragCoefficient {
    fn default() -> Self {
        DragCoefficient(0.47)
    }
}
