use std::f64::consts::TAU;
use std::ops::RangeInclusive;

use egui::*;

use egui_plot::{
    Arrows, AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner,
    GridInput, GridMark, HLine, Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, PlotPoint,
    PlotPoints, PlotResponse, Points, Polygon, Text, VLine,
};

// ----------------------------------------------------------------------------

#[derive(PartialEq, Eq)]
enum Trace {
    Interaction,
    LinkedAxes,
}

impl Default for Trace {
    fn default() -> Self {
        Self::LinkedAxes
    }
}

// ----------------------------------------------------------------------------

#[derive(PartialEq, Default)]
pub struct FlightView {
    interaction_demo: InteractionDemo,
    linked_axes_demo: LinkedAxesDemo,
    open_panel: Trace,
}

impl super::UiPanel for FlightView {
    fn name(&self) -> &'static str {
        "🗠 Flight"
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
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.open_panel, Trace::Interaction, "Interaction");
            ui.selectable_value(&mut self.open_panel, Trace::LinkedAxes, "Linked Axes");
        });
        ui.separator();


        match self.open_panel {
            Trace::Interaction => {
                self.interaction_demo.ui(ui);
            }
            Trace::LinkedAxes => {
                self.linked_axes_demo.ui(ui);
            }
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(PartialEq)]
struct LinkedAxesDemo {
    link_x: bool,
    link_y: bool,
    link_cursor_x: bool,
    link_cursor_y: bool,
}

impl Default for LinkedAxesDemo {
    fn default() -> Self {
        Self {
            link_x: true,
            link_y: true,
            link_cursor_x: true,
            link_cursor_y: true,
        }
    }
}

impl LinkedAxesDemo {
    fn line_with_slope(slope: f64) -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| slope * x,
            ..,
            100,
        ))
    }

    fn sin() -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| x.sin(),
            ..,
            100,
        ))
    }

    fn cos() -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| x.cos(),
            ..,
            100,
        ))
    }

    fn configure_plot(plot_ui: &mut egui_plot::PlotUi) {
        plot_ui.line(Self::line_with_slope(0.5));
        plot_ui.line(Self::line_with_slope(1.0));
        plot_ui.line(Self::line_with_slope(2.0));
        plot_ui.line(Self::sin());
        plot_ui.line(Self::cos());
    }

    fn ui(&mut self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Linked axes:");
            ui.checkbox(&mut self.link_x, "X");
            ui.checkbox(&mut self.link_y, "Y");
        });
        ui.horizontal(|ui| {
            ui.label("Linked cursors:");
            ui.checkbox(&mut self.link_cursor_x, "X");
            ui.checkbox(&mut self.link_cursor_y, "Y");
        });

        let link_group_id = ui.id().with("linked_demo");
        ui.horizontal(|ui| {
            Plot::new("left-top")
                .data_aspect(1.0)
                .width(250.0)
                .height(250.0)
                .link_axis(link_group_id, self.link_x, self.link_y)
                .link_cursor(link_group_id, self.link_cursor_x, self.link_cursor_y)
                .show(ui, Self::configure_plot);
            Plot::new("right-top")
                .data_aspect(2.0)
                .width(150.0)
                .height(250.0)
                .y_axis_width(3)
                .y_axis_label("y")
                .y_axis_position(egui_plot::HPlacement::Right)
                .link_axis(link_group_id, self.link_x, self.link_y)
                .link_cursor(link_group_id, self.link_cursor_x, self.link_cursor_y)
                .show(ui, Self::configure_plot);
        });
        Plot::new("left-bottom")
            .data_aspect(0.5)
            .width(250.0)
            .height(150.0)
            .x_axis_label("x")
            .link_axis(link_group_id, self.link_x, self.link_y)
            .link_cursor(link_group_id, self.link_cursor_x, self.link_cursor_y)
            .show(ui, Self::configure_plot)
            .response
    }
}

// ----------------------------------------------------------------------------

#[derive(PartialEq, Default)]
struct ItemsDemo {
    texture: Option<egui::TextureHandle>,
}

