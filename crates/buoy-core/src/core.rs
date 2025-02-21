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
            CorePhysicsPlugin,
            SimStatePlugin,
            FormattingPlugin,
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
            space::plugin,
            time::plugin,
        ));
    }
}

/// The plugin that handles the overall simulation state.
struct SimStatePlugin;

impl Plugin for SimStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimState>();
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    Stopped,
    #[default]
    Running,
    Faulted,
}

struct FormattingPlugin;

impl Plugin for FormattingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UomQuantity>();
    }
}

#[derive(Component, Debug, Reflect)]
pub struct UomQuantity {
    value: f32,
    unit: String,
}

impl UomQuantity {
    pub fn new<D, U, V>(quantity: &Quantity<D, U, V>) -> Self
    where
        D: uom::si::Dimension + ?Sized,
        U: uom::si::Units<V> + ?Sized + uom::si::Unit,
        V: uom::num::Num + uom::Conversion<V> + Into<f32> + Clone,
    {
        Self {
            value: quantity.value.clone().into(),
            unit: U::abbreviation().to_string(),
        }
    }
}

impl Display for UomQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}
