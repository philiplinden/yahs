//! Forces that act in the vertical axis. All forces assume a positive-up
//! coordinate frame and are reported in Newtons.
#![allow(dead_code)]

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_trait_query::{self, RegisterExt};

use super::{aero::aero_drag_from_collider, Atmosphere, Density, Mass, Volume};

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weight>();
        app.register_type::<Buoyancy>();
        app.register_type::<Drag>();

        app.register_component_as::<dyn Force, Weight>();
        app.register_component_as::<dyn Force, Buoyancy>();
        app.register_component_as::<dyn Force, Drag>();

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
        app.add_systems(Update, on_rigid_body_added.in_set(ForceUpdateOrder::First));
        app.add_systems(
            Update,
            (
                update_weight_parameters,
                update_buoyant_parameters,
                update_drag_parameters,
            )
                .in_set(ForceUpdateOrder::Prepare),
        );
        app.add_systems(
            Update,
            update_total_external_force.in_set(ForceUpdateOrder::Apply),
        );

        // for debugging, let's assume there will always be just one balloon.
        // app.init_resource::<NetForce>();
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
    weight: Weight,
    buoyancy: Buoyancy,
    drag: Drag,
}

/// Add a `ForceCollection` to entities with a `RigidBody` when they are added.
fn on_rigid_body_added(mut commands: Commands, query: Query<Entity, Added<RigidBody>>) {
    for entity in &query {
        commands.entity(entity).insert(ForceBundle {
            weight: Weight::default(),
            buoyancy: Buoyancy::default(),
            drag: Drag::default(),
        });
    }
}

/// This trait is used to identify a force vector component. All forces are
/// collected and summed to determine the net force acting on a rigid body.
#[bevy_trait_query::queryable]
pub trait Force {
    fn force(&self) -> Vec3;
    fn direction(&self) -> Vec3 {
        self.force().normalize()
    }
    fn magnitude(&self) -> f32 {
        self.force().length()
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

/// Force (N) due to drag as a solid body moves through a fluid.
#[derive(Component, Reflect)]
pub struct Drag {
    local_gravity: f32,
    drag_normal: Vec3,
    ambient_density: Density,
    drag_area: f32,
    drag_coeff: f32,
}
impl Default for Drag {
    fn default() -> Self {
        Self {
            local_gravity: 0.0,
            drag_normal: Vec3::ZERO,
            ambient_density: Density::ZERO,
            drag_area: 0.0,
            drag_coeff: 0.0,
        }
    }
}
impl Drag {
    pub fn update(
        &mut self,
        local_gravity: f32,
        drag_normal: Vec3,
        ambient_density: Density,
        drag_area: f32,
        drag_coeff: f32,
    ) {
        self.local_gravity = local_gravity;
        self.ambient_density = ambient_density;
        self.drag_normal = drag_normal;
        self.drag_area = drag_area;
        self.drag_coeff = drag_coeff;
    }
}
impl Force for Drag {
    fn force(&self) -> Vec3 {
        drag(
            self.ambient_density.kg_per_m3(),
            self.drag_normal,
            self.drag_area,
            self.drag_coeff,
        )
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(ambient_density: f32, drag_normal: Vec3, drag_area: f32, drag_coeff: f32) -> Vec3 {
    drag_normal * drag_coeff / 2.0
        * ambient_density
        * drag_normal.length_squared()
        * drag_area
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn update_drag_parameters(
    atmosphere: Res<Atmosphere>,
    mut bodies: Query<(&mut Drag, &Position, &LinearVelocity, &Collider), With<RigidBody>>,
) {
    for (mut drag, position, velocity, collider) in bodies.iter_mut() {
        let density = atmosphere.density(position.0);
        let local_gravity = g(position.0);
        
        let wind_velocity = Vec3::ZERO; // Can be updated with actual wind
        let drag_force = aero_drag_from_collider(collider, *velocity, wind_velocity);
        
        // Update drag component with normalized force direction
        drag.update(
            local_gravity,
            if drag_force == Vec3::ZERO { Vec3::ZERO } else { -drag_force.normalize() },
            density,
            1.0, // Area handled in aero_drag_from_collider
            1.0, // Coefficient handled in aero_drag_from_collider
        );
    }
}

/// Set the `ExternalForce` to the sum of all forces in the `Forces` collection.
/// This effectively applies all the calculated force vectors to the physics
/// rigid body without regard to where the forces came from.
fn update_total_external_force(
    mut body_forces: Query<(&mut ExternalForce, &dyn Force), With<RigidBody>>,
) {
    // Iterate over each entity that has force vector components.
    for (mut physics_force_component, acting_forces) in body_forces.iter_mut() {
        let mut net_force = Vec3::ZERO; // reset the net force to zero
                                        // Iterate over each force vector component and compute its value.
        for force in acting_forces {
            net_force += force.force();
        }
        physics_force_component.set_force(net_force);
    }
}
