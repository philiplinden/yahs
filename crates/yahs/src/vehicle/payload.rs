//! The things that are carried by the balloon.
#![allow(dead_code)]
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::debug;
use crate::forces::{DragForce, Forces, WeightForce};

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<Payload>();
    app.add_systems(
        Update, debug::notify_on_added::<Payload>,
    );
}

/// A thing carried by the balloon.
#[derive(Component, Default, Reflect)]
#[require(Transform, RigidBody(|| RigidBody::Dynamic), Forces, WeightForce)]
pub struct Payload {
    pub shape: Cuboid,
}

#[derive(Bundle)]
pub struct PayloadBundle {
    name: Name,
    payload: Payload,
    mass: Mass,
    transform: Transform,
}

impl PayloadBundle {
    pub fn new(shape: Cuboid, mass: Mass, transform: Transform) -> Self {
        PayloadBundle {
            name: Name::new("Payload"),
            payload: Payload { shape },
            mass,
            transform,
        }
    }
}
