use eframe::egui::{self, CentralPanel, Vec2, text_selection::visuals, vec2};

mod farmcore;

use farmcore::widgets::FarmUiExt;

const WINDOW_SIZE: Vec2 = vec2(400.0, 400.0);

#[derive(Default)]
struct App {
    file_text: String,
    file_error: Option<String>,
}

impl App {
    fn setup_theme(ctx: &egui::Context) {
        ctx.all_styles_mut(|style| {
            style.visuals.menu_corner_radius = egui::CornerRadius::ZERO;
            style.visuals.window_fill = egui::Color32::from_rgb(70, 20, 20);

            let widgets = &mut style.visuals.widgets;
            widgets.inactive.bg_stroke = egui::Stroke::NONE;
            widgets.inactive.corner_radius = egui::CornerRadius::ZERO;
            widgets.inactive.fg_stroke.color = egui::Color32::WHITE;

            widgets.hovered.bg_stroke = egui::Stroke::NONE;
            widgets.hovered.corner_radius = egui::CornerRadius::ZERO;
            widgets.hovered.fg_stroke.color = egui::Color32::WHITE;

            widgets.active.bg_stroke = egui::Stroke::NONE;
            widgets.active.corner_radius = egui::CornerRadius::ZERO;
            widgets.active.fg_stroke.color = egui::Color32::WHITE;

            widgets.open.corner_radius = egui::CornerRadius::ZERO;


            widgets.open.bg_stroke = egui::Stroke::NONE;
            widgets.open.corner_radius = egui::CornerRadius::ZERO;
            widgets.open.fg_stroke.color = egui::Color32::WHITE;

            widgets.inactive.weak_bg_fill = egui::Color32::RED; // normal
            widgets.hovered.weak_bg_fill = egui::Color32::RED;  // hovered
            widgets.active.weak_bg_fill = egui::Color32::RED;   // pressed
            widgets.open.weak_bg_fill = egui::Color32::RED;     // menu open

            style.spacing.button_padding = egui::vec2(14.0, 5.0);
        });
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("menu_bar")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::RED)
                    .stroke(egui::Stroke::NONE),
            )
            .show(ui, |ui| {
            egui::MenuBar::new()
                .ui(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        ui.button("click me");
                    });
                });
            });

        CentralPanel::default().show(ui, |ui| {
            ui.heading("Farm Note");
            ui.menu_button("File", |ui| {});
            ui.add(
                egui::TextEdit::multiline(&mut self.file_text)
                    .code_editor()
                    .desired_rows(20)
                    .desired_width(f32::INFINITY),
            );
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
        Box::new(|cc| {
            App::setup_theme(&cc.egui_ctx);
            Ok(Box::new(App::default()))
        }),
    )
}

