use android_activity::{AndroidApp, MainEvent, PollEvent};
use skia_safe::{gpu, Color, Font, Paint, Surface, Typeface};
use ndk_sys::ANativeWindow;
use std::time::Duration;

#[no_mangle]
fn android_main(app: AndroidApp) {
    let mut nativeWindow: Option<*mut ANativeWindow> = None;

    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    nativeWindow = Some(app.native_window().unwrap().ptr().as_ptr());
                    if let Some(windowPtr) = nativeWindow {
                        unsafe { pintarConSkia(windowPtr); }
                    }
                }
                // Parche de fuego: En la v0.5 el evento se llama Terminated
                PollEvent::Main(MainEvent::Terminated) => {
                    nativeWindow = None;
                }
                PollEvent::Main(MainEvent::Destroy) => return,
                _ => {}
            }
        });
    }
}

unsafe fn pintarConSkia(_window: *mut ANativeWindow) {
    // Los fierros de Skia entrarán aquí en el siguiente round, carnal
}

