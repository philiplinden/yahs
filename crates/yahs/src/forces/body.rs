//! Forces applied to rigid bodies due to gravity and buoyancy.

use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{
    gas::Atmosphere,
    vehicle::Balloon,
    forces::{Density, Force, Mass},
    thermodynamics::{EARTH_RADIUS_M, STANDARD_G},
    geometry::Volume,
};

pub struct BodyForcesPlugin;

impl Plugin for BodyForcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weight>();
        app.register_type::<Buoyancy>();

        app.add_systems(
            FixedUpdate,
            (update_weight_parameters, update_buoyant_parameters),
        );
    }
}

/// Downward force (N) vector due to gravity as a function of altitude (m) and
/// mass (kg). The direction of this force is always world-space down.
#[derive(Component, Reflect, Debug)]
#[require(Mass, Position)]
pub struct Weight {
    position: Vec3,
    mass: f32,
}
impl Default for Weight {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            mass: 0.0,
        }
    }
}
impl Weight {
    pub fn update(&mut self, position: Vec3, mass: f32) {
        self.position = position;
        self.mass = mass;
    }
}
impl Force for Weight {
    fn name(&self) -> String {
        String::from("Weight")
    }
    fn force(&self) -> Vec3 {
        weight(self.position, self.mass)
    }
    fn point_of_application(&self) -> Vec3 {
        self.position
    }
    fn color(&self) -> Option<Color> {
        Some(Color::srgb(0.0, 1.0, 0.0))
    }
}

/// Force (N) from gravity at an altitude (m) above mean sea level.
fn gravity(position: Vec3) -> Vec3 {
    let altitude = position.y; // [m]
    Vec3::NEG_Y * STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude))
}

/// Downward force (N) vector due to gravity as a function of altitude (m) and
/// mass (kg). The direction of this force is always world-space down.
pub fn weight(position: Vec3, mass: f32) -> Vec3 {
    gravity(position) * mass // [N]
}

fn update_weight_parameters(mut bodies: Query<(&mut Weight, &Position, &Mass), With<Balloon>>) {
    for (mut weight, position, mass) in bodies.iter_mut() {
        weight.update(position.0, mass.0);
        debug!("Updating Weight: Position: {:?}, Mass: {:?}", position.0, mass.0);
    }
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
#[derive(Component, Reflect, Debug)]
#[require(Volume, Position)]
pub struct Buoyancy {
    position: Vec3,
    displaced_volume: Volume,
    ambient_density: Density,
}
impl Default for Buoyancy {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            displaced_volume: Volume::ZERO,
            ambient_density: Density::ZERO,
        }
    }
}
impl Buoyancy {
    pub fn update(&mut self, position: Vec3, displaced_volume: Volume, ambient_density: Density) {
        self.position = position;
        self.displaced_volume = displaced_volume;
        self.ambient_density = ambient_density;
    }
}
impl Force for Buoyancy {
    fn name(&self) -> String {
        String::from("Buoyancy")
    }
    fn force(&self) -> Vec3 {
        buoyancy(self.position, self.displaced_volume, self.ambient_density)
    }
    fn point_of_application(&self) -> Vec3 {
        self.position
    }
    fn color(&self) -> Option<Color> {
        Some(Color::srgb(0.0, 0.0, 1.0))
    }
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up (it opposes gravity).
pub fn buoyancy(position: Vec3, displaced_volume: Volume, ambient_density: Density) -> Vec3 {
    Vec3::Y * (displaced_volume.m3() * ambient_density.kg_per_m3() * gravity(position).length())
}

fn update_buoyant_parameters(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Buoyancy, &Position, &Balloon)>,
) {
    for (mut buoyancy, position, balloon) in bodies.iter_mut() {
        let ambient_density = atmosphere.density(position.0);
        let displaced_volume = Volume(balloon.shape.volume());
        buoyancy.update(position.0, displaced_volume, ambient_density);
    }
}
