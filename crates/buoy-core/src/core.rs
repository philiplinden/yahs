use std::fmt::Display;

use super::*;
use avian3d::prelude::{PhysicsInterpolationPlugin, PhysicsPlugins, PhysicsSet};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};
use uom::si::{f32::*, Quantity};

pub struct BuoyPlugin;

impl Plugin for BuoyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CoreSystemsPlugin,
            CorePhysicsPlugin,
        ));
    }
}

struct CorePhysicsPlugin;

impl Plugin for CorePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
            ideal_gas::plugin,
            atmosphere::plugin,
            forces::plugin,
            grid::plugin,
            time::plugin,
        ));
    }
}

/// The plugin that handles the overall simulation state.
struct CoreSystemsPlugin;

impl Plugin for CoreSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimState>();
        app.add_plugins((
            format::plugin,
            scene::plugin,
        ));
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    Stopped,
    #[default]
    Running,
    Faulted,
}
