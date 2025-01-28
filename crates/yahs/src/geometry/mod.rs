mod primitives;
mod raycast;
mod volume;

pub use volume::*;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Volume>();
}
