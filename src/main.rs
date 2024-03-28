#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui::Vec2;
use egui_plot::{Legend, Plot, PlotPoints, Points};
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

const ROW: usize = 5;
const COL: usize = 5;

fn main() {
    let options = eframe::NativeOptions::default();
    let mut app = MyApp::default();
    app.data = match app.read_meas_data("seq.dat".to_string()) {
        Ok(data) => data,
        Err(_) => app.get_measurement(),
    };
    _ = eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(app)));
}

struct MyApp {
    data: PlotPoints,
    before: SystemTime,
    plot_clicked: [bool; COL * ROW],
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            data: PlotPoints::new(vec![[0.0, 0.0]]),
            before: SystemTime::now(),
            plot_clicked: [false; COL * ROW],
        }
    }
}

impl MyApp {
    #[allow(dead_code)]
    fn get_measurement(&self) -> PlotPoints {
        (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x.cos(), x.sin()]
            })
            .collect::<PlotPoints>()
    }

    fn read_meas_data(&mut self, filename: String) -> std::io::Result<PlotPoints> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let ret_val = contents
            .lines()
            .map(|line| {
                let mut parts = line.split_ascii_whitespace();
                let istr = parts.next().unwrap();
                let qstr = parts.next().unwrap();
                [
                    istr.trim().parse::<f64>().unwrap(),
                    qstr.trim().parse::<f64>().unwrap(),
                ]
            })
            .collect();
        Ok(ret_val)
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let window_height = ui.available_height();
            let window_width = ui.available_width();
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
                    for row in 1..ROW {
                        ui.horizontal(|ui| {
                            for column in 1..COL {
                                let grid_size_x = window_width / ROW as f32;
                                let grid_size_y = window_height / COL as f32;
                                let plot = Plot::new(format!("Sinus plotter {row}-{column}"))
                                    .data_aspect(1.0)
                                    .set_margin_fraction(Vec2 { x: 0.01, y: 0.01 })
                                    .height(grid_size_x * 0.20)
                                    .width(grid_size_y * 0.20);

                                plot.show(ui, |plot_ui| {
                                    let points = Points::new(
                                        self.data
                                            .points()
                                            .iter()
                                            .map(|i| [i.x, i.y])
                                            .collect::<Vec<[f64; 2]>>(),
                                    ); //will do a .clone()
                                    plot_ui.points(points);
                                    if plot_ui.response().clicked() {
                                        for (_, value) in self.plot_clicked.iter_mut().enumerate() {
                                            *value = false;
                                        }
                                        println!("--> row {row} col {column}");
                                        self.plot_clicked[column + row * ROW] = true;
                                    }
                                });
                            }
                        });
                    }
                });
                for (index, value) in self.plot_clicked.iter_mut().enumerate() {
                    if *value {
                        //println!("{}  {}-{}", index, index / (ROW), index % COL);
                        ui.label(format!("Plot  {}-{}", index / (ROW), index % COL));
                        //*value = false;
                        let row = (index / ROW) as u32;
                        let col = (index % COL) as u32;
                        let plot = Plot::new(format!("Sinus plotter {row}-{col}"))
                            .data_aspect(1.0)
                            .view_aspect(1.0)
                            .legend(Legend::default())
                            .height(window_height);
                        plot.show(ui, |plot_ui| {
                            let points = Points::new(
                                self.data
                                    .points()
                                    .iter()
                                    .map(|i| [i.x, i.y])
                                    .collect::<Vec<[f64; 2]>>(),
                            ); //will do a .clone()
                            plot_ui.points(points);
                        });
                    }
                }
            });
        });
    }
}
