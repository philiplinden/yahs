use egui::*;
use egui_plot::{
    Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot,
};
use log::error;
use crate::gui::View;
use crate::simulator::config::{self, Config};

// ----------------------------------------------------------------------------

const DEFAULT_CONFIG_PATH: &str = "config/default.toml";

#[derive(PartialEq)]
pub struct ConfigView {
    config: Config,
    picked_path: Option<String>,
}

impl Default for ConfigView {
    fn default() -> Self {
        Self {
            config: config::parse_from_file("config/default.toml"),
            picked_path: Some(String::from(DEFAULT_CONFIG_PATH)),
        }
    }
}
impl super::UiPanel for ConfigView {
    fn name(&self) -> &'static str {
        "☰ Config"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .vscroll(false)
            .default_size(vec2(400.0, 400.0))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for ConfigView {
    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {
                self.load_config();
            }
            if ui.button("Choose File").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
            self.display_path(ui);
        });
    }
}

impl ConfigView {
    fn path_string(&mut self) -> String {
        self.picked_path.clone().unwrap_or(String::from("(none)"))
    }

    fn load_config(&mut self) {
        if let Some(picked_path) = &self.picked_path {
            self.config = config::parse_from_file(picked_path);
        } else {
            error!("Choose a valid path first. Got: {:?}", self.path_string());
        }
    }

    fn display_path(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("File:");
            ui.monospace(self.path_string());
        });
    }
}
// ----------------------------------------------------------------------------

#[derive(PartialEq, Default)]
pub struct FlightView;

impl super::UiPanel for FlightView {
    fn name(&self) -> &'static str {
        "🗠 Flight"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .vscroll(false)
            .default_size(vec2(400.0, 400.0))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for FlightView {
    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            egui::reset_button(ui, self);
            ui.collapsing("Instructions", |ui| {
                ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                if cfg!(target_arch = "wasm32") {
                    ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                } else if cfg!(target_os = "macos") {
                    ui.label("Zoom with ctrl / ⌘ + scroll.");
                } else {
                    ui.label("Zoom with ctrl + scroll.");
                }
                ui.label("Reset view with double-click.");
            });
        });
        ui.separator();
        Plot::new("left-bottom")
            .data_aspect(0.5)
            .x_axis_label("flight time (s)")
            .show(ui, Self::configure_plot);
    }
}

impl FlightView {
    fn cos() -> Line {
        Line::new(egui_plot::PlotPoints::from_explicit_callback(
            move |x| x.cos(),
            ..,
            100,
        ))
    }

    fn configure_plot(plot_ui: &mut egui_plot::PlotUi) {
        plot_ui.line(Self::cos());
    }
}

// ----------------------------------------------------------------------------

#[derive(PartialEq, Eq)]
enum Chart {
    GaussBars,
    StackedBars,
    BoxPlot,
}

impl Default for Chart {
    fn default() -> Self {
        Self::GaussBars
    }
}

#[derive(PartialEq, Default)]
pub struct StatsView {
    chart: Chart,
}

impl super::UiPanel for StatsView {
    fn name(&self) -> &'static str {
        "🗠 Stats"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .default_size(vec2(400.0, 400.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for StatsView {
    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            egui::reset_button(ui, self);
            ui.collapsing("Instructions", |ui| {
                ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                if cfg!(target_arch = "wasm32") {
                    ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                } else if cfg!(target_os = "macos") {
                    ui.label("Zoom with ctrl / ⌘ + scroll.");
                } else {
                    ui.label("Zoom with ctrl + scroll.");
                }
                ui.label("Reset view with double-click.");
            });
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.chart, Chart::GaussBars, "Histogram");
            ui.selectable_value(&mut self.chart, Chart::StackedBars, "Stacked Bar Chart");
            ui.selectable_value(&mut self.chart, Chart::BoxPlot, "Box Plot");
        });
        ui.separator();
        let _ = match self.chart {
            Chart::GaussBars => self.bar_gauss(ui),
            Chart::StackedBars => self.bar_stacked(ui),
            Chart::BoxPlot => self.box_plot(ui),
        };
    }
}

