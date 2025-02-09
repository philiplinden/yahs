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
use yahs::prelude::{Balloon, Forces, ForceType};

const MAX_HISTORY: usize = 1000; // Store last 1000 data points

pub struct PlotPlugin;

impl Plugin for PlotPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<KinematicsPlotData>()
            .init_resource::<ForcesPlotData>()
            .init_resource::<GasPropertiesPlotData>();
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

pub fn update_plots(
    mut kinematics_data: ResMut<KinematicsPlotData>,
    mut forces_data: ResMut<ForcesPlotData>,
    mut gas_data: ResMut<GasPropertiesPlotData>,
    time: Res<Time<Physics>>,
    mut contexts: EguiContexts,
    balloons: Query<(&Transform, &Forces, &LinearVelocity, &Balloon), With<Balloon>>,
) {
    if let Some((transform, forces, velocity, balloon)) = balloons.iter().next() {
        let current_time = time.elapsed_secs() as f64;
        
        kinematics_data.push_data(
            current_time,
            transform.translation.y as f64,
            velocity.0.y as f64,
        );

        forces_data.push_data(current_time, forces);

        // Add gas properties data
        gas_data.push_data(
            current_time,
            balloon.volume().m3() as f64,
            balloon.gas.temperature.kelvin() as f64,
            balloon.gas.pressure.pascals() as f64,
        );
    }

    // Kinematics plot
    show_plot_window(
        &mut contexts,
        "Kinematics",
        "kinematics_plots",
        vec![
            (kinematics_data.get_plot_points(&kinematics_data.altitude), "Altitude (m)"),
            (kinematics_data.get_plot_points(&kinematics_data.velocity), "Velocity (m/s)"),
        ],
    );

    // Forces plot
    show_plot_window(
        &mut contexts,
        "Forces",
        "force_plots",
        vec![
            (forces_data.get_plot_points(&forces_data.net_force), "Net Force (N)"),
            (forces_data.get_plot_points(&forces_data.buoyancy), "Buoyancy (N)"),
            (forces_data.get_plot_points(&forces_data.drag), "Drag (N)"),
            (forces_data.get_plot_points(&forces_data.weight), "Weight (N)"),
        ],
    );

    // Gas properties plot
    show_plot_window(
        &mut contexts,
        "Gas Properties",
        "gas_properties_plots",
        vec![
            (gas_data.get_plot_points(&gas_data.volume), "Volume (m³)"),
            (gas_data.get_plot_points(&gas_data.temperature), "Temperature (K)"),
            (gas_data.get_plot_points(&gas_data.pressure), "Pressure (Pa)"),
        ],
    );
} 
