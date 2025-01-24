mod console;
mod fps;

use bevy::prelude::*;

pub struct DevToolsPlugins;

impl Plugin for DevToolsPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            fps::plugin,
            console::plugin,
        ));
    }
}
