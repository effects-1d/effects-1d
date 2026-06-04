#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use effects_1d_common::easing::BezierEasing;
use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints, Points};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        // sets this to be the default, global collector for this application.
        .init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((640.0, 450.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    p0: egui::Vec2,
    p1: egui::Vec2,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            p0: egui::Vec2::new(0.5, 0.5),
            p1: egui::Vec2::new(0.5, 0.5),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let bezier_easing = BezierEasing::new(self.p0.x, self.p0.y, self.p1.x, self.p1.y);
        let bezier_easing_steps: Vec<PlotPoint> = bezier_easing
            .steps()
            .iter()
            .map(|step| PlotPoint::new(step.x, step.y))
            .collect();

        egui::CentralPanel::default().show_inside(ui, move |ui| {
            ui.horizontal_centered(move |ui| {
                ui.vertical(|ui| {
                    ui.heading("Bezier Parameters");
                    ui.add(egui::Slider::new(&mut self.p0.x, 0.0..=1.0).text("p0.x"));
                    ui.add(egui::Slider::new(&mut self.p0.y, 0.0..=1.0).text("p0.y"));
                    ui.add(egui::Slider::new(&mut self.p1.x, 0.0..=1.0).text("p1.x"));
                    ui.add(egui::Slider::new(&mut self.p1.y, 0.0..=1.0).text("p1.y"));
                });
                ui.vertical(move |ui| {
                    Plot::new("Bezier Plot")
                        .legend(Legend::default())
                        .show(ui, move |plot_ui| {
                            plot_ui.line(Line::new(
                                "",
                                vec![[0.0, 0.0], [self.p0.x as f64, self.p0.y as f64]],
                            ));
                            plot_ui.line(Line::new(
                                "",
                                vec![[1.0, 1.0], [self.p1.x as f64, self.p1.y as f64]],
                            ));
                            //plot_ui.line(Line::new(PlotPoints::Owned(bezier_easing_steps.clone())));
                            plot_ui.points(
                                Points::new(
                                    "",
                                    vec![
                                        [self.p0.x as f64, self.p0.y as f64],
                                        [self.p1.x as f64, self.p1.y as f64],
                                    ],
                                )
                                .radius(4.0),
                            );
                            plot_ui.points(
                                Points::new("", PlotPoints::Owned(bezier_easing_steps)).radius(3.0),
                            );
                            plot_ui.line(Line::new(
                                "",
                                PlotPoints::from_explicit_callback(
                                    move |x| bezier_easing.evaluate(x as f32) as f64,
                                    0.0..=1.0,
                                    512,
                                ),
                            ));
                        })
                });
            });
        });
    }
}
