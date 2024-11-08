use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::config::PropertiesConfig;
use crate::AppState;

pub struct BalloonDesignerPlugin;

impl Plugin for BalloonDesignerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BalloonDesignerState>()
            .add_systems(Update, ui_system.run_if(in_state(AppState::Running)));
    }
}

#[derive(Resource)]
struct BalloonDesignerState {
    diameter: f32,
    thickness: f32,
    selected_gas: usize,
    selected_material: usize,
}

impl Default for BalloonDesignerState {
    fn default() -> Self {
        Self {
            diameter: 1.5,
            thickness: 0.0001,
            selected_gas: 0,
            selected_material: 0,
        }
    }
}

fn ui_system(
    mut egui_context: EguiContexts,
    mut state: ResMut<BalloonDesignerState>,
    properties: Res<PropertiesConfig>,
) {
    egui::Window::new("Balloon Designer").show(egui_context.ctx_mut(), |ui| {
        ui.label("Adjust the balloon properties:");

        ui.add(egui::Slider::new(&mut state.diameter, 0.5..=5.0).text("Diameter (m)"));
        ui.add(egui::Slider::new(&mut state.thickness, 0.00001..=0.01).text("Thickness (m)"));

        ui.label("Select Gas Species:");
        if let Some(gas) = properties.gases.get(state.selected_gas) {
            egui::ComboBox::from_label("Gas")
                .selected_text(&gas.name)
                .show_ui(ui, |ui| {
                    for (index, gas) in properties.gases.iter().enumerate() {
                        ui.selectable_value(&mut state.selected_gas, index, &gas.name);
                    }
                });
        }

        ui.label("Select Balloon Material:");
        if let Some(materials) = properties.materials.get(state.selected_material) {
            egui::ComboBox::from_label("Material")
                .selected_text(&materials.name)
                .show_ui(ui, |ui| {
                    for (index, material) in properties.materials.iter().enumerate() {
                        ui.selectable_value(&mut state.selected_material, index, &material.name);
                    }
                });
        }
    });
}
