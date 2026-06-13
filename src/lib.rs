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
                // Parche 1: Se usan llaves y .. para ignorar los datos internos del InitWindow
                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    nativeWindow = Some(app.native_window().unwrap().ptr().as_ptr());
                    if let Some(windowPtr) = nativeWindow {
                        unsafe { pintarConSkia(windowPtr); }
                    }
                }
                // Parche 2: En la v0.5 se llama WindowDestroyed
                PollEvent::Main(MainEvent::WindowDestroyed { .. }) => {
                    nativeWindow = None;
                }
                // Parche 3: En la v0.5 se llama Destroy
                PollEvent::Main(MainEvent::Destroy) => return,
                _ => {}
            }
        });
    }
}

// Parche 4: Le ponemos guion bajo a _window para silenciar el warning si aún no pintas nada
unsafe fn pintarConSkia(_window: *mut ANativeWindow) {
    // Aquí va a ir tu magia pesada con Skia más adelante, carnal
}

