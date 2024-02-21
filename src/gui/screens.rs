use std::collections::BTreeSet;

use egui::{Context, Modifiers, ScrollArea, Ui};

use super::UiPanel;

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct Screens {
    #[cfg_attr(feature = "serde", serde(skip))]
    screens: Vec<Box<dyn UiPanel>>,

    open: BTreeSet<String>,
}

impl Default for Screens {
    fn default() -> Self {
        Self::from_demos(vec![
            Box::<super::views::FlightView>::default(),
            Box::<super::views::StatsView>::default(),
        ])
    }
}

impl Screens {
    pub fn from_demos(screens: Vec<Box<dyn UiPanel>>) -> Self {
        let mut open = BTreeSet::new();
        open.insert(super::views::FlightView::default().name().to_owned());

        Self { screens, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { screens, open } = self;
        for screen in screens {
            if screen.is_enabled(ui.ctx()) {
                let mut is_open = open.contains(screen.name());
                ui.toggle_value(&mut is_open, screen.name());
                set_open(open, screen.name(), is_open);
            }
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { screens, open } = self;
        for screen in screens {
            let mut is_open = open.contains(screen.name());
            screen.show(ctx, &mut is_open);
            set_open(open, screen.name(), is_open);
        }
    }
}

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

// ----------------------------------------------------------------------------

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// ----------------------------------------------------------------------------

/// A menu bar in which you can select different demo windows to show.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct MissionControl {
    screens: Screens,
}

impl Default for MissionControl {
    fn default() -> Self {
        Self {
            screens: Default::default(),
        }
    }
}

impl MissionControl {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        egui::SidePanel::left("mission_control_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("yahs");
                });

                ui.separator();

                use egui::special_emojis::GITHUB;
                ui.hyperlink_to(
                    format!("{GITHUB} yahs on GitHub"),
                    "https://github.com/brickworks/yahs",
                );
                ui.hyperlink_to(
                    format!("{GITHUB} @philiplinden"),
                    "https://github.com/philiplinden",
                );

                ui.separator();

                self.demo_list_ui(ui);
            });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file_menu_button(ui);
            });
        });

        self.show_windows(ctx);
    }

    /// Show the open windows.
    fn show_windows(&mut self, ctx: &Context) {
        self.screens.windows(ctx);
    }

    fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.screens.checkboxes(ui);
                ui.separator();

                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        });
    }
}

// ----------------------------------------------------------------------------

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

    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

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
