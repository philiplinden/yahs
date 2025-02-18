//! Forces applied to rigid bodies.
use avian3d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use std::ops::{Add, AddAssign};

use crate::{
    atmosphere::Atmosphere,
    constants::{EARTH_RADIUS_M, STANDARD_G},
    core::SimState,
    units::*,
};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec3::NEG_Y * STANDARD_G));
    app.add_systems(
        FixedUpdate, (update_gravity)
            .chain()
            .in_set(PhysicsStepSet::First)
            .run_if(in_state(SimState::Running)),
    );
}

/// Fraction of standard gravity at an altitude (m) above mean sea level.
pub fn scale_gravity(altitude_meters: Scalar) -> Scalar {
    EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude_meters)
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(
    velocity: Vec3,
    ambient_density: DensityUnit,
    drag_area: AreaUnit,
    drag_coefficient: Scalar,
) -> Vec3 {
    -0.5 * drag_coefficient
        * ambient_density.kg_per_m3()
        * drag_area.m2()
        * velocity.length()
        * velocity
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up (it opposes gravity).
pub fn buoyancy(
    gravity_acceleration: AccelerationUnit,
    displaced_volume: VolumeUnit,
    ambient_density: DensityUnit,
) -> Vec3 {
    Vec3::Y
        * (displaced_volume.m3() * ambient_density.kg_per_m3() * gravity_acceleration.m_per_s2())
}

fn update_gravity(mut query: Query<(&mut GravityScale, &Position)>) {
    for (mut gravity_scale, position) in query.iter_mut() {
        gravity_scale.0 = scale_gravity(position.y);
    }
}

// fn apply_buoyancy(
//     mut query: Query<(&mut ExternalForce, &Position, &GravityScale, &Collider)>,
//     atmosphere: Res<Atmosphere>,
//     gravity: Res<Gravity>,
// ) -> Vec3 {
//     for (mut external_force, position, gravity_scale, collider) in query.iter_mut() {
//         let gravity_acceleration = gravity.0 * gravity_scale.0;
//         let ambient_density = atmosphere.density(position.y);
//         let displaced_volume = collider.volume();
//         let buoyancy_force = buoyancy(gravity_acceleration, displaced_volume, ambient_density);
//         external_force.apply_force(buoyancy_force);
//     }
// }
