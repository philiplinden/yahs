use std::collections::BTreeSet;
use super::{
    Monitor,
    monitors,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct Monitors {
    #[cfg_attr(feature = "serde", serde(skip))]
    monitors: Vec<Box<dyn Monitor>>,

    open: BTreeSet<String>,
}

impl Default for Monitors {
    fn default() -> Self {
        Self::from_monitors(vec![
            Box::<monitors::Altitude>::default(),
            Box::<monitors::AtmoDensity>::default(),
            Box::<monitors::AtmoPressure>::default(),
            Box::<monitors::AtmoTemperature>::default(),
            Box::<monitors::GforceCompass>::default(),
            Box::<monitors::Balloon>::default(),
            Box::<monitors::VelocityAcceleration>::default(),
            Box::<monitors::Forces>::default(),
        ])
    }
}

impl Monitors {
    pub fn from_monitors(monitors: Vec<Box<dyn Monitor>>) -> Self {
        let mut open = BTreeSet::new();
        Self { monitors, open }
    }

    pub fn checkboxes(&mut self, ui: &mut egui::Ui) {
        let Self { monitors, open } = self;
        for monitor in monitors {
            if monitor.is_enabled(ui.ctx()) {
                let mut is_open = open.contains(monitor.name());
                ui.toggle_value(&mut is_open, monitor.name());
                set_open(open, monitor.name(), is_open);
            }
        }
    }

    pub fn windows(&mut self, ctx: &egui::Context) {
        let Self { monitors, open } = self;
        for monitor in monitors {
            let mut is_open = open.contains(monitor.name());
            monitor.show(ctx, &mut is_open);
            set_open(open, monitor.name(), is_open);
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

/// A menu bar in which you can select different monitor windows to show.
// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Shell {
    about_is_open: bool,
    about: About,
    monitors: Monitors,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            about_is_open: true,
            about: Default::default(),
            monitors: Default::default(),
        }
    }
}
impl Shell {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Shell {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            PlotPanel::default().show(ctx);

            ui.separator();

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
