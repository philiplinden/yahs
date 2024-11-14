//! Forces that act in the vertical axis. All forces assume a positive-up
//! coordinate frame and are reported in Newtons.
#![allow(dead_code)]

use avian3d::prelude::*;
use bevy::prelude::*;

use super::{Atmosphere, Density, Mass, Volume};

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WeightForce>();
        app.register_type::<BuoyantForce>();

        // Disable the default forces since we apply our own.
        app.insert_resource(Gravity(Vec3::ZERO));

        // Update forces before solving physics.
        app.add_systems(
            Update,
            (
                update_weight_force,
                apply_weight_force_when_spawning,
                update_buoyant_force,
                apply_buoyant_force_when_spawning,
            )
                .in_set(PhysicsStepSet::First),
        );
    }
}

/// All the forces that act on a rigid body.
#[derive(Bundle)]
pub struct ForcesBundle {
    weight: WeightForce,
    buoyancy: BuoyantForce,
}

impl Default for ForcesBundle {
    fn default() -> Self {
        ForcesBundle {
            weight: WeightForce(ExternalForce::new(Vec3::ZERO)),
            buoyancy: BuoyantForce(ExternalForce::new(Vec3::ZERO)),
        }
    }
}

trait Force {
    fn external_force(&self) -> &ExternalForce;
    fn external_force_mut(&mut self) -> &mut ExternalForce;

    /// Same as ExternalForce::force()
    fn force(&self) -> Vec3 {
        self.external_force().force()
    }

    /// Same as ExternalForce::set_force()
    fn set_force(&mut self, force: Vec3) {
        self.external_force_mut().set_force(force);
    }

    /// Same as ExternalForce::apply_force()
    fn apply_force(&mut self, force: Vec3) {
        self.external_force_mut().apply_force(force);
    }

    /// If a force already has a vector, apply it with that same vector.
    fn apply_self(&mut self) {
        self.apply_force(self.force());
    }

    /// Returns the normalized force vector.
    fn normalize(&self) -> Vec3 {
        self.force().normalize()
    }
}

// Implement the trait for any type that has an ExternalForce field
impl<T> Force for T
where
    T: AsRef<ExternalForce> + AsMut<ExternalForce>,
{
    fn external_force(&self) -> &ExternalForce {
        self.as_ref()
    }

    fn external_force_mut(&mut self) -> &mut ExternalForce {
        self.as_mut()
    }
}

#[derive(Component, Reflect)]
pub struct WeightForce(ExternalForce);

impl AsRef<ExternalForce> for WeightForce {
    fn as_ref(&self) -> &ExternalForce {
        &self.0
    }
}

impl AsMut<ExternalForce> for WeightForce {
    fn as_mut(&mut self) -> &mut ExternalForce {
        &mut self.0
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

fn update_weight_force(mut bodies: Query<(&mut WeightForce, &Position, &Mass)>) {
    for (mut force, position, mass) in bodies.iter_mut() {
        force.set_force(weight(position.0, mass.kg()));
    }
}

/// Apply the force vector when it is spawned.
fn apply_weight_force_when_spawning(mut query: Query<&mut WeightForce, Added<WeightForce>>) {
    for mut force in query.iter_mut() {
        force.apply_self();
    }
}

/// Apply the force vector when it is spawned.
fn apply_buoyant_force_when_spawning(mut query: Query<&mut BuoyantForce, Added<BuoyantForce>>) {
    for mut force in query.iter_mut() {
        force.apply_self();
    }
}

#[derive(Component, Reflect)]
pub struct BuoyantForce(ExternalForce);

impl AsRef<ExternalForce> for BuoyantForce {
    fn as_ref(&self) -> &ExternalForce {
        &self.0
    }
}

impl AsMut<ExternalForce> for BuoyantForce {
    fn as_mut(&mut self) -> &mut ExternalForce {
        &mut self.0
    }
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up.
pub fn buoyancy(position: Vec3, volume: Volume, ambient_density: Density) -> Vec3 {
    Vec3::Y * (volume.cubic_meters() * ambient_density.kg_per_m3() * g(position))
}

fn update_buoyant_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut BuoyantForce, &Position, &Volume)>,
) {
    for (mut force, position, volume) in bodies.iter_mut() {
        let density = atmosphere.density(position.0);
        force.set_force(buoyancy(position.0, *volume, density));
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, velocity: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let direction = -velocity.normalize();
    direction * drag_coeff / 2.0 * ambient_density * f32::powf(velocity.length(), 2.0) * drag_area
}

// pub fn step() {

//     let total_dry_mass = body.total_mass() + parachute.total_mass();
//     let weight_force = forces::weight(altitude, total_dry_mass);
//     let buoyancy_force = forces::buoyancy(altitude, atmosphere, balloon.lift_gas);

//     let total_drag_force = forces::drag(atmosphere, ascent_rate, balloon)
//         + forces::drag(atmosphere, ascent_rate, body)
//         + forces::drag(atmosphere, ascent_rate, parachute.main)
//         + forces::drag(atmosphere, ascent_rate, parachute.drogue);
//     debug!(
//         "weight: {:?} buoyancy: {:?} drag: {:?}",
//         weight_force, buoyancy_force, total_drag_force
//     );

//     // calculate the net force
//     let net_force = weight_force + buoyancy_force + total_drag_force;
//     let acceleration = net_force / total_dry_mass;
//     let ascent_rate = ascent_rate + acceleration * delta_t;
//     let altitude = altitude + ascent_rate * delta_t;
// }
