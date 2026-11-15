use eframe::egui;

fn octo_rich_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .color(egui::Color32::DARK_RED)
        .italics()
        .size(32.0)
}

pub trait FarmUiExt {
    /// Adds a dark-red, italic 32-point label and returns its response.
    fn octo_label(&mut self, text: &str) -> egui::Response;

    /// Adds a heading using the same text style as [`Self::octo_label`].
    fn octo_heading(&mut self, text: &str) -> egui::Response;


    fn octo_grid(&mut self, id: &str) -> egui::InnerResponse<()>;

    fn menu_bar(
        &mut self,
        file_text: &mut String,
        file_error: &mut Option<String>,
    ) -> egui::InnerResponse<()>;
}

impl FarmUiExt for egui::Ui {
    fn octo_label(&mut self, text: &str) -> egui::Response {
        self.label(octo_rich_text(text))
    }

    fn octo_heading(&mut self, text: &str) -> egui::Response {
        self.heading(octo_rich_text(text))
    }

    fn octo_grid(&mut self, id: &str) -> egui::InnerResponse<()> {
        egui::Grid::new(id).show(self, |ui| {
            ui.label("name");
            ui.label("users");
            ui.end_row();
        })
    }

    fn menu_bar(
        &mut self,
        file_text: &mut String,
        file_error: &mut Option<String>,
    ) -> egui::InnerResponse<()> {
        egui::MenuBar::new().ui(self, |ui| {
            ui.menu_button("File", |ui| {
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
                if ui.button("Quit").clicked() {
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            if let Some(error) = file_error.as_deref() {
                ui.colored_label(egui::Color32::RED, error);
            }

        })
    }
}
