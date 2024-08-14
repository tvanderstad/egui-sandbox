use std::future::Future;

pub struct ThreadFuture {
    join_handle: std::thread::JoinHandle<()>,
}

impl Future for ThreadFuture {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> std::task::Poll<()> {
        let this = self.get_mut();
        if this.join_handle.is_finished() {
            std::task::Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    }
}

pub trait IntoFutureExt {
    fn into_future(self) -> impl Future<Output = ()>;
}

impl IntoFutureExt for std::thread::JoinHandle<()> {
    fn into_future(self) -> ThreadFuture {
        ThreadFuture { join_handle: self }
    }
}
