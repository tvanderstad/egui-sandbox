use eframe::egui::{self, CentralPanel, Frame};

struct MyApp {
    button_id: Option<egui::Id>,
}

impl MyApp {
    fn new() -> Self {
        Self { button_id: None }
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
                ctx.style_mut(|s| s.spacing.item_spacing = egui::vec2(0.0, 0.0));

                if self.button_id.is_some()
                    && ctx.interaction_snapshot(|r| r.clicked) == self.button_id
                {
                    println!("clicked");
                }

                self.button_id = Some(ui.button("button").id);
            });
    }
}
