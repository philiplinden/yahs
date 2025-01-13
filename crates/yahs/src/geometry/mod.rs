mod primitives;
mod raycast;
mod volume;

pub use volume::Volume;

use bevy::prelude::*;

pub struct GeometryToolsPlugin;

impl Plugin for GeometryToolsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Volume>();
    }
}
