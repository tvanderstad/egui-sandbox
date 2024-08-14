use std::{future::Future, thread};

use eframe::egui;

pub struct ThreadFuture {
    join_handle: std::thread::JoinHandle<()>,
}

pub fn spawn<F: FnOnce() -> () + Send + 'static>(ctx: &egui::Context, f: F) -> ThreadFuture {
    let ctx = ctx.clone();
    ThreadFuture {
        join_handle: thread::spawn(move || {
            f();
            ctx.request_repaint();
        }),
    }
}

impl Future for ThreadFuture {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context) -> std::task::Poll<()> {
        let this = self.get_mut();
        if this.join_handle.is_finished() {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}
