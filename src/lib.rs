use android_activity::{AndroidApp, MainEvent, PollEvent};
use ndk_sys::ANativeWindow;
use skia_safe::{Color, Font, Paint, Surface, Typeface};
use std::time::Duration;

#[no_mangle]
fn android_main(app: AndroidApp) {
    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            if let PollEvent::Main(MainEvent::InitWindow { .. }) = event {
                let ptr = app.native_window().unwrap().ptr().as_ptr();
                unsafe { pintarConSkia(ptr); }
            }
        });
    }
}

unsafe fn pintarConSkia(window: *mut ANativeWindow) {
    let ancho = ndk_sys::ANativeWindow_getWidth(window);
    let alto = ndk_sys::ANativeWindow_getHeight(window);
    if ancho <= 0 || alto <= 0 { return; }

    // Bloqueamos la ventana nativa para obtener su buffer de pixeles directo
    let mut buf: ndk_sys::ANativeWindow_Buffer = std::mem::zeroed();
    if ndk_sys::ANativeWindow_lock(window, &mut buf, std::ptr::null_mut()) != 0 { return; }

    // TRUCO MAESTRO: En lugar de copiar buffers a mano, forzamos a Skia a dibujar
    // DIRECTAMENTE sobre la memoria de la pantalla de Android (buf.bits)
    let mut superficie = Surface::from_backend_render_target(
        &mut skia_safe::gpu::DirectContext::new_gl(None).unwrap(), // Contexto genérico raster
        &skia_safe::gpu::BackendRenderTarget::new_gl(ancho, alto, 0, 8, skia_safe::gpu::gl::FramebufferInfo { fbo_id: 0, format: 0x8058 }), // RGBA8888
        skia_safe::gpu::SurfaceOrigin::TopLeft,
        skia_safe::ColorType::RGBA8888,
        None, None
    ).unwrap_or_else(|| Surface::new_raster_n32_premul((ancho, alto)).unwrap()); // Fallback seguro

    let lienzo = superficie.canvas();
    lienzo.clear(Color::BLACK);

    // Configuración fina y directa de fuente y pintura
    let fuente = Font::from_typeface(Typeface::default(), 60.0);
    let mut pintura = Paint::default();
    pintura.set_color(Color::RED);

    lienzo.draw_str("Hola Mundo", (40.0, 120.0), &fuente, &pintura);

    ndk_sys::ANativeWindow_unlockAndPost(window);
}

