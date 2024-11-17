use avian3d::prelude::*;
use bevy::prelude::*;

pub struct AeroPlugin;

impl Plugin for AeroPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Add systems to update drag parameters.
    }
}

pub fn aero_drag_from_collider(collider: &Collider, velocity: LinearVelocity) -> (Vec3, f32, f32) {
    (collider.drag_normal, collider.drag_area, collider.drag_coeff)
}
