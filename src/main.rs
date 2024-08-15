use eframe::egui;
use futures::FutureExt;
use std::sync::mpsc;
use std::thread; // just used for thread::Sleep()
use std::time::Duration;

mod thread_future;

struct MyApp {
    // app data
    label_text: String,

    // communication between the UI and async tasks
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,

    // async stuff
    futures: Vec<futures::future::LocalBoxFuture<'static, ()>>,

    // observability
    frame_count: usize,
}

impl MyApp {
    fn new() -> Self {
        let label_text = "Ready".into();
        let (tx, rx) = mpsc::channel();
        let futures = Vec::new();
        let frame_count = 0;
        Self {
            label_text,
            tx,
            rx,
            futures,
            frame_count,
        }
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
            println!("frame {}", self.frame_count);
            self.frame_count += 1;

            // drive outstanding async tasks without blocking
            let mut i = 0;
            loop {
                if i >= self.futures.len() {
                    break;
                }
                let mut cx = std::task::Context::from_waker(futures::task::noop_waker_ref());
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
                let ctx = ctx.clone();
                let tx = self.tx.clone();
                self.futures.push(arbitrary_task(ctx, tx).boxed_local());
            }
        });
    }
}

async fn arbitrary_task(ctx: egui::Context, tx: mpsc::Sender<String>) {
    tx.send("Running...".into()).unwrap();
    {
        thread_future::spawn(&ctx, move || {
            // Simulate work
            thread::sleep(Duration::from_secs(1));
        })
        .await
        .unwrap();
    }
    tx.send("Still running...".into()).unwrap();
    {
        thread_future::spawn(&ctx, move || {
            // Simulate more work
            thread::sleep(Duration::from_secs(1));
        })
        .await
        .unwrap();
    }
    tx.send("Done!".into()).unwrap();
}
