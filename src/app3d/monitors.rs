//! UI for monitoring the simulation.
#![allow(unused_imports)]
use bevy::{
    ecs::system::{
        lifetimeless::{SQuery, SRes},
        SystemParam,
    },
    prelude::*,
};
use iyes_perf_ui::{entry::PerfUiEntry, prelude::*, utils::format_pretty_float};

use crate::simulator::{
    balloon::Balloon,
    forces::{Buoyancy, Drag, Force, Weight},
    ideal_gas::IdealGas,
    SimState,
};

pub struct MonitorsPlugin;

impl Plugin for MonitorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PerfUiPlugin);
        app.add_perf_ui_simple_entry::<SimStateMonitor>();
        app.add_perf_ui_simple_entry::<ForceMonitor>();
        app.add_perf_ui_simple_entry::<GasMonitor>();
        app.add_systems(Startup, spawn_monitors);
    }
}

fn spawn_monitors(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot {
            position: PerfUiPosition::BottomRight,
            ..default()
        },
        SimStateMonitor::default(),
        ForceMonitor::default(),
        GasMonitor::default(),
    ));
}

#[derive(Component)]
struct SimStateMonitor {
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

impl Default for SimStateMonitor {
    fn default() -> Self {
        SimStateMonitor {
            label: String::new(),
            display_units: false,
            threshold_highlight: Some(10.0),
            color_gradient: ColorGradient::new_preset_gyr(0.0, 5.0, 10.0).unwrap(),
            digits: 7,
            precision: 0,
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for SimStateMonitor {
    type Value = SimState;
    type SystemParam = SRes<State<SimState>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Simulation State"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn format_value(&self, value: &Self::Value) -> String {
        match value {
            SimState::Running => String::from("Running"),
            SimState::Stopped => String::from("Stopped"),
            _ => String::from("Unknown"),
        }
    }

    fn update_value(
        &self,
        state: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(*state.as_ref().get())
    }

    // (optional) We should add a width hint, so that the displayed
    // strings in the UI can be correctly aligned.
    // This value represents the largest length the formatted string
    // is expected to have.
    fn width_hint(&self) -> usize {
        // there is a helper we can use, since we use `format_pretty_float`
        let w = iyes_perf_ui::utils::width_hint_pretty_float(self.digits, self.precision);
        if self.display_units {
            w + 2
        } else {
            w
        }
    }

    // (optional) Called every frame to determine if a custom color should be used for the value
    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        match *value {
            SimState::Running => self.color_gradient.get_color_for_value(0.0),
            SimState::Stopped => self.color_gradient.get_color_for_value(5.0),
            _ => self.color_gradient.get_color_for_value(10.0),
        }
    }

    // (optional) Called every frame to determine if the value should be highlighted
    fn value_highlight(&self, value: &Self::Value) -> bool {
        self.threshold_highlight
            .map(|_| value == &SimState::Stopped)
            .unwrap_or(false)
    }
}

#[derive(Component)]
struct ForceMonitor {
    /// The label text to display, to allow customization
    pub label: String,
    /// Should we display units?
    pub display_units: bool,
    /// Highlight the value if it goes above this threshold
    #[allow(dead_code)]
    pub threshold_highlight: Option<f32>,
    /// Support color gradients!
    #[allow(dead_code)]
    pub color_gradient: ColorGradient,
    /// Width for formatting the string
    pub digits: u8,
    /// Precision for formatting the string
    pub precision: u8,
    /// Required to ensure the entry appears in the correct place in the Perf UI
    pub sort_key: i32,
}

impl Default for ForceMonitor {
    fn default() -> Self {
        ForceMonitor {
            label: String::new(),
            display_units: true,
            threshold_highlight: Some(10.0),
            color_gradient: ColorGradient::new_preset_gyr(0.0, 10.0, 100.0).unwrap(),
            digits: 5,
            precision: 3,
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for ForceMonitor {
    type Value = (f32, f32, f32);
    type SystemParam = SQuery<(&'static Weight, &'static Buoyancy, &'static Drag), With<Balloon>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Forces"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn format_value(&self, value: &Self::Value) -> String {
        // we can use a premade helper function for nice-looking formatting
        let mut f_g = format_pretty_float(self.digits, self.precision, value.0 as f64);
        let mut f_b = format_pretty_float(self.digits, self.precision, value.1 as f64);
        let mut f_d = format_pretty_float(self.digits, self.precision, value.2 as f64);
        // (and append units to it)
        if self.display_units {
            f_g.push_str(" N");
            f_b.push_str(" N");
            f_d.push_str(" N");
        }
        format!("Gravity:  {}\nBuoyancy: {}\nDrag:     {}", f_g, f_b, f_d)
    }

    fn update_value(
        &self,
        force_resources: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        for (weight, buoyancy, drag) in force_resources.iter() {
            return Some((weight.force().y, buoyancy.force().y, drag.force().y));
        }
        None
    }

    // (optional) We should add a width hint, so that the displayed
    // strings in the UI can be correctly aligned.
    // This value represents the largest length the formatted string
    // is expected to have.
    fn width_hint(&self) -> usize {
        // there is a helper we can use, since we use `format_pretty_float`
        let w = iyes_perf_ui::utils::width_hint_pretty_float(self.digits, self.precision);
        if self.display_units {
            w + 2
        } else {
            w
        }
    }
}

#[derive(Component)]
struct GasMonitor {
    /// The label text to display, to allow customization
    pub label: String,
    /// Should we display units?
    pub display_units: bool,
    /// Highlight the value if it goes above this threshold
    #[allow(dead_code)]
    pub threshold_highlight: Option<f32>,
    /// Support color gradients!
    #[allow(dead_code)]
    pub color_gradient: ColorGradient,
    /// Width for formatting the string
    pub digits: u8,
    /// Precision for formatting the string
    pub precision: u8,
    /// Required to ensure the entry appears in the correct place in the Perf UI
    pub sort_key: i32,
}

impl Default for GasMonitor {
    fn default() -> Self {
        GasMonitor {
            label: String::new(),
            display_units: true,
            threshold_highlight: Some(10.0),
            color_gradient: ColorGradient::new_preset_gyr(0.0, 10.0, 100.0).unwrap(),
            digits: 5,
            precision: 2,
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for GasMonitor {
    type Value = (f32, f32, f32, f32, f32, String);
    type SystemParam = SQuery<(&'static Balloon, &'static IdealGas)>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Gas"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn format_value(&self, value: &Self::Value) -> String {
        // we can use a premade helper function for nice-looking formatting
        let mut volume = format_pretty_float(self.digits, self.precision, value.0 as f64);
        let mut pressure = format_pretty_float(self.digits, self.precision, value.1 as f64);
        let mut temperature = format_pretty_float(self.digits, self.precision, value.2 as f64);
        let mut density = format_pretty_float(self.digits, self.precision, value.3 as f64);
        let mut mass = format_pretty_float(self.digits, self.precision, value.4 as f64);
        let mut species = value.5.clone();

        // (and append units to it)
        if self.display_units {
            volume.push_str(" m3");
            pressure.push_str(" kPa");
            temperature.push_str(" K");
            density.push_str(" kg/m3");
            mass.push_str(" kg");
            species.push_str("");
        }
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            species, volume, pressure, temperature, density, mass
        )
    }

    fn update_value(
        &self,
        items: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let (balloon, gas) = items.get_single().unwrap();
        Some((
            balloon.shape.volume(),
            gas.pressure.kilopascals(),
            gas.temperature.kelvin(),
            gas.density.kilograms_per_cubic_meter(),
            gas.mass.value(),
            gas.species.name.clone(),
        ))
    }

    // (optional) We should add a width hint, so that the displayed
    // strings in the UI can be correctly aligned.
    // This value represents the largest length the formatted string
    // is expected to have.
    fn width_hint(&self) -> usize {
        // there is a helper we can use, since we use `format_pretty_float`
        let w = iyes_perf_ui::utils::width_hint_pretty_float(self.digits, self.precision);
        if self.display_units {
            w + 5
        } else {
            w
        }
    }
}
