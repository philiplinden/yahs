use avian3d::prelude::*;
use bevy::prelude::*;
use super::{
    ForceVector,
    Forces,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_net_forces);
}

fn update_net_forces(
    mut query: Query<(
        &mut ExternalForce,
        &mut ExternalTorque,
        &mut Forces,
    )>,
) {
    for (mut ext_force, mut ext_torque, mut forces) in query.iter_mut() {
        // Clear previous frame's forces
        ext_force.clear();
        ext_torque.clear();

        // Apply all forces
        for force in &forces.vectors {
            ext_force.apply_force_at_point(force.force, force.point, Vec3::ZERO);
        }
        forces.clear();
    }
}
