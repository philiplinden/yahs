//! Forces applied to rigid bodies due to gravity and buoyancy.

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_trait_query::{self, RegisterExt};

use super::{Atmosphere, Density, Force, ForceUpdateOrder, Mass, Volume};
use crate::simulator::properties::{EARTH_RADIUS_M, STANDARD_G};

pub struct BodyForcesPlugin;

impl Plugin for BodyForcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weight>();
        app.register_type::<Buoyancy>();

        app.register_component_as::<dyn Force, Weight>();
        app.register_component_as::<dyn Force, Buoyancy>();

        app.add_systems(
            Update,
            (update_weight_parameters, update_buoyant_parameters).in_set(ForceUpdateOrder::Prepare),
        );
    }
}

/// Downward force (N) vector due to gravity as a function of altitude (m) and
/// mass (kg). The direction of this force is always world-space down.
#[derive(Component, Reflect)]
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
    fn force(&self) -> Vec3 {
        weight(self.position, self.mass)
    }
    fn point_of_application(&self) -> Vec3 {
        self.position
    }
}

/// Force (N) from gravity at an altitude (m) above mean sea level.
fn g(position: Vec3) -> f32 {
    let altitude = position.y; // [m]
    STANDARD_G * (EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude))
}

/// Downward force (N) vector due to gravity as a function of altitude (m) and
/// mass (kg). The direction of this force is always world-space down.
pub fn weight(position: Vec3, mass: f32) -> Vec3 {
    Vec3::NEG_Y * g(position) * mass // [N]
}

fn update_weight_parameters(mut bodies: Query<(&mut Weight, &Position, &Mass), With<RigidBody>>) {
    for (mut weight, position, mass) in bodies.iter_mut() {
        weight.update(position.0, mass.kg());
    }
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
#[derive(Component, Reflect)]
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
    fn force(&self) -> Vec3 {
        buoyancy(self.position, self.displaced_volume, self.ambient_density)
    }
    fn point_of_application(&self) -> Vec3 {
        self.position
    }
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up (it opposes gravity).
pub fn buoyancy(position: Vec3, displaced_volume: Volume, ambient_density: Density) -> Vec3 {
    Vec3::Y * (displaced_volume.cubic_meters() * ambient_density.kg_per_m3() * g(position))
}

fn update_buoyant_parameters(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Buoyancy, &Position, &Volume), With<RigidBody>>,
) {
    for (mut buoyancy, position, volume) in bodies.iter_mut() {
        let density = atmosphere.density(position.0);
        buoyancy.update(position.0, *volume, density);
    }
}
