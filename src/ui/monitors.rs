//! UI for monitoring the simulation.

use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

/// Plugin for the monitor UI.
pub struct MonitorPlugin;

impl Plugin for MonitorPlugin {
    fn build(&self, app: &mut App) {
        app
            // we must register our custom entry type
            .add_perf_ui_simple_entry::<PerfUiTimeSinceLastClick>()
            .init_resource::<TimeSinceLastClick>();
    }
}
