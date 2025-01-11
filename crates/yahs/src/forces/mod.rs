//! Forces applied to rigid bodies.
pub mod aero;
pub mod body;

use avian3d::prelude::*;
use bevy::prelude::*;

// Re-export common forces
#[allow(unused_imports)]
pub use aero::Drag;
#[allow(unused_imports)]
pub use body::{Buoyancy, Weight};

use crate::{
    atmosphere::Atmosphere,
    balloon::Balloon,
    core::{SimState, SimulationUpdateOrder},
    properties::{Density, Volume},
};
pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
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
                .chain()
                .in_set(SimulationUpdateOrder::Forces),
        );
        app.add_systems(
            Update,
            (
                add_external_forces_to_new_bodies.in_set(ForceUpdateOrder::First),
                update_total_external_force
                    .in_set(ForceUpdateOrder::Apply)
                    .run_if(in_state(SimState::Running)),
            ),
        );

        app.add_plugins((aero::AeroForcesPlugin, body::BodyForcesPlugin));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForceUpdateOrder {
    First,
    Prepare,
    Apply,
    #[allow(dead_code)]
    Last,
}

/// This trait is used to identify a force vector component. All forces are
/// collected and summed to determine the net force acting on a rigid body. All
/// forces assume a right-handed Y-up coordinate frame and are reported in
/// Newtons.
pub trait Force {
    fn name(&self) -> String {
        String::from("Force")
    }
    fn force(&self) -> Vec3;
    fn direction(&self) -> Vec3 {
        self.force().normalize()
    }
    fn magnitude(&self) -> f32 {
        self.force().length()
    }
    fn point_of_application(&self) -> Vec3;
    fn torque(&self) -> Vec3 {
        Vec3::ZERO
    }
    fn color(&self) -> Option<Color> {
        None
    }
}

/// Whenever a balloon is added to the scene, add the external force component
/// to it. This component is used to apply the forces to the physics rigid body.
fn add_external_forces_to_new_bodies(
    mut commands: Commands,
    entities: Query<Entity, Added<Balloon>>,
) {
    for entity in entities.iter() {
        let physics_force_component = ExternalForce::new(Vec3::ZERO).with_persistence(false);
        commands
            .entity(entity)
            .insert((RigidBody::Dynamic, physics_force_component));
    }
}

/// Collect all the force components and sum them up to get the total force
/// acting on the rigid body.
fn update_total_external_force(
    mut body_forces: Query<(&mut ExternalForce, &Weight, &Buoyancy, &Drag), With<Balloon>>,
) {
    // Iterate over each entity that has force vector components.
    for (mut physics_force_component, weight, buoyancy, drag) in body_forces.iter_mut() {
        let mut net_force = Vec3::ZERO;

        net_force += weight.force();
        net_force += buoyancy.force();
        net_force += drag.force();

        physics_force_component.apply_force(net_force);
    }
}
