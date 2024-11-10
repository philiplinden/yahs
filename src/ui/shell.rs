use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiShellState>();
        app.add_systems(Update, (shell_ui_system, help_ui_system));
    }
}

#[derive(Resource)]
struct UiShellState {
    debug_ui_open: bool,
    about_ui_open: bool,
}

impl Default for UiShellState {
    fn default() -> Self {
        Self {
            debug_ui_open: false,
            about_ui_open: false,
        }
    }
}

fn shell_ui_system(mut contexts: EguiContexts, mut shell_state: ResMut<UiShellState>) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        // Don't do anything if we don't have a context.
        // This can happen when the window is minimized or not in focus.
        return;
    };
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.horizontal(|ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
                egui::menu::menu_button(ui, "Help", |ui| {
                    #[cfg(feature = "dev")]
                    if ui.button("Debug").clicked() {
                        shell_state.debug_ui_open = true;
                    }
                    if ui.button("About").clicked() {
                        shell_state.about_ui_open = true;
                    };
                });
            });
        });
    });
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            egui::warn_if_debug_build(ui);
        });
    });

}

fn help_ui_system(mut contexts: EguiContexts, mut shell_state: ResMut<UiShellState>) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        // Don't do anything if we don't have a context.
        // This can happen when the window is minimized or not in focus.
        return;
    };
    if shell_state.about_ui_open {
        about_ui_system(&mut shell_state.about_ui_open, ctx);
    }
    if shell_state.debug_ui_open {
        debug_ui_system(&mut shell_state.debug_ui_open, ctx);
    }
}

fn about_ui_system(is_open: &mut bool, ctx: &mut egui::Context) {
    egui::Window::new("About")
        .id(egui::Id::new("about_window"))
        .enabled(true)
        .open(is_open)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Yet Another HAB Simulator");
            powered_by_egui_and_bevy(ui);
        });
}

fn debug_ui_system(is_open: &mut bool, ctx: &mut egui::Context) {
    egui::Window::new("Debug")
        .id(egui::Id::new("debug_window"))
        .enabled(true)
        .open(is_open)
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Debug");
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
