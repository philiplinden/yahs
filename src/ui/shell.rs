use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_system);
    }
}

fn ui_system(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.horizontal(|ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
                // ui.label("Edit");
                // ui.label("View");
                // ui.label("Help");
            });
        });
    });
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            powered_by_egui_and_bevy(ui);
            egui::warn_if_debug_build(ui);
        });
    });
}

fn powered_by_egui_and_bevy(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to("bevy", "https://github.com/bevyengine/bevy");
        ui.label(".");
    });
}
