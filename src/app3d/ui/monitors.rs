//! UI for monitoring the simulation.

use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct ForceMonitorPlugin   ;

impl Plugin for ForceMonitorPlugin {
    fn build(&self, app: &mut App) {
        app.add_perf_ui_simple_entry::<WeightForceMonitor>();
        app.add_perf_ui_simple_entry::<BuoyancyForceMonitor>();
        app.add_perf_ui_simple_entry::<DragForceMonitor>();
        app.add_perf_ui_simple_entry::<NetForceMonitor>();
    }
}

// TODO: make a generic force monitor that takes a force trait and displays the
// force vector so we don't have to make a new monitor for each force type and
// redo all that boilerplate code.

#[derive(Component)]
struct WeightForceMonitor {
    /// The label text to display, to allow customization
    pub label: String,
    /// Should we display units?
    pub display_units: bool,
    /// Highlight the value if it goes above this threshold
    pub threshold_highlight: Option<f32>,
    /// Support color gradients!
    pub color_gradient: ColorGradient,
    /// Width for formatting the string
    pub digits: u8,
    /// Precision for formatting the string
    pub precision: u8,

    /// Required to ensure the entry appears in the correct place in the Perf UI
    pub sort_key: i32,
}

#[derive(Component)]
struct BuoyancyForceMonitor {
    /// The label text to display, to allow customization
    pub label: String,
    /// Should we display units?
    pub display_units: bool,
    /// Highlight the value if it goes above this threshold
    pub threshold_highlight: Option<f32>,
    /// Support color gradients!
    pub color_gradient: ColorGradient,
    /// Width for formatting the string
    pub digits: u8,
    /// Precision for formatting the string
    pub precision: u8,

    /// Required to ensure the entry appears in the correct place in the Perf UI
    pub sort_key: i32,
}

#[derive(Component)]
struct DragForceMonitor {
    /// The label text to display, to allow customization
    pub label: String,
    /// Should we display units?
    pub display_units: bool,
    /// Highlight the value if it goes above this threshold
    pub threshold_highlight: Option<f32>,
    /// Support color gradients!
    pub color_gradient: ColorGradient,
    /// Width for formatting the string
    pub digits: u8,
    /// Precision for formatting the string
    pub precision: u8,

    /// Required to ensure the entry appears in the correct place in the Perf UI
    pub sort_key: i32,
}

#[derive(Component)]
struct NetForceMonitor {
    /// The label text to display, to allow customization
    pub label: String,
    /// Should we display units?
    pub display_units: bool,
    /// Highlight the value if it goes above this threshold
    pub threshold_highlight: Option<f32>,
    /// Support color gradients!
    pub color_gradient: ColorGradient,
    /// Width for formatting the string
    pub digits: u8,
    /// Precision for formatting the string
    pub precision: u8,

    /// Required to ensure the entry appears in the correct place in the Perf UI
    pub sort_key: i32,
}