impl ItemsDemo {
    fn ui(&mut self, ui: &mut Ui) -> Response {
        let n = 100;
        let mut sin_values: Vec<_> = (0..=n)
            .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
            .map(|i| [i, i.sin()])
            .collect();

        let line = Line::new(sin_values.split_off(n / 2)).fill(-1.5);
        let polygon = Polygon::new(PlotPoints::from_parametric_callback(
            |t| (4.0 * t.sin() + 2.0 * t.cos(), 4.0 * t.cos() + 2.0 * t.sin()),
            0.0..TAU,
            100,
        ));
        let points = Points::new(sin_values).stems(-1.5).radius(1.0);

        let arrows = {
            let pos_radius = 8.0;
            let tip_radius = 7.0;
            let arrow_origins = PlotPoints::from_parametric_callback(
                |t| (pos_radius * t.sin(), pos_radius * t.cos()),
                0.0..TAU,
                36,
            );
            let arrow_tips = PlotPoints::from_parametric_callback(
                |t| (tip_radius * t.sin(), tip_radius * t.cos()),
                0.0..TAU,
                36,
            );
            Arrows::new(arrow_origins, arrow_tips)
        };

        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("plot_demo", egui::ColorImage::example(), Default::default())
        });
        let image = PlotImage::new(
            texture,
            PlotPoint::new(0.0, 10.0),
            5.0 * vec2(texture.aspect_ratio(), 1.0),
        );

        let plot = Plot::new("items_demo")
            .legend(Legend::default().position(Corner::RightBottom))
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);
        plot.show(ui, |plot_ui| {
            plot_ui.hline(HLine::new(9.0).name("Lines horizontal"));
            plot_ui.hline(HLine::new(-9.0).name("Lines horizontal"));
            plot_ui.vline(VLine::new(9.0).name("Lines vertical"));
            plot_ui.vline(VLine::new(-9.0).name("Lines vertical"));
            plot_ui.line(line.name("Line with fill"));
            plot_ui.polygon(polygon.name("Convex polygon"));
            plot_ui.points(points.name("Points with stems"));
            plot_ui.text(Text::new(PlotPoint::new(-3.0, -3.0), "wow").name("Text"));
            plot_ui.text(Text::new(PlotPoint::new(-2.0, 2.5), "so graph").name("Text"));
            plot_ui.text(Text::new(PlotPoint::new(3.0, 3.0), "much color").name("Text"));
            plot_ui.text(Text::new(PlotPoint::new(2.5, -2.0), "such plot").name("Text"));
            plot_ui.image(image.name("Image"));
            plot_ui.arrows(arrows.name("Arrows"));
        })
        .response
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, PartialEq)]
struct InteractionDemo {}

impl InteractionDemo {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) -> Response {
        let id = ui.make_persistent_id("interaction_demo");

        // This demonstrates how to read info about the plot _before_ showing it:
        let plot_memory = egui_plot::PlotMemory::load(ui.ctx(), id);
        if let Some(plot_memory) = plot_memory {
            let bounds = plot_memory.bounds();
            ui.label(format!(
                "plot bounds: min: {:.02?}, max: {:.02?}",
                bounds.min(),
                bounds.max()
            ));
        }

        let plot = Plot::new("interaction_demo").id(id).height(300.0);

        let PlotResponse {
            response,
            inner: (screen_pos, pointer_coordinate, pointer_coordinate_drag_delta, bounds, hovered),
            hovered_plot_item,
            ..
        } = plot.show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(PlotPoints::from_explicit_callback(
                    move |x| x.sin(),
                    ..,
                    100,
                ))
                .color(Color32::RED)
                .id(egui::Id::new("sin")),
            );
            plot_ui.line(
                Line::new(PlotPoints::from_explicit_callback(
                    move |x| x.cos(),
                    ..,
                    100,
                ))
                .color(Color32::BLUE)
                .id(egui::Id::new("cos")),
            );

            (
                plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
                plot_ui.pointer_coordinate(),
                plot_ui.pointer_coordinate_drag_delta(),
                plot_ui.plot_bounds(),
                plot_ui.response().hovered(),
            )
        });

        ui.label(format!(
            "plot bounds: min: {:.02?}, max: {:.02?}",
            bounds.min(),
            bounds.max()
        ));
        ui.label(format!(
            "origin in screen coordinates: x: {:.02}, y: {:.02}",
            screen_pos.x, screen_pos.y
        ));
        ui.label(format!("plot hovered: {hovered}"));
        let coordinate_text = if let Some(coordinate) = pointer_coordinate {
            format!("x: {:.02}, y: {:.02}", coordinate.x, coordinate.y)
        } else {
            "None".to_owned()
        };
        ui.label(format!("pointer coordinate: {coordinate_text}"));
        let coordinate_text = format!(
            "x: {:.02}, y: {:.02}",
            pointer_coordinate_drag_delta.x, pointer_coordinate_drag_delta.y
        );
        ui.label(format!("pointer coordinate drag delta: {coordinate_text}"));

        let hovered_item = if hovered_plot_item == Some(egui::Id::new("sin")) {
            "red sin"
        } else if hovered_plot_item == Some(egui::Id::new("cos")) {
            "blue cos"
        } else {
            "none"
        };
        ui.label(format!("hovered plot item: {hovered_item}"));

        response
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
    chart: Chart
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
        let chart_response = match self.chart {
            Chart::GaussBars => self.bar_gauss(ui),
            Chart::StackedBars => self.bar_stacked(ui),
            Chart::BoxPlot => self.box_plot(ui),
        };
        chart_response;
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

fn is_approx_zero(val: f64) -> bool {
    val.abs() < 1e-6
}

fn is_approx_integer(val: f64) -> bool {
    val.fract().abs() < 1e-6
}
