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
                PollEvent::Main(MainEvent::InitWindow) => {
                    nativeWindow = Some(app.native_window().unwrap().ptr().as_ptr());
                    if let Some(windowPtr) = nativeWindow {
                        unsafe { pintarConSkia(windowPtr); }
                    }
                }
                PollEvent::Main(MainEvent::DestroyWindow) => {
                    nativeWindow = None;
                }
                PollEvent::Main(MainEvent::Terminate) => return,
                _ => {}
            }
        });
    }
}

unsafe fn pintarConSkia(window: *mut ANativeWindow) {
    // Aquí va tu lógica fina de renderizado con Skia
    // Por ahora un cascarón limpio para que compile al tiro
}

