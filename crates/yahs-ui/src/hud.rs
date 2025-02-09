use crate::{
    controls::KeyBindingsConfig, 
    plot::update_plots,
};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use yahs::prelude::{Balloon, Forces, SimState};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, (update_hud, update_plots).chain());
    }
}

fn update_hud(
    mut contexts: EguiContexts,
    time: Res<Time<Physics>>,
    state: Res<State<SimState>>,
    key_bindings: Res<KeyBindingsConfig>,
    balloons: Query<(Entity, &Transform, &Forces, &LinearVelocity, &Children), With<Balloon>>,
    children_forces: Query<&Forces, With<Parent>>,
) {
    egui::Window::new("System Info")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            // Simulation info section
            ui.label(format!("Sim State: {:?}", state.get()));
            ui.label(format!("Physics Time: {:.4} s", time.elapsed_secs()));

            // Balloon info section
            for (entity, transform, forces, velocity, children) in balloons.iter() {
                ui.add_space(8.0);
                ui.label(format!("{:?}", entity));
                ui.label(format!("Position: {:.4} m", transform.translation));
                ui.label(format!("Velocity: {:.4} m/s", velocity.0));

                let mut num_forces = forces.vectors.len();
                let mut total_force = forces.net_force();

                for &child in children.iter() {
                    if let Ok(child_forces) = children_forces.get(child) {
                        total_force += child_forces.net_force();
                        num_forces += child_forces.vectors.len();
                    }
                }
                ui.label(format!(
                    "Forces: {:.4} N from {:?} forces",
                    total_force.force, num_forces
                ));
            }
        });

    // Controls window
    egui::Window::new("Controls")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(format!(
                "Toggle Pause: {:?}",
                key_bindings.time_controls.toggle_pause
            ));
            ui.label(format!("Faster: {:?}", key_bindings.time_controls.faster));
            ui.label(format!("Slower: {:?}", key_bindings.time_controls.slower));
            ui.label(format!(
                "Reset Speed: {:?}",
                key_bindings.time_controls.reset_speed
            ));
            ui.label(format!(
                "Step Once: {:?}",
                key_bindings.time_controls.step_once
            ));
            ui.label(format!(
                "Rotate Camera: {:?} Mouse",
                key_bindings.camera_controls.hold_look
            ));
        });
}
