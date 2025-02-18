pub mod space;
pub mod time;
pub mod forces;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        space::plugin,
        time::plugin,
        forces::plugin,
    ));
}
