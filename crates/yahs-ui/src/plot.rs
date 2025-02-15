use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_egui::{
    egui,
    EguiContexts,
};
use egui_plot::{
    Line,
    Plot,
    PlotPoints,
};
use std::collections::VecDeque;
use yahs::prelude::{Balloon, Forces, ForceType, SimState};
use std::collections::HashMap;

const MAX_HISTORY: usize = 1000; // Store last 1000 data points

pub struct PlotPlugin;

impl Plugin for PlotPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<KinematicsPlotData>()
            .init_resource::<ForcesPlotData>()
            .init_resource::<GasPropertiesPlotData>()
            .init_resource::<PlotVisibility>();
    }
}

trait PlotDataCollection {
    fn push_time_value(&mut self, time: f64, value: f64, queue: &mut VecDeque<f64>);
    fn get_plot_points(&self, time: &VecDeque<f64>, values: &VecDeque<f64>) -> PlotPoints;
    fn maintain_size(&mut self, queue: &mut VecDeque<f64>);
}

impl PlotDataCollection for VecDeque<f64> {
    fn push_time_value(&mut self, time: f64, value: f64, queue: &mut VecDeque<f64>) {
        if self.len() >= MAX_HISTORY {
            self.pop_front();
            queue.pop_front();
        }
        self.push_back(time);
        queue.push_back(value);
    }

    fn get_plot_points(&self, time: &VecDeque<f64>, values: &VecDeque<f64>) -> PlotPoints {
        time.iter()
            .zip(values.iter())
            .map(|(&t, &v)| [t, v])
            .collect()
    }

    fn maintain_size(&mut self, queue: &mut VecDeque<f64>) {
        if self.len() >= MAX_HISTORY {
            self.pop_front();
            queue.pop_front();
        }
    }
}

#[derive(Resource)]
pub struct KinematicsPlotData {
    time: VecDeque<f64>,
    altitude: VecDeque<f64>,
    velocity: VecDeque<f64>,
}

#[derive(Resource)]
pub struct ForcesPlotData {
    time: VecDeque<f64>,
    net_force: VecDeque<f64>,
    buoyancy: VecDeque<f64>,
    drag: VecDeque<f64>,
    weight: VecDeque<f64>,
}

#[derive(Resource)]
pub struct GasPropertiesPlotData {
    time: VecDeque<f64>,
    volume: VecDeque<f64>,
    temperature: VecDeque<f64>,
    pressure: VecDeque<f64>,
}

impl Default for KinematicsPlotData {
    fn default() -> Self {
        Self {
            time: VecDeque::with_capacity(MAX_HISTORY),
            altitude: VecDeque::with_capacity(MAX_HISTORY),
            velocity: VecDeque::with_capacity(MAX_HISTORY),
        }
    }
}

impl Default for ForcesPlotData {
    fn default() -> Self {
        Self {
            time: VecDeque::with_capacity(MAX_HISTORY),
            net_force: VecDeque::with_capacity(MAX_HISTORY),
            buoyancy: VecDeque::with_capacity(MAX_HISTORY),
            drag: VecDeque::with_capacity(MAX_HISTORY),
            weight: VecDeque::with_capacity(MAX_HISTORY),
        }
    }
}

impl Default for GasPropertiesPlotData {
    fn default() -> Self {
        Self {
            time: VecDeque::with_capacity(MAX_HISTORY),
            volume: VecDeque::with_capacity(MAX_HISTORY),
            temperature: VecDeque::with_capacity(MAX_HISTORY),
            pressure: VecDeque::with_capacity(MAX_HISTORY),
        }
    }
}

impl KinematicsPlotData {
    fn push_data(&mut self, time: f64, altitude: f64, velocity: f64) {
        self.time.maintain_size(&mut self.altitude);
        self.time.maintain_size(&mut self.velocity);
        
        self.time.push_back(time);
        self.altitude.push_back(altitude);
        self.velocity.push_back(velocity);
    }

