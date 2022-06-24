#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use egui::plot::{Line, Plot, Value, Values};
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let window_height =ui.available_height();
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Some text 1");
                    ui.label("Some text 2");
                    ui.label("Some text 3");
                });
                
                let sin = (0..1000).map(|i| {
                    let x = i as f64 * 0.01;
                    Value::new(x, x.sin())
                });
                let line = Line::new(Values::from_values_iter(sin));
                let myplot = Plot::new("Sinus plotter").data_aspect(1.5).height(window_height-5.0);
                myplot.show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
            });
            
        });
    }
}
