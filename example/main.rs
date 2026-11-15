use eframe::egui::{self, CentralPanel, Vec2, vec2};

mod farmcore;

use farmcore::widgets::FarmUiExt;

const WINDOW_SIZE: Vec2 = vec2(400.0, 400.0);

#[derive(Default)]
struct App {
    file_text: String,
    file_error: Option<String>,
}

impl App {
    fn editor(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.file_text)
                    .code_editor()
                    .desired_rows(12)
                    .desired_width(f32::INFINITY),
            );
        });
    }
}

impl App {
    fn demo_content(ui: &mut egui::Ui) {
        egui::Grid::new("Example").show(ui, |ui| {
            ui.label("name");
            ui.label("users");
            ui.end_row();
        });

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

        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new("text").heading().size(80.0));
        });
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello");
            ui.menu_bar(&mut self.file_text, &mut self.file_error);
            self.editor(ui);
            Self::demo_content(ui);
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
        "Farm Note", 
        options, 
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

fn menu_bar(
        &mut self,
        file_text: &mut String,
        file_error: &mut Option<String>,
    ) -> egui::InnerResponse<()> {
        self.horizontal(|ui| {
            ui.menu_button("files", |ui| {
                if ui.button("Load file").clicked() {
                    match std::fs::read_to_string("note.md") {
                        Ok(contents) => {
                            *file_text = contents;
                            *file_error = None;
                        }
                        Err(error) => *file_error = Some(error.to_string()),
                    }
                }
                ui.separator();
                if ui.button("Save").clicked() {
                    match std::fs::write("note.md", file_text.as_str()) {
                        Ok(()) => *file_error = None,
                        Err(error) => *file_error = Some(error.to_string()),
                    }
                }   
            });

            if let Some(error) = file_error.as_deref() {
                ui.colored_label(egui::Color32::RED, error);
            }

        })
    }