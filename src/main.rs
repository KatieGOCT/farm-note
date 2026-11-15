use eframe::egui::{self, CentralPanel, Vec2, vec2};

mod octo;

use octo::widgets::OctoUiExt;

const WINDOW_SIZE: Vec2 = vec2(400.0, 400.0);

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello");
            ui.octo_label("test");
            ui.separator();
            ui.label("First Tab");
            ui.label("Second Tab");
            ui.label("Third Tab");
            ui.octo_label("Hellow new me");
            ui.octo_grid("30303");

            // Horizontal layout
            ui.horizontal(|ui | {
                ui.label("Techhs");
                if ui.button("Save").clicked() {
                    println!("Clicked")
                }

            });

            // Grid
            egui::Grid::new("Example").show(ui, |ui| {
                ui.label("name");
                ui.label("users");
                ui.end_row();
            });

                        // Grid
            egui::Frame::new()
                .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                .show(ui, |ui| {
                    egui::Grid::new("Example1").show(ui, |ui| {
                        ui.label("name22");
                        ui.label("users111");
                });
                
            });
            egui::Grid::new("Example2").show(ui, |ui| {
                ui.label("name");
                ui.label("users");
                ui.end_row();
            });


            // center 
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("text")
                    .heading()
                    .size(80.0)
                );
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Jrss", 
        options, 
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}