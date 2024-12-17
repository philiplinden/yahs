//! The things that are carried by the balloon.
#![allow(dead_code)]
use avian3d::prelude::*;
use bevy::prelude::*;

use super::forces::{Drag, Force, Weight};

pub struct PayloadPlugin;

impl Plugin for PayloadPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Startup, spawn_payload);
    }
}

/// A thing carried by the balloon.
#[derive(Component, Default)]
#[require(Transform, RigidBody(|| RigidBody::Dynamic), Weight, Drag)]
pub struct Payload;

/// A tether that connects the balloon to the payload.
#[derive(Component, Default)]
pub struct Tether;
