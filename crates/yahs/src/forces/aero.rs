//! Forces applied to rigid bodies due to aerodynamic drag.

use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{
    gas::Atmosphere,
    vehicle::Balloon,
    forces::{Density, Force},
};

pub struct AeroForcesPlugin;

impl Plugin for AeroForcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Drag>();

        // Physics systems should run at fixed rate for stability.
        app.add_systems(FixedUpdate, update_drag_parameters);
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
#[derive(Component, Reflect, Debug)]
pub struct Drag {
    position: Vec3,
    flow_velocity: Vec3,
    ambient_density: Density,
    drag_area: f32,
    drag_coeff: f32,
}
impl Default for Drag {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
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
        position: Vec3,
        flow_velocity: Vec3,
        ambient_density: Density,
        drag_area: f32,
        drag_coeff: f32,
    ) {
        self.position = position;
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
        self.position
    }
    fn color(&self) -> Option<Color> {
        Some(Color::srgb(1.0, 0.0, 0.0))
    }
}

fn update_drag_parameters(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Drag, &Position, &LinearVelocity, &Balloon)>,
) {
    for (mut drag, position, velocity, balloon) in bodies.iter_mut() {
        drag.update(
            position.0,
            velocity.0,
            atmosphere.density(position.0),
            PI * balloon.shape.diameter(),
            1.17, // default drag coefficient for a sphere
        );
        #[cfg(feature = "log")]
        info!("Updating Drag: Position: {:?}, Relative Flow Velocity: {:?}, Ambient Density: {:?}, Drag Area: {:?}, Drag Coefficient: {:?}", drag.position, drag.flow_velocity, drag.ambient_density, drag.drag_area, drag.drag_coeff);
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(velocity: Vec3, ambient_density: f32, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let drag_direction = -velocity.normalize_or_zero(); // oppose the object's velocity
    let drag_magnitude = drag_coeff / 2.0 * ambient_density * velocity.length_squared() * drag_area;
    drag_direction * drag_magnitude
}

// Get the drag coefficient for a given shape and ambient conditions.
// fn drag_coefficient(shape: &dyn Shape, _atmosphere: &Atmosphere) -> f32 {
//     match shape.shape_type() {
//         ShapeType::Ball => 1.17,
//         ShapeType::Cuboid => 2.05,
//         _ => 1.0,
//     }
// }
