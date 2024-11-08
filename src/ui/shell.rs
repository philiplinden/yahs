use bevy::prelude::*;
use bevy_egui::{egui::{self, Modifiers, Ui}, EguiContexts};

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_system);
    }
}

fn ui_system(mut egui_context: EguiContexts) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        powered_by_egui_and_bevy(ui);
        file_menu_button(ui);
    });
}

fn powered_by_egui_and_bevy(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "bevy",
            "https://github.com/bevyengine/bevy",
        );
        ui.label(".");
    });
}

fn file_menu_button(ui: &mut Ui) {
    let organize_shortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
    let reset_shortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

    // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
    // or else they would only be checked if the "File" menu was actually open!

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    ui.menu_button("View", |ui| {
        ui.set_min_width(220.0);

        // On the web the browser controls the zoom
        #[cfg(not(target_arch = "wasm32"))]
        {
            egui::gui_zoom::zoom_menu_buttons(ui);
            ui.weak(format!(
                "Current zoom: {:.0}%",
                100.0 * ui.ctx().zoom_factor()
            ))
            .on_hover_text("The UI zoom level, on top of the operating system's default value");
            ui.separator();
        }

        if ui
            .add(
                egui::Button::new("Organize Windows")
                    .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
            )
            .clicked()
        {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
            ui.close_menu();
        }

        if ui
            .add(
                egui::Button::new("Reset app memory")
                    .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
            )
            .on_hover_text("Forget scroll, positions, sizes etc")
            .clicked()
        {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
            ui.close_menu();
        }
    });
}

    // pub fn checkboxes(&mut self, ui: &mut Ui) {
    //     let Self { screens, open } = self;
    //     for screen in screens {
    //         if screen.is_enabled(ui.ctx()) {
    //             let mut is_open = open.contains(screen.name());
    //             ui.toggle_value(&mut is_open, screen.name());
    //             set_open(open, screen.name(), is_open);
    //         }
    //     }
    // }

    // pub fn windows(&mut self, ctx: &Context) {
    //     let Self { screens, open } = self;
    //     for screen in screens {
    //         let mut is_open = open.contains(screen.name());
    //         screen.show(ctx, &mut is_open);
    //         set_open(open, screen.name(), is_open);
    //     }
    // }
