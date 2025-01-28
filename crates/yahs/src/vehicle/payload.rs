//! The things that are carried by the balloon.
#![allow(dead_code)]
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::debug;
use crate::forces::Forces;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<Payload>();
    app.register_type::<Tether>();
    app.add_systems(Update, debug::notify_on_added::<Payload>);
    // app.add_systems(Startup, spawn_payload);
}

/// A thing carried by the balloon.
#[derive(Component, Default, Reflect)]
#[require(Transform, RigidBody(|| RigidBody::Dynamic), Forces)]
pub struct Payload;

/// A tether that connects the balloon to the payload.
#[derive(Component, Default, Reflect)]
pub struct Tether;
