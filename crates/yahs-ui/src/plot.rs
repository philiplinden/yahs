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
use yahs::prelude::{Balloon, Forces};

const MAX_HISTORY: usize = 1000; // Store last 1000 data points

#[derive(Resource)]
pub struct PlotData {
    time: VecDeque<f64>,
    altitude: VecDeque<f64>,
    velocity: VecDeque<f64>,
    net_force: VecDeque<f64>,
}

impl Default for PlotData {
    fn default() -> Self {
        Self {
            time: VecDeque::with_capacity(MAX_HISTORY),
            altitude: VecDeque::with_capacity(MAX_HISTORY),
            velocity: VecDeque::with_capacity(MAX_HISTORY),
            net_force: VecDeque::with_capacity(MAX_HISTORY),
        }
    }
}

impl PlotData {
    fn push_data(
        &mut self,
        time: f64,
        altitude: f64,
        velocity: f64,
        net_force: f64,
    ) {
        if self.time.len() >= MAX_HISTORY {
            self.time.pop_front();
            self.altitude.pop_front();
            self.velocity.pop_front();
            self.net_force.pop_front();
        }

        self.time.push_back(time);
        self.altitude.push_back(altitude);
        self.velocity.push_back(velocity);
        self.net_force.push_back(net_force);
    }

    fn get_plot_points(&self, values: &VecDeque<f64>) -> PlotPoints {
        self.time
            .iter()
            .zip(values.iter())
            .map(|(&t, &v)| [t, v])
            .collect()
    }
}

pub fn update_plots(
    mut plot_data: ResMut<PlotData>,
    time: Res<Time<Physics>>,
    mut contexts: EguiContexts,
    balloons: Query<(&Transform, &Forces, &LinearVelocity, &Children), With<Balloon>>,
) {
    if let Some((transform, forces, velocity, children)) = balloons.iter().next() {
        plot_data.push_data(
            time.elapsed_secs() as f64,
            transform.translation.y as f64,
            velocity.0.y as f64,
            forces.net_force().force.y as f64,
        );
    }

    egui::Window::new("Telemetry")
        .default_size([600.0, 400.0])
        .resizable(true)
        .movable(true)
        .collapsible(true)
        .show(contexts.ctx_mut(), |ui| {
            Plot::new("balloon_plots")
                .legend(egui_plot::Legend::default())
                .view_aspect(2.0)
                .include_y(0.0)
                .auto_bounds([true, true].into())
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new(plot_data.get_plot_points(&plot_data.altitude))
                            .name("Altitude (m)")

                            .width(2.0),
                    );
                    plot_ui.line(
                        Line::new(plot_data.get_plot_points(&plot_data.velocity))
                            .name("Velocity (m/s)")
                            .width(2.0),
                    );
                    plot_ui.line(
                        Line::new(plot_data.get_plot_points(&plot_data.net_force))
                            .name("Net Force (N)")
                            .width(2.0),
                    );
                });
        });
} 
