use eframe::egui;
use std::task::{RawWaker, RawWakerVTable, Waker};

struct RepaintWaker {
    ctx: egui::Context,
}

fn clone_waker(data: *const ()) -> RawWaker {
    let waker = unsafe { &*(data as *const RepaintWaker) };
    let cloned = Box::new(RepaintWaker {
        ctx: waker.ctx.clone(),
    });
    RawWaker::new(Box::into_raw(cloned) as *const (), &VTABLE)
}

fn wake_waker(data: *const ()) {
    let waker = unsafe { &*(data as *const RepaintWaker) };

    waker.ctx.request_repaint();
}

fn wake_by_ref_waker(data: *const ()) {
    let waker = unsafe { &*(data as *const RepaintWaker) };

    waker.ctx.request_repaint();
}

fn drop_waker(data: *const ()) {
    unsafe {
        drop(Box::from_raw(
            data as *const RepaintWaker as *mut RepaintWaker,
        ))
    };
}

const VTABLE: RawWakerVTable =
    RawWakerVTable::new(clone_waker, wake_waker, wake_by_ref_waker, drop_waker);

pub fn create(ctx: egui::Context) -> Waker {
    // Store the context in a Box so we can convert it to a raw pointer
    let waker_data = Box::new(RepaintWaker { ctx });
    let raw_waker = RawWaker::new(Box::into_raw(waker_data) as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}