impl StatsView {
    fn bar_gauss(&self, ui: &mut Ui) -> Response {
        let chart = BarChart::new(
            (-395..=395)
                .step_by(10)
                .map(|x| x as f64 * 0.01)
                .map(|x| {
                    (
                        x,
                        (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
                    )
                })
                // The 10 factor here is purely for a nice 1:1 aspect ratio
                .map(|(x, f)| Bar::new(x, f * 10.0).width(0.095))
                .collect(),
        )
        .color(Color32::LIGHT_BLUE)
        .name("Normal Distribution");

        Plot::new("Normal Distribution Demo")
            .legend(Legend::default())
            .clamp_grid(true)
            .y_axis_width(3)
            .allow_zoom(true)
            .allow_drag(true)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }

    fn bar_stacked(&self, ui: &mut Ui) -> Response {
        let chart1 = BarChart::new(vec![
            Bar::new(0.5, 1.0).name("Day 1"),
            Bar::new(1.5, 3.0).name("Day 2"),
            Bar::new(2.5, 1.0).name("Day 3"),
            Bar::new(3.5, 2.0).name("Day 4"),
            Bar::new(4.5, 4.0).name("Day 5"),
        ])
        .width(0.7)
        .name("Set 1");

        let chart2 = BarChart::new(vec![
            Bar::new(0.5, 1.0),
            Bar::new(1.5, 1.5),
            Bar::new(2.5, 0.1),
            Bar::new(3.5, 0.7),
            Bar::new(4.5, 0.8),
        ])
        .width(0.7)
        .name("Set 2")
        .stack_on(&[&chart1]);

        let chart3 = BarChart::new(vec![
            Bar::new(0.5, -0.5),
            Bar::new(1.5, 1.0),
            Bar::new(2.5, 0.5),
            Bar::new(3.5, -1.0),
            Bar::new(4.5, 0.3),
        ])
        .width(0.7)
        .name("Set 3")
        .stack_on(&[&chart1, &chart2]);

        let chart4 = BarChart::new(vec![
            Bar::new(0.5, 0.5),
            Bar::new(1.5, 1.0),
            Bar::new(2.5, 0.5),
            Bar::new(3.5, -0.5),
            Bar::new(4.5, -0.5),
        ])
        .width(0.7)
        .name("Set 4")
        .stack_on(&[&chart1, &chart2, &chart3]);

        Plot::new("Stacked Bar Chart Demo")
            .legend(Legend::default())
            .data_aspect(1.0)
            .allow_drag(true)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart1);
                plot_ui.bar_chart(chart2);
                plot_ui.bar_chart(chart3);
                plot_ui.bar_chart(chart4);
            })
            .response
    }

    fn box_plot(&self, ui: &mut Ui) -> Response {
        let yellow = Color32::from_rgb(248, 252, 168);
        let box1 = BoxPlot::new(vec![
            BoxElem::new(0.5, BoxSpread::new(1.5, 2.2, 2.5, 2.6, 3.1)).name("Day 1"),
            BoxElem::new(2.5, BoxSpread::new(0.4, 1.0, 1.1, 1.4, 2.1)).name("Day 2"),
            BoxElem::new(4.5, BoxSpread::new(1.7, 2.0, 2.2, 2.5, 2.9)).name("Day 3"),
        ])
        .name("Experiment A");

        let box2 = BoxPlot::new(vec![
            BoxElem::new(1.0, BoxSpread::new(0.2, 0.5, 1.0, 2.0, 2.7)).name("Day 1"),
            BoxElem::new(3.0, BoxSpread::new(1.5, 1.7, 2.1, 2.9, 3.3))
                .name("Day 2: interesting")
                .stroke(Stroke::new(1.5, yellow))
                .fill(yellow.linear_multiply(0.2)),
            BoxElem::new(5.0, BoxSpread::new(1.3, 2.0, 2.3, 2.9, 4.0)).name("Day 3"),
        ])
        .name("Experiment B");

        let box3 = BoxPlot::new(vec![
            BoxElem::new(1.5, BoxSpread::new(2.1, 2.2, 2.6, 2.8, 3.0)).name("Day 1"),
            BoxElem::new(3.5, BoxSpread::new(1.3, 1.5, 1.9, 2.2, 2.4)).name("Day 2"),
            BoxElem::new(5.5, BoxSpread::new(0.2, 0.4, 1.0, 1.3, 1.5)).name("Day 3"),
        ])
        .name("Experiment C");

        Plot::new("Box Plot Demo")
            .legend(Legend::default())
            .allow_zoom(true)
            .allow_drag(true)
            .show(ui, |plot_ui| {
                plot_ui.box_plot(box1);
                plot_ui.box_plot(box2);
                plot_ui.box_plot(box3);
            })
            .response
    }
}

// ----------------------------------------------------------------------------
