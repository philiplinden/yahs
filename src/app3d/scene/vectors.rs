use bevy::{prelude::*, render::primitives::Aabb};
use avian3d::math::PI;
use crate::simulator::forces::Force;

pub struct ForceVectorPlugin;

impl Plugin for ForceVectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_axes);
    }
}

/// System to visualize all force vectors
// fn visualize_forces(
//     gizmos: Res<Gizmos>,
//     query: Query<(&Transform, &dyn Force)>,
// ) {
//     for (transform, all_forces) in query.iter() {
//         let origin = transform.translation;
//         let segments: Vec<(Vec3, Vec3)> = all_forces.iter().map(|force| {
//             let force_vector = force.force();
//             (origin, origin + force_vector)
//         }).collect();

//     }
// } 

// This system draws the axes based on the cube's transform, with length based on the size of
// the entity's axis-aligned bounding box (AABB).
fn draw_axes(mut gizmos: Gizmos, query: Query<(&Transform, &Aabb)>) {
    for (&transform, &aabb) in &query {
        let length = aabb.half_extents.length() * 0.1;
        gizmos.axes(transform, length);
    }
}
