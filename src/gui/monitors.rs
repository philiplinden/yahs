use std::f64::consts::TAU;

use egui::*;
use egui_plot::{CoordinatesFormatter, Corner, Legend, Line, LineStyle, Plot, PlotPoints};

use crate::simulator::SimOutput;

pub enum TelemetryStream{
    Kinematics
}

#[derive(PartialEq)]
pub struct Monitor {
    title: String,
    telemetry: TelemetryStream,
    follow: bool,
    coordinates: bool,
    show_axes: bool,
    show_grid: bool,
}

impl Default for Monitor {
    fn default() -> Self {
        Self {
            title: String::from("📺"),
            telemetry: TelemetryStream,
            follow: false,
            coordinates: true,
            show_axes: true,
            show_grid: true,
        }
    }
}

impl super::UiPanel for Monitor {
    fn name(&self) -> &'static str {
        "📺 Monitor"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .default_size(vec2(400.0, 400.0))
            .vscroll(false)
            .show(ctx, |ui| self.telemetry.ui(ui));
    }
}

impl Monitor {
    fn ui(&mut self, ui: &mut Ui) {
    
    ui.horizontal(|ui| {
        ui.group(|ui| {
            self.about_ui(ui);
        });
    });
    }

    fn about_ui(&mut self, ui: &mut Ui) {
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
    })}

    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            title,
            telemetry,
            follow,
            coordinates,
            show_axes,
            show_grid,
        } = self;
            ui.collapsing("Options", |ui| {
                ui.vertical(|ui| {
                    ui.checkbox(show_axes, "Show axes");
                    ui.checkbox(show_grid, "Show grid");
                    ui.checkbox(coordinates, "Show coordinates on hover")
                        .on_hover_text("Can take a custom formatting function.");
                })
            })
        }
}

// struct Altitude {}

// impl Telemetry for Altitude {}

// impl Altitude {
//     fn ui(&mut self, ui: &mut Ui) -> Response {
//         self.instructions_ui(ui);
//         self.options_ui(ui);

//         ui.separator();

//         if self.animate {
//             ui.ctx().request_repaint();
//             self.time += ui.input(|i| i.unstable_dt).at_most(1.0 / 30.0) as f64;
//         };
//         let plot = Plot::new("plot")
//             .legend(Legend::default())
//             .y_axis_width(4)
//             .show_axes(self.show_axes)
//             .show_grid(self.show_grid)
//             .coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());

//         plot.show(ui, |plot_ui| {
//             plot_ui.line(self.circle());
//             plot_ui.line(self.sin());
//             plot_ui.line(self.thingy());
//         })
//         .response
//     }
//         ui.horizontal(|ui| {
//             ui.group(|ui| {
//                 ui.vertical(|ui| {
//                     ui.label("Circle:");
//                     ui.add(
//                         egui::DragValue::new(circle_radius)
//                             .speed(0.1)
//                             .clamp_range(0.0..=f64::INFINITY)
//                             .prefix("r: "),
//                     );
//                     ui.horizontal(|ui| {
//                         ui.add(
//                             egui::DragValue::new(&mut circle_center.x)
//                                 .speed(0.1)
//                                 .prefix("x: "),
//                         );
//                         ui.add(
//                             egui::DragValue::new(&mut circle_center.y)
//                                 .speed(1.0)
//                                 .prefix("y: "),
//                         );
//                     });
//                 });
//             });


//             ui.vertical(|ui| {
//                 ui.style_mut().wrap = Some(false);
//                 ui.checkbox(animate, "Animate");
//                 ui.checkbox(square, "Square view")
//                     .on_hover_text("Always keep the viewport square.");
//                 ui.checkbox(proportional, "Proportional data axes")
//                     .on_hover_text("Tick are the same size on both axes.");

//                 ComboBox::from_label("Line style")
//                     .selected_text(line_style.to_string())
//                     .show_ui(ui, |ui| {
//                         for style in &[
//                             LineStyle::Solid,
//                             LineStyle::dashed_dense(),
//                             LineStyle::dashed_loose(),
//                             LineStyle::dotted_dense(),
//                             LineStyle::dotted_loose(),
//                         ] {
//                             ui.selectable_value(line_style, *style, style.to_string());
//                         }
//                     });
//             });
//         });
//     }

//     fn circle(&self) -> Line {
//         let n = 512;
//         let circle_points: PlotPoints = (0..=n)
//             .map(|i| {
//                 let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
//                 let r = self.circle_radius;
//                 [
//                     r * t.cos() + self.circle_center.x as f64,
//                     r * t.sin() + self.circle_center.y as f64,
//                 ]
//             })
//             .collect();
//         Line::new(circle_points)
//             .color(Color32::from_rgb(100, 200, 100))
//             .style(self.line_style)
//             .name("circle")
//     }

//     fn sin(&self) -> Line {
//         let time = self.time;
//         Line::new(PlotPoints::from_explicit_callback(
//             move |x| 0.5 * (2.0 * x).sin() * time.sin(),
//             ..,
//             512,
//         ))
//         .color(Color32::from_rgb(200, 100, 100))
//         .style(self.line_style)
//         .name("wave")
//     }

//     fn thingy(&self) -> Line {
//         let time = self.time;
//         Line::new(PlotPoints::from_parametric_callback(
//             move |t| ((2.0 * t + time).sin(), (3.0 * t).sin()),
//             0.0..=TAU,
//             256,
//         ))
//         .color(Color32::from_rgb(100, 150, 250))
//         .style(self.line_style)
//         .name("x = sin(2t), y = sin(3t)")
//     }
// }
