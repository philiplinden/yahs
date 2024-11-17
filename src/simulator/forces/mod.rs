pub mod aero;
pub mod body;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_trait_query::{self, RegisterExt};

use super::{Atmosphere, Density, Mass, Volume};

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
                .before(PhysicsStepSet::First),
        );
        app.add_systems(Update, on_rigid_body_added.in_set(ForceUpdateOrder::First));
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
    weight: body::Weight,
    buoyancy: body::Buoyancy,
    drag: aero::Drag,
}

/// Add a `ForceCollection` to entities with a `RigidBody` when they are added.
fn on_rigid_body_added(mut commands: Commands, query: Query<Entity, Added<RigidBody>>) {
    for entity in &query {
        commands.entity(entity).insert(ForceBundle {
            weight: body::Weight::default(),
            buoyancy: body::Buoyancy::default(),
            drag: aero::Drag::default(),
        });
    }
}

/// This trait is used to identify a force vector component. All forces are
/// collected and summed to determine the net force acting on a rigid body. All
/// forces assume a right-handed Y-up coordinate frame and are reported in
/// Newtons.
#[bevy_trait_query::queryable]
pub trait Force {
    fn force(&self) -> Vec3;
    fn direction(&self) -> Vec3 {
        self.force().normalize()
    }
    fn magnitude(&self) -> f32 {
        self.force().length()
    }
    fn point_of_application(&self) -> Vec3;
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
