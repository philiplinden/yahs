use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use super::UiPanel;

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<Shell>()
            .add_systems(Update, ui_system);
    }
}

#[derive(Resource)]
pub struct Shell {
    screens: Screens,
    config: Option<EnvConfig>,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            screens: Screens::default(),
            config: None,
            output: None,
            run_handle: None,
        }
    }
}

fn ui_system(mut egui_context: ResMut<EguiContext>, mut shell: ResMut<Shell>) {
    egui::SidePanel::left("mission_control_panel")
        .resizable(false)
        .default_width(150.0)
        .show(egui_context.ctx_mut(), |ui| {
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
            egui::widgets::global_dark_light_mode_buttons(ui);
            ui.separator();
            shell.screen_list_ui(ui);
            ui.separator();
            shell.sim_control_buttons(ui);
            ui.separator();
        });

    egui::TopBottomPanel::top("menu_bar").show(egui_context.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            file_menu_button(ui);
        });
    });

    shell.show_windows(egui_context.ctx_mut());

    egui::TopBottomPanel::bottom("powered_by_bevy_egui").show(egui_context.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_bevy(ui);
        });
    });
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
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.separator();
                self.screen_list_ui(ui);
                ui.separator();
                self.sim_control_buttons(ui);
                ui.separator();
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
            });
        });
    }

    fn sim_control_buttons(&mut self, ui: &mut egui::Ui) {
        if ui.button("Simulate").clicked() {
            if self.run_handle.is_some() {
                panic!("Can't start again, sim already ran. Need to stop.")
            }
            let outpath = PathBuf::from("out.csv");
            let init_state = Arc::new(Mutex::new(SimOutput::default()));
            if let Some(config) = self.config.clone() {
                let output = init_state.clone();
                self.run_handle = Some(std::thread::spawn(move || {
                    AsyncSim::run_sim(config, output, outpath);
                }));
            }
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
struct Screens {
    screens: Vec<Box<dyn UiPanel>>,
    open: BTreeSet<String>,
}

impl Screens {
    pub fn from_demos(screens: Vec<Box<dyn UiPanel>>) -> Self {
        let mut open = BTreeSet::new();
        open.insert(super::views::ConfigView::default().name().to_owned());

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
            Box::<super::views::ConfigView>::default(),
            Box::<super::views::FlightView>::default(),
            Box::<super::views::StatsView>::default(),
        ])
    }
}

impl Screens {
    pub fn from_demos(screens: Vec<Box<dyn UiPanel>>) -> Self {
        let mut open = BTreeSet::new();
        open.insert(super::views::ConfigView::default().name().to_owned());

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
