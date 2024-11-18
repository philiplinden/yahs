//! Forces applied to rigid bodies due to aerodynamic drag.

use avian3d::{math::PI, prelude::*};
use parry3d::shape::{ShapeType, Shape, Ball};
use bevy::prelude::*;
use bevy_trait_query::{self, RegisterExt};

use super::{Atmosphere, Density, ForceUpdateOrder, Force, SimulatedBody};

pub struct AeroForcesPlugin;

impl Plugin for AeroForcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Drag>();
        app.register_component_as::<dyn Force, Drag>();
        app.add_systems(Update, update_drag_parameters.in_set(ForceUpdateOrder::Prepare));
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
#[derive(Component, Reflect)]
pub struct Drag {
    flow_velocity: Vec3,
    ambient_density: Density,
    drag_area: f32,
    drag_coeff: f32,
}
impl Default for Drag {
    fn default() -> Self {
        Self {
            flow_velocity: Vec3::ZERO,
            ambient_density: Density::ZERO,
            drag_area: 0.0,
            drag_coeff: 1.0,
        }
    }
}
impl Drag {
    pub fn update(
        &mut self,
        flow_velocity: Vec3,
        ambient_density: Density,
        drag_area: f32,
        drag_coeff: f32,
    ) {
        self.flow_velocity = flow_velocity;
        self.ambient_density = ambient_density;
        self.drag_area = drag_area;
        self.drag_coeff = drag_coeff;
    }
}
impl Force for Drag {
    fn name(&self) -> String {
        String::from("Drag")
    }
    fn force(&self) -> Vec3 {
        drag(
            self.flow_velocity,
            self.ambient_density.kg_per_m3(),
            self.drag_area,
            self.drag_coeff,
        )
    }
    fn point_of_application(&self) -> Vec3 {
        Vec3::ZERO
    }
}

fn update_drag_parameters(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Drag, &Position, &LinearVelocity, &Collider), With<SimulatedBody>>,
) {
    for (mut drag, position, velocity, collider) in bodies.iter_mut() {
        let bounding_sphere = collider.shape().compute_bounding_sphere(&position.0.into());
        drag.update(
            velocity.0,
            atmosphere.density(position.0),
            projected_spherical_area(bounding_sphere.radius()),
            drag_coefficient(&Ball::new(bounding_sphere.radius()), &atmosphere),
        );
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(flow_velocity: Vec3, ambient_density: f32, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let drag_direction = flow_velocity.normalize_or_zero(); // parallel to flow
    let drag_magnitude = drag_coeff / 2.0
        * ambient_density
        * flow_velocity.length_squared()
        * drag_area;
    drag_direction * drag_magnitude
}

/// Get the projected area (m^2) of a sphere with a given radius (m)
fn projected_spherical_area(radius: f32) -> f32 {
    f32::powf(radius, 2.0) * PI
}

/// Get the drag coefficient for a given shape and ambient conditions.
fn drag_coefficient(shape: &dyn Shape, _atmosphere: &Atmosphere) -> f32 {
    match shape.shape_type() {
        ShapeType::Ball => 1.17,
        ShapeType::Cuboid => 2.05,
        _ => 1.0,
    }
}
