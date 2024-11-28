//! Forces applied to rigid bodies.
pub mod aero;
pub mod body;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_trait_query;

// Re-expert common forces
#[allow(unused_imports)]
pub use aero::Drag;
#[allow(unused_imports)]
pub use body::{Buoyancy, Weight};

use super::{Atmosphere, Balloon, Density, SimulatedBody, SimulationUpdateOrder, SimState, Volume};
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
            on_simulated_body_added.in_set(ForceUpdateOrder::First),
        );
        app.add_systems(
            Update,
            update_total_external_force
                .in_set(ForceUpdateOrder::Apply)
                .run_if(in_state(SimState::Running)),
        );

        app.add_plugins((aero::AeroForcesPlugin, body::BodyForcesPlugin));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForceUpdateOrder {
    First,
    Prepare,
    Apply,
}

/// A bundle of force components to be added to entities with a `RigidBody`. The
/// body can only have one of each type of force component.
#[derive(Bundle, Default)]
pub struct ForceBundle {
    weight: body::Weight,
    buoyancy: body::Buoyancy,
    drag: aero::Drag,
}

fn on_simulated_body_added(mut commands: Commands, query: Query<Entity, Added<SimulatedBody>>) {
    for entity in &query {
        commands.entity(entity).insert(ForceBundle::default());
    }
}

/// This trait is used to identify a force vector component. All forces are
/// collected and summed to determine the net force acting on a rigid body. All
/// forces assume a right-handed Y-up coordinate frame and are reported in
/// Newtons.
#[bevy_trait_query::queryable]
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
    fn point_of_application(&self) -> Vec3 {
        Vec3::ZERO
    }
    fn torque(&self) -> Vec3 {
        Vec3::ZERO
    }
    fn color(&self) -> Option<Color> {
        None
    }
}

/// Set the `ExternalForce` to the sum of all forces in the `Forces` collection.
/// This effectively applies all the calculated force vectors to the physics
/// rigid body without regard to where the forces came from.
///
/// TODO: preserve the position of the total force vector and apply it at that
/// point instead of the center of mass.
fn update_total_external_force(
    mut body_forces: Query<(&mut ExternalForce, &dyn Force, &RigidBody), With<SimulatedBody>>,
) {
    // Iterate over each entity that has force vector components.
    for (mut physics_force_component, acting_forces, rigid_body) in body_forces.iter_mut() {
        // Forces only act on dynamic bodies. Don't bother with other kinds.
        if rigid_body.is_dynamic() {
            let mut net_force = Vec3::ZERO; // reset the net force to zero

            // Iterate over each force vector component and compute its value.
            for force in acting_forces.iter() {
                if force.magnitude().is_nan() {
                    error!("{} has NaN magnitude!", force.name());
                } else {
                    net_force += force.force();
                }
            }
            physics_force_component.set_force(net_force);
        }
    }
}