    fn get_plot_points(&self, values: &VecDeque<f64>) -> PlotPoints {
        self.time.get_plot_points(&self.time, values)
    }
}

impl ForcesPlotData {
    fn push_data(&mut self, time: f64, forces: &Forces) {
        self.time.maintain_size(&mut self.net_force);
        self.time.maintain_size(&mut self.buoyancy);
        self.time.maintain_size(&mut self.drag);
        self.time.maintain_size(&mut self.weight);

        self.time.push_back(time);
        
        // Net force is special - it's calculated from all forces
        self.net_force.push_back(forces.net_force().force.y as f64);
        
        // Find individual forces by their type
        for force in &forces.vectors {
            let value = force.force.y as f64;
            match force.force_type {
                ForceType::Buoyancy => self.buoyancy.push_back(value),
                ForceType::Drag => self.drag.push_back(value),
                ForceType::Weight => self.weight.push_back(value),
                _ => {}
            }
        }
    }

    fn get_plot_points(&self, values: &VecDeque<f64>) -> PlotPoints {
        self.time.get_plot_points(&self.time, values)
    }
}

impl GasPropertiesPlotData {
    fn push_data(&mut self, time: f64, volume: f64, temperature: f64, pressure: f64) {
        self.time.maintain_size(&mut self.volume);
        self.time.maintain_size(&mut self.temperature);
        self.time.maintain_size(&mut self.pressure);
        
        self.time.push_back(time);
        self.volume.push_back(volume);
        self.temperature.push_back(temperature);
        self.pressure.push_back(pressure);
    }

    fn get_plot_points(&self, values: &VecDeque<f64>) -> PlotPoints {
        self.time.get_plot_points(&self.time, values)
    }
}

// Modify PlotValue to include all plot types we want to show
#[derive(Clone)]
enum PlotValue {
    Position,
    Velocity,
    Forces,
    Volume,
    Temperature,
    Pressure,
}

// Add this struct to store plot metadata
struct PlotMetadata {
    name: &'static str,
    unit: &'static str,
    id: &'static str,
}

impl PlotValue {
    fn all() -> Vec<PlotValue> {
        vec![
            PlotValue::Position,
            PlotValue::Velocity,
            PlotValue::Forces,
            PlotValue::Volume,
            PlotValue::Temperature,
            PlotValue::Pressure,
        ]
    }

    fn metadata(&self) -> PlotMetadata {
        match self {
            PlotValue::Position => PlotMetadata {
                name: "Position",
                unit: "m",
                id: "position_plot",
            },
            PlotValue::Velocity => PlotMetadata {
                name: "Velocity",
                unit: "m/s",
                id: "velocity_plot",
            },
            PlotValue::Forces => PlotMetadata {
                name: "Forces",
                unit: "N",
                id: "force_plots",
            },
            PlotValue::Volume => PlotMetadata {
                name: "Volume",
                unit: "m³",
                id: "volume_plot",
            },
            PlotValue::Temperature => PlotMetadata {
                name: "Temperature",
                unit: "K",
                id: "temperature_plot",
            },
            PlotValue::Pressure => PlotMetadata {
                name: "Pressure",
                unit: "Pa",
                id: "pressure_plot",
            },
        }
    }
}

fn show_value_plot(
    contexts: &mut EguiContexts,
    time: &VecDeque<f64>,
    values: &VecDeque<f64>,
    plot_value: PlotValue,
) {
    let metadata = plot_value.metadata();
    let points: PlotPoints = time.iter()
        .zip(values.iter())
        .map(|(&t, &v)| [t, v])
        .collect();

    show_plot_window(
        contexts,
        metadata.name,
        metadata.id,
        vec![(points, &format!("{} ({})", metadata.name, metadata.unit))],
    );
}

