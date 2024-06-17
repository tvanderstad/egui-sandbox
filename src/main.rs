use eframe::egui::{self, Button, Sense, Ui, Widget as _};

#[derive(Default)]
struct MyApp {}

impl MyApp {
    fn ui(&mut self, ui: &mut egui::Ui) {
        Button::new("A").ui(ui);

        let response = ui.add_sized([200.0, 20.0], |ui: &mut Ui| {
            let (rect, response) = ui.allocate_exact_size(
                egui::Vec2 { x: 100.0, y: 10.0 },
                Sense::focusable_noninteractive(),
            );
            let color = if ui.memory(|m| m.has_focus(response.id)) {
                egui::Color32::BLUE
            } else {
                egui::Color32::WHITE
            };
            ui.painter().rect(rect, 0.0, color, egui::Stroke::NONE);
            response
        });

        Button::new("B").ui(ui);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

fn main() {
    eframe::run_native(
        "test",
        eframe::NativeOptions {
            ..Default::default()
        },
        Box::new(|_cc: &eframe::CreationContext| Ok(Box::new(MyApp::default()))),
    )
    .unwrap();
}
