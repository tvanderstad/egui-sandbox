use eframe::egui::{self, scroll_area, Id, Sense};

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
        egui::CentralPanel::default().show(ctx, |ui| {
            // show ui
            let maybe_prev_state: Option<scroll_area::State> =
                ctx.data_mut(|d| d.get_persisted(ui.id().with(Id::new("id"))));
            let scroll_area_output = egui::ScrollArea::vertical().id_source("id").show(ui, |ui| {
                for i in 0..100 {
                    ui.label(format!("label {}", i));
                }
            });

            if let Some(prev_state) = maybe_prev_state {
                if prev_state.offset != scroll_area_output.state.offset {
                    println!("scrolled");
                }
            }

            if let Some(pos) = ui
                .allocate_rect(scroll_area_output.inner_rect, Sense::click())
                .interact_pointer_pos()
            {
                println!("clicked {:?}", pos);
            };
        });
    }
}
