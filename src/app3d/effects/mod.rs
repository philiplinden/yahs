//! This is the base module for rendering the oscilloscope display.

mod crt;

use bevy::prelude::*;
use crt::CrtPlugin;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CrtPlugin);
    }
}
