//! The things that are carried by the balloon.
#![allow(dead_code)]

use bevy::prelude::*;

pub struct PayloadPlugin;

impl Plugin for PayloadPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_payload);
    }
}

/// A thing carried by the balloon.
#[derive(Component)]
pub struct Payload;


/// A tether that connects the balloon to the payload.
#[derive(Component)]
pub struct Tether;
