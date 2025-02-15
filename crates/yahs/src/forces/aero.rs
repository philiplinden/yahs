//! Forces applied to rigid bodies due to aerodynamic drag.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gas::Atmosphere,
    vehicle::balloon::Balloon,
    forces::{ForceVector, ForceType, Forces},
    units::VolumeUnit,
    geometry::sphere_radius_from_volume,
    constants::PI,
};

/// Calculate the Reynolds number
fn reynolds_number(velocity: Vec3, density: f32, characteristic_length: f32, viscosity: f32) -> f32 {
    density * velocity.length() * characteristic_length / viscosity
}

/// Adjust the drag coefficient based on the Reynolds number
fn adjusted_drag_coefficient(re: f32) -> f32 {
    if re < 1e5 {
        0.47 // Laminar flow
    } else {
        0.1 // Turbulent flow
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(velocity: Vec3, ambient_density: f32, drag_area: f32, viscosity: f32) -> Vec3 {
    let re = reynolds_number(velocity, ambient_density, drag_area.sqrt(), viscosity);
    let drag_coeff = adjusted_drag_coefficient(re);
    -0.5 * drag_coeff * ambient_density * drag_area * velocity.length() * velocity
}

#[derive(Component, Default)]
#[require(Position, LinearVelocity, Forces)]
pub struct DragForce;

pub(super) fn update_drag_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Forces, &Position, &LinearVelocity, &Balloon), With<DragForce>>,
) {
    for (mut forces, position, velocity, balloon) in bodies.iter_mut() {
        let radius = sphere_radius_from_volume(balloon.volume().m3());
        let drag_area = PI * radius * radius; // cross-sectional area
        let viscosity = 1.81e-5; // Air viscosity at 15°C in kg/(m·s)

        let drag_force = drag(
            velocity.0,
            atmosphere.density(position.0).kg_per_m3(),
            drag_area,
            viscosity,
        );

        if let Some(force) = forces.vectors.iter_mut().find(|f| f.force_type == ForceType::Drag) {
            force.force = drag_force;
            force.point = position.0;
        } else {
            let force = ForceVector {
                name: "Drag".to_string(),
                force: drag_force,
                point: position.0,
                color: Some(Color::srgb(1.0, 1.0, 0.0)),
                force_type: ForceType::Drag,
            };
            forces.add(force);
        }
    }
}
