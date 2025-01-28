//! Forces applied to rigid bodies due to aerodynamic drag.

use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{
    gas::Atmosphere,
    vehicle::balloon::Balloon,
    forces::{ForceVector, Forces},
    thermodynamics::Density,
};

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(velocity: Vec3, ambient_density: f32, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let drag_direction = -velocity.normalize_or_zero(); // oppose the object's velocity
    let drag_magnitude = drag_coeff / 2.0 * ambient_density * velocity.length_squared() * drag_area;
    drag_direction * drag_magnitude
}

#[derive(Component, Default)]
#[require(Position, LinearVelocity, Balloon, Forces)]
pub struct DragForce;

pub(super) fn update_drag_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Forces, &Position, &LinearVelocity, &Balloon), With<DragForce>>,
) {
    for (mut forces, position, velocity, balloon) in bodies.iter_mut() {
        let force = ForceVector {
            name: "Drag".to_string(),
            force: drag(
                velocity.0,
                atmosphere.density(position.0).kg_per_m3(),
                PI * balloon.shape.diameter(),
                1.17, // default drag coefficient for a sphere
            ),
            point: position.0,
            color: Some(Color::srgb(1.0, 1.0, 0.0)),
        };
        forces.add(force);
    }
}
