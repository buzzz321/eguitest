#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui::plot::{Line, Plot, Points, Value, Values};
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn main() {
    let options = eframe::NativeOptions::default();
    let mut app = MyApp::default();
    app.data = app.read_meas_data("seq.dat".to_string()).unwrap();
    eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(app)));
}

struct MyApp {
    data: Vec<Value>,
    before: SystemTime,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            data: Vec::<Value>::new(),
            before: SystemTime::now(),
        }
    }
}

impl MyApp {
    #[allow(dead_code)]
    fn get_measurement(&self) -> Line {
        let sin = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            Value::new(x, x.sin())
        });
        Line::new(Values::from_values_iter(sin))
    }
    fn read_meas_data(&mut self, filename: String) -> std::io::Result<Vec<Value>> {
        let mut ret_val = Vec::<Value>::new();
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        for line in contents.lines() {
            let mut parts = line.split_ascii_whitespace();
            let istr = parts.next().unwrap();
            let qstr = parts.next().unwrap();
            ret_val.push(Value::new(
                istr.trim().parse::<f32>().unwrap(),
                qstr.trim().parse::<f32>().unwrap(),
            ));
        }
        Ok(ret_val)
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let window_height = ui.available_height();
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.set_width(120.0);
                        ui.label("Time (ms)");
                        let now = SystemTime::now();
                        let elapse = now.duration_since(self.before).unwrap();
                        self.before = now;
                        ui.text_edit_singleline(&mut elapse.as_millis().to_string());
                    });
                    ui.horizontal(|ui| {
                        ui.set_width(120.0);
                        ui.label("Some text 2");

                        ui.text_edit_singleline(&mut "1234");
                    });
                    ui.horizontal(|ui| {
                        ui.set_width(120.0);
                        ui.label("Some text 3");

                        ui.text_edit_singleline(&mut "1234");
                    });
                });
                ui.vertical(|ui| {
                    for column in 1..5 {
                        ui.horizontal(|ui| {
                            for row in 1..5 {
                                let plot = Plot::new(format!(
                                    "Sinus plotter {}-{}",
                                    row.to_string(),
                                    column.to_string()
                                ))
                                .data_aspect(1.0)
                                .view_aspect(0.2)
                                .height(window_height / 5.0);

                                plot.show(ui, |plot_ui| {
                                    let points =
                                        Points::new(Values::from_values(self.data.to_vec())); //will do a .clone()
                                    plot_ui.points(points);
                                });
                            }
                        });
                    }
                });
            });
        });
    }
}
