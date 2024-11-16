//! Forces that act in the vertical axis. All forces assume a positive-up
//! coordinate frame and are reported in Newtons.
#![allow(dead_code)]

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_trait_query::{self, RegisterExt};

use super::{Atmosphere, Density, Mass, Volume};

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WeightForce>();
        app.register_type::<BuoyantForce>();
        app.register_type::<DragForce>();

        app.register_component_as::<dyn Force, WeightForce>();
        app.register_component_as::<dyn Force, BuoyantForce>();
        app.register_component_as::<dyn Force, DragForce>();

        // Disable the default forces since we apply our own.
        app.insert_resource(Gravity(Vec3::ZERO));

        // Update force vectors before solving physics.
        app.configure_sets(
            Update,
            (
                ForceUpdateOrder::First,
                ForceUpdateOrder::Prepare,
                ForceUpdateOrder::Apply,
            )
                .before(PhysicsStepSet::First),
        );
        app.add_systems(
            Update,
            on_rigid_body_added.in_set(ForceUpdateOrder::First),
        );
        app.add_systems(
            Update,
            (update_weight_force, update_buoyant_force, update_drag_force)
                .before(collect_forces)
                .in_set(ForceUpdateOrder::Prepare),
        );
        app.add_systems(
            Update,
            update_total_external_force.in_set(ForceUpdateOrder::Apply),
        );

        // for debugging, let's assume there will always be just one balloon.
        app.init_resource::<NetForce>();
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum ForceUpdateOrder {
    First,
    Prepare,
    Apply,
}

#[derive(Bundle)]
pub struct ForceBundle {
    collection: ForceCollection,
    weight: WeightForce,
    buoyancy: BuoyantForce,
    drag: DragForce,
}

/// Add a `ForceCollection` to entities with a `RigidBody` when they are added.
fn on_rigid_body_added(mut commands: Commands, query: Query<Entity, Added<RigidBody>>) {
    for entity in &query {
        commands.entity(entity).insert((ForceBundle {
            collection: ForceCollection::default(),
            weight: WeightForce::default(),
            buoyancy: BuoyantForce::default(),
            drag: DragForce::default(),
        }));
    }
}

/// This trait is used to identify a force component.
#[bevy_trait_query::queryable]
pub trait Force {
    fn force(&self) -> Vec3 { self.0 }
    fn update(&mut self);
    fn direction(&self) -> Vec3 { self.force().normalize() }
    fn magnitude(&self) -> f32 { self.force().length() }
}

/// Downward force (N) vector due to gravity as a function of altitude (m) and
/// mass (kg). The direction of this force is always world-space down.
#[derive(Component, Default, Reflect)]
pub struct WeightForce(Vec3);
impl Force for WeightForce {
    fn update(&mut self, position: Vec3, mass: f32) {
        self.0 = weight(position, mass);
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

fn update_weight_force(
    mut bodies: Query<(&mut WeightForce, &Position, &Mass), With<RigidBody>>,
) {
    for (mut force, position, mass) in bodies.iter_mut() {
        force.0 = weight(position.0, mass.kg());
    }
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
#[derive(Component, Default, Reflect)]
pub struct BuoyantForce(Vec3);

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up.
pub fn buoyancy(position: Vec3, volume: Volume, ambient_density: Density) -> Vec3 {
    Vec3::Y * (volume.cubic_meters() * ambient_density.kg_per_m3() * g(position))
}

fn update_buoyant_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut BuoyantForce, &Position, &Volume), With<RigidBody>>,
) {
    for (mut force, position, volume) in bodies.iter_mut() {
        let density = atmosphere.density(position.0);
        force.0 = buoyancy(position.0, *volume, density);
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
#[derive(Component, Default, Reflect)]
pub struct DragForce(Vec3);

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, velocity: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    let direction = -velocity.normalize();
    direction * drag_coeff / 2.0 * ambient_density * f32::powf(velocity.length(), 2.0) * drag_area
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn update_drag_force(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut DragForce, &Position, &LinearVelocity), With<RigidBody>>,
) {
    // Todo: update drag force
}

/// Dump all the forces into a single vector that can be queried and summed.
/// TODO: maybe use observer pattern to find and update this collection, or
/// populate it by emitting an event from each force system to populate the
/// array. 
#[derive(Component, Default, Reflect)]
struct ForceCollection {
    forces: Vec<Box<dyn Force>>,
}

#[derive(Resource, Default, Reflect)]
struct NetForce(Vec3);

/// Collect all the forces into the force collection.
fn collect_forces(mut forces: Query<(&mut ForceCollection, With<RigidBody>>) {
    for mut collection in forces.iter_mut() {
        collection.0.push(Vec3::ZERO);
    }
}

/// Set the `ExternalForce` to the sum of all forces in the `Forces` collection.
/// This effectively applies all the calculated force vectors to the physics
/// rigid body without regard to where the forces came from.
fn update_total_external_force(
    mut external_forces: Query<(&mut ExternalForce, &ForceCollection), With<RigidBody>>,
    // TODO: this is a temporary resource that assumes there is only balloon.
    mut net_force: ResMut<NetForce>,
) {
    for (mut physics_force, forces) in external_forces.iter_mut() {
        net_force.0 = forces.0.iter().sum();
        physics_force.set_force(net_force.0);
    }
}
