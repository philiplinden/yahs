use bevy::{
    math::{
        primitives::{Capsule3d, Cone, Cuboid, Cylinder, Sphere},
        bounding::RayCast3d
    },
    prelude::*,
};

/// https://docs.rs/bevy/latest/bevy/math/bounding/struct.RayCast3d.html
/// https://github.com/bevyengine/bevy/blob/latest/examples/math/sampling_primitives.rs
/// https://github.com/aevyrie/bevy_mod_raycast
trait ProjectedArea {
    fn projected_area(&self, direction: Vec3) -> f32;
}

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
