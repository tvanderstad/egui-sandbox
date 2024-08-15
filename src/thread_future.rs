use std::{future::Future, thread};

use eframe::egui;

pub struct ThreadFuture<T> {
    join_handle: Option<std::thread::JoinHandle<T>>,
}

pub fn spawn<F, T>(ctx: &egui::Context, f: F) -> ThreadFuture<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let ctx = ctx.clone();
    ThreadFuture {
        join_handle: Some(thread::spawn(move || {
            let result = f();
            ctx.request_repaint();
            result
        })),
    }
}

impl<T> Future for ThreadFuture<T> {
    type Output = std::thread::Result<T>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        if let Some(join_handle) = this.join_handle.take() {
            if join_handle.is_finished() {
                let result = join_handle.join(); // consume the join handle
                std::task::Poll::Ready(result)
            } else {
                this.join_handle = Some(join_handle); // put the join handle back
                std::task::Poll::Pending
            }
        } else {
            panic!("future polled after moving result")
        }
    }
}
