//! This module is temporary. It sets up a basic scene with a fluid volume and
//! some other objects to test simulation features.
//!
//! It should be replaced with a system that allows a scene to be loaded from a
//! config file or spawned at runtime.

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_scene);
}

fn setup_scene(mut commands: Commands) {
    
}
