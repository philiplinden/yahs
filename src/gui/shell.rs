use std::{collections::BTreeSet, path::PathBuf};

use egui::{Context, Modifiers, ScrollArea, Ui};

use super::UiPanel;
use crate::simulator::{
    config::{parse_config, Config},
    schedule::AsyncSim,
};

/// A menu bar in which you can select different info windows to show.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Shell {
    screens: Screens,
    config: Config,
    sim: AsyncSim,
}

impl Default for Shell {
    fn default() -> Self {
        let default_config_path = PathBuf::from("config/default.toml");
        let default_outpath = PathBuf::from("out.csv");
        let config = parse_config(&default_config_path);
        let sim = AsyncSim::new(&default_config_path, default_outpath);
        Self {
            screens: Screens::default(),
            config,
            sim,
        }
    }
}

impl Shell {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }

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

                self.screen_list_ui(ui);
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

    fn screen_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.screens.checkboxes(ui);
                ui.separator();

                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
                if ui.button("Simulate").clicked() {
                    self.sim.start();
                }
            });
        });
    }
}

impl eframe::App for Shell {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::CentralPanel::default().show(ctx, |_ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.ui(ctx);
        });
        egui::TopBottomPanel::bottom("powered_by_eframe").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
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

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