fn show_plot_window(
    contexts: &mut EguiContexts,
    title: &str,
    plot_id: &str,
    lines: Vec<(PlotPoints, &str)>,
) {
    egui::Window::new(title)
        .default_size([600.0, 400.0])
        .resizable(true)
        .movable(true)
        .collapsible(true)
        .show(contexts.ctx_mut(), |ui| {
            Plot::new(plot_id)
                .legend(egui_plot::Legend::default())
                .include_y(0.0)
                .auto_bounds([true, true].into())
                .show(ui, |plot_ui| {
                    for (points, name) in lines {
                        plot_ui.line(
                            Line::new(points)
                                .name(name)
                                .width(2.0),
                        );
                    }
                });
        });
}

// Add this near the top with other resources
#[derive(Resource, Default)]
pub struct PlotVisibility {
    visible_plots: HashMap<String, bool>,
}

// Add this function to show the control window
fn show_plot_control_window(
    contexts: &mut EguiContexts,
    plot_visibility: &mut PlotVisibility,
) {
    egui::Window::new("Plot Controls")
        .default_size([200.0, 400.0])
        .resizable(true)
        .movable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Show/Hide Plots");
            ui.separator();

            // Generate checkboxes for all plot types
            for plot_type in PlotValue::all() {
                let metadata = plot_type.metadata();
                ui.checkbox(
                    plot_visibility.visible_plots.entry(metadata.name.to_string()).or_insert(false),
                    metadata.name,
                );
            }
        });
}

pub fn update_plots(
    mut kinematics_data: ResMut<KinematicsPlotData>,
    mut forces_data: ResMut<ForcesPlotData>,
    mut gas_data: ResMut<GasPropertiesPlotData>,
    mut plot_visibility: ResMut<PlotVisibility>,
    time: Res<Time<Physics>>,
    mut contexts: EguiContexts,
    balloons: Query<(&Transform, &Forces, &LinearVelocity, &Balloon), With<Balloon>>,
    sim_state: Res<State<SimState>>,
) {
    if *sim_state == SimState::Running {
        if let Some((transform, forces, velocity, balloon)) = balloons.iter().next() {
            let current_time = time.elapsed_secs() as f64;
            
            kinematics_data.push_data(
                current_time,
                transform.translation.y as f64,
                velocity.0.y as f64,
            );

            forces_data.push_data(current_time, forces);

            gas_data.push_data(
                current_time,
                balloon.volume().m3() as f64,
                balloon.gas.temperature.kelvin() as f64,
                balloon.gas.pressure.pascals() as f64,
            );
        }
    }

    show_plot_control_window(&mut contexts, &mut plot_visibility);

    // Show enabled plots
    for plot_type in PlotValue::all() {
        let metadata = plot_type.metadata();
        if *plot_visibility.visible_plots.get(metadata.name).unwrap_or(&false) {
            match plot_type {
                PlotValue::Position => {
                    show_value_plot(&mut contexts, &kinematics_data.time, &kinematics_data.altitude, PlotValue::Position);
                }
                PlotValue::Velocity => {
                    show_value_plot(&mut contexts, &kinematics_data.time, &kinematics_data.velocity, PlotValue::Velocity);
                }
                PlotValue::Forces => {
                    show_plot_window(
                        &mut contexts,
                        metadata.name,
                        metadata.id,
                        vec![
                            (forces_data.get_plot_points(&forces_data.net_force), "Net Force (N)"),
                            (forces_data.get_plot_points(&forces_data.buoyancy), "Buoyancy (N)"),
                            (forces_data.get_plot_points(&forces_data.drag), "Drag (N)"),
                            (forces_data.get_plot_points(&forces_data.weight), "Weight (N)"),
                        ],
                    );
                }
                PlotValue::Volume => {
                    show_value_plot(&mut contexts, &gas_data.time, &gas_data.volume, PlotValue::Volume);
                }
                PlotValue::Temperature => {
                    show_value_plot(&mut contexts, &gas_data.time, &gas_data.temperature, PlotValue::Temperature);
                }
                PlotValue::Pressure => {
                    show_value_plot(&mut contexts, &gas_data.time, &gas_data.pressure, PlotValue::Pressure);
                }
            }
        }
    }
} 
