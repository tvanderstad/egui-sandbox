use eframe::egui::{self, CentralPanel, Color32, Frame, Id, Rect, Rounding, Sense, Stroke, Vec2};

struct MyApp {
    rect_id: Option<egui::Id>,
}

impl MyApp {
    fn new() -> Self {
        Self { rect_id: None }
    }
}

fn main() {
    eframe::run_native(
        "test",
        eframe::NativeOptions::default(),
        Box::new(|_cc: &eframe::CreationContext| Ok(Box::new(MyApp::new()))),
    )
    .unwrap();
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(Frame {
                fill: ctx.style().visuals.panel_fill,
                ..Default::default() // removes default margins
            })
            .show(ctx, |ui| {
                println!("----- show -----");
                ctx.style_mut(|s| s.spacing.item_spacing = egui::vec2(0.0, 0.0));

                if let Some(rect_id) = self.rect_id {
                    if let Some(response) = ctx.read_response(rect_id) {
                        if response.clicked() {
                            println!("response reported CLICKED on next frame");
                        }
                        if response.hovered() {
                            println!("response reported HOVERED on next frame");
                        }
                    }
                }

                let id = Id::new(0);
                let (_, rect) = ui.allocate_space(Vec2::new(100.0, 100.0));
                let response = ui.interact(rect, id, Sense::click_and_drag());

                ui.painter().rect(
                    rect,
                    Rounding::ZERO,
                    Color32::TRANSPARENT,
                    Stroke::new(2., Color32::BLUE),
                );

                self.rect_id = Some(response.id);
                if response.clicked() {
                    println!("response reported CLICKED on frame drawn");
                }
                if response.hovered() {
                    println!("response reported HOVERED on frame drawn");
                }
            });
    }
}
