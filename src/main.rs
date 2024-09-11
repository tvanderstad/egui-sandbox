use eframe::egui::{self, scroll_area, CentralPanel, Frame, Pos2, Rect, Sense};

struct MyApp {}

impl MyApp {
    fn new() -> Self {
        Self {}
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

                let available_size = ui.available_size();

                // show ui
                let scroll_area_id = ui.id().with(egui::Id::new("id"));
                let maybe_prev_state: Option<scroll_area::State> =
                    ui.data_mut(|d| d.get_persisted(scroll_area_id));

                let scroll_area_output =
                    egui::ScrollArea::vertical().id_source("id").show(ui, |ui| {
                        for i in 0..100 {
                            ui.label(format!("label {}", i));
                        }

                        let rect = Rect::from_min_size(Pos2::ZERO, available_size);
                        let response = ui.allocate_rect(rect, Sense::click());
                        if let Some(pos) = response.interact_pointer_pos() {
                            println!("clicked {:?}", pos);
                        };
                    });

                if let Some(prev_state) = maybe_prev_state {
                    if prev_state.offset != scroll_area_output.state.offset {
                        println!("scrolled");
                    }
                }
            });
    }
}
