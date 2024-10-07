use eframe::egui::{self, CentralPanel, EventFilter, Key, TextEdit};

#[derive(Default)]
struct MyApp {
    find: Option<Find>,
    text: String,
}

struct Find {
    term: String,
}

fn main() {
    eframe::run_native(
        "test",
        eframe::NativeOptions::default(),
        Box::new(|_cc: &eframe::CreationContext| Ok(Box::new(MyApp::default()))),
    )
    .unwrap();
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Find-in-Document toolbar
            // * cmd+f while find closed: open find
            // * cmd+f while find open and unfocused: focus find
            // * cmd+f while find focused: close find
            // * enter while find focused: find next
            // * shift+enter while find focused: find previous
            // * esc while find focused: close find
            let mut search_term_response = None;
            if self.find.is_some() {
                ui.horizontal(|ui| {
                    ui.label("Seach document:");
                    let resp = if let Some(find) = &mut self.find {
                        ui.add(
                            TextEdit::singleline(&mut find.term)
                                .desired_width(ui.available_width())
                                .hint_text("Type here to search"),
                        )
                    } else {
                        unreachable!()
                    };
                    if ctx.input(|i| i.key_pressed(Key::Enter) && !i.modifiers.shift) {
                        println!("find next");
                    }
                    if ctx.input(|i| i.key_pressed(Key::Enter) && i.modifiers.shift) {
                        println!("find previous");
                    }
                    if ctx.input(|i| i.key_pressed(Key::Escape)) && resp.has_focus() {
                        self.find = None;
                        ui.ctx().request_repaint();
                    }

                    // request focus on the frame it appears
                    if ui.memory(|m| m.area_rect(resp.id)).is_none() {
                        resp.request_focus();
                    }

                    if resp.has_focus() {
                        ui.memory_mut(|m| {
                            m.set_focus_lock_filter(
                                resp.id,
                                EventFilter {
                                    tab: true,
                                    horizontal_arrows: true,
                                    vertical_arrows: true,
                                    escape: true,
                                },
                            )
                        })
                    }

                    search_term_response = Some(resp);
                });
            };
            if ctx.input(|i| i.key_pressed(Key::F) && i.modifiers.command) {
                if self.find.is_none() {
                    self.find = Some(Find {
                        term: String::new(),
                    });
                } else {
                    let search_term_response = search_term_response.unwrap();
                    if search_term_response.has_focus() {
                        self.find = None;
                    } else {
                        search_term_response.request_focus();
                    }
                }
                ui.ctx().request_repaint();
            }

            // Main text editor
            let text_edit_response =
                ui.add(TextEdit::multiline(&mut self.text).desired_width(ui.available_width()));
            if ui.memory(|m| m.focused().is_none()) {
                text_edit_response.request_focus();
                ui.ctx().request_repaint();
            }
        });
    }
}
