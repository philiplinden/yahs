use bevy::prelude::*;
use avian3d::debug_render::PhysicsDebugPlugin;
use big_space::prelude::FloatingOriginDebugPlugin;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsDebugPlugin::default(), // Draws colliders
        FloatingOriginDebugPlugin::<i64>::default(), // Draws cell AABBs and grids
    ));
}
