//! Forces applied to rigid bodies.
mod aero;
mod buoyancy;
mod weight;

use avian3d::{math::Quaternion, prelude::*};
use bevy::prelude::*;
use std::ops::{Add, AddAssign};

use crate::core::SimState;
use crate::debug;

// Re-export common forces
pub use aero::{drag, DragForce};
pub use buoyancy::{buoyancy, BuoyancyForce};
pub use weight::{gravity, weight, WeightForce};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec3::ZERO));
    app.register_type::<Forces>();
    app.add_systems(
        Update,
        (
            debug::notify_on_added::<WeightForce>,
            debug::notify_on_added::<BuoyancyForce>,
            debug::notify_on_added::<DragForce>,
        ),
    );
    app.add_systems(
        FixedUpdate,
        (
            weight::update_weight_force,
            buoyancy::update_buoyancy_force,
            aero::update_drag_force,
            apply_external_force,
        )
            .chain()
            .in_set(PhysicsSet::Prepare)
            .run_if(in_state(SimState::Running)),
    );
}

/// A collection of force vectors that will be applied to an entity
#[derive(Component, Debug, Default, Clone, Reflect)]
pub struct Forces {
    pub vectors: Vec<ForceVector>,
}

impl Forces {
    pub fn new() -> Self {
        Self {
            vectors: Vec::new(),
        }
    }

    pub fn add(&mut self, force: ForceVector) {
        self.vectors.push(force);
    }

    pub fn clear(&mut self) {
        self.vectors.clear();
    }

    pub fn net_force(&self) -> ForceVector {
        ForceVector {
            name: "Net Force".to_string(),
            force: self.vectors.iter().map(|f| f.force).sum(),
            point: self.vectors.iter().map(|f| f.point).sum(),
            color: None,
            force_type: ForceType::Net,
        }
    }
}

/// Identifies different types of forces for tracking and updating
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ForceType {
    Weight,
    Buoyancy,
    Drag,
    Net,
    Generic,
}

/// A force vector component that will be summed and applied as an external
/// force. All forces are collected and summed to determine the net force acting
/// on a rigid body. All forces assume a right-handed Y-up coordinate frame and
/// are reported in Newtons.
#[derive(Component, Debug, Clone, Reflect)]
pub struct ForceVector {
    pub name: String,
    pub force: Vec3,
    pub point: Vec3,
    pub color: Option<Color>,
    pub force_type: ForceType,
}

impl Default for ForceVector {
    fn default() -> Self {
        Self {
            name: "Force".to_string(),
            force: Vec3::ZERO,
            point: Vec3::ZERO,
            color: None,
            force_type: ForceType::Generic,
        }
    }
}

impl Into<ForceVector> for Vec3 {
    fn into(self) -> ForceVector {
        ForceVector {
            name: "Force".to_string(),
            force: self,
            point: Vec3::ZERO,
            color: None,
            force_type: ForceType::Generic,
        }
    }
}

impl From<ForceVector> for Vec3 {
    fn from(force: ForceVector) -> Self {
        force.force
    }
}

impl Into<ForceVector> for Isometry3d {
    fn into(self) -> ForceVector {
        ForceVector {
            name: "Force".to_string(),
            force: (self.rotation * Vec3::Y).normalize_or_zero(),
            point: self.translation.into(),
            color: None,
            force_type: ForceType::Generic,
        }
    }
}

impl From<ForceVector> for Isometry3d {
    fn from(force: ForceVector) -> Self {
        let rotation = if force.force != Vec3::ZERO {
            Quat::from_rotation_arc(Vec3::Y, force.force.normalize())
        } else {
            Quat::IDENTITY
        };
        Isometry3d::new(force.point, rotation)
    }
}

impl Add for ForceVector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ForceVector {
            name: "Combined Force".to_string(),
            force: self.force + other.force,
            point: self.point + other.point,
            color: None,
            force_type: ForceType::Net,
        }
    }
}

impl AddAssign for ForceVector {
    fn add_assign(&mut self, other: Self) {
        self.force += other.force;
        self.point += other.point;
        self.name = "Combined Force".to_string();
        self.color = None;
        self.force_type = ForceType::Net;
    }
}

/// Sum all forces for an entity and its children and apply the net force as an
/// external force to the rigid body.
///
/// FIXME: I think this double dips when adding forces from children.
fn apply_external_force(
    mut query: Query<(&Forces, &mut ExternalForce, &Children), Without<Parent>>,
    children_forces: Query<&Forces, With<Parent>>,
) {
    for (forces, mut external_force, children) in query.iter_mut() {
        let mut total_forces = forces.net_force();
        for &child in children.iter() {
            if let Ok(child_forces) = children_forces.get(child) {
                total_forces += child_forces.net_force();
            }
        }
        external_force.clear();
        external_force.apply_force(total_forces.force);

    }

}
