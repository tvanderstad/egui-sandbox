use eframe::egui;
use futures::FutureExt;
use std::sync::mpsc;
use std::thread; // just used for thread::Sleep()
use std::time::Duration;
use thread_future::IntoFutureExt as IntoFuture;

mod repaint_waker;
mod thread_future;

struct MyApp {
    // app data
    label_text: String,

    // communication between the UI and async tasks
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,

    // async stuff
    waker: std::task::Waker,
    futures: Vec<futures::future::LocalBoxFuture<'static, ()>>,
}

impl MyApp {
    fn new(ctx: egui::Context) -> Self {
        let label_text = "Ready".into();
        let (tx, rx) = mpsc::channel();
        let futures = Vec::new();
        let waker = repaint_waker::create(ctx);
        Self {
            label_text,
            tx,
            rx,
            futures,
            waker,
        }
    }
}

fn main() {
    eframe::run_native(
        "test",
        eframe::NativeOptions::default(),
        Box::new(|cc: &eframe::CreationContext| Ok(Box::new(MyApp::new(cc.egui_ctx.clone())))),
    )
    .unwrap();
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // drive outstanding async tasks without blocking
            let mut i = 0;
            loop {
                if i >= self.futures.len() {
                    break;
                }
                let mut cx = std::task::Context::from_waker(&self.waker);
                match self.futures[i].poll_unpin(&mut cx) {
                    std::task::Poll::Ready(()) => {
                        drop(self.futures.remove(i));
                    }
                    std::task::Poll::Pending => {
                        i += 1;
                    }
                }
            }

            // process any messages from the async tasks
            if let Ok(msg) = self.rx.try_recv() {
                self.label_text = msg;
            }

            // show the ui
            ui.label(&self.label_text);
            if ui.button("Button").clicked() {
                let tx = self.tx.clone();
                self.futures.push(button_clicked(tx).boxed_local());
            }
        });
    }
}

async fn button_clicked(tx: mpsc::Sender<String>) {
    tx.send("Running...".into()).unwrap();
    {
        thread::spawn(move || {
            // Simulate more work
            thread::sleep(Duration::from_secs(1));
        })
        .into_future()
        .await;
    }
    tx.send("Still running...".into()).unwrap();
    {
        thread::spawn(move || {
            // Simulate more work
            thread::sleep(Duration::from_secs(1));
        })
        .into_future()
        .await;
    }
    tx.send("Done!".into()).unwrap();
}
