use eframe::egui;

fn octo_rich_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .color(egui::Color32::DARK_RED)
        .italics()
        .size(32.0)
}

pub trait OctoUiExt {
    /// Adds a dark-red, italic 32-point label and returns its response.
    fn octo_label(&mut self, text: &str) -> egui::Response;

    /// Adds a heading using the same text style as [`Self::octo_label`].
    fn octo_heading(&mut self, text: &str) -> egui::Response;

    fn octo_grid(&mut self, id: &str) -> egui::InnerResponse<()>;
}

impl OctoUiExt for egui::Ui {
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
}