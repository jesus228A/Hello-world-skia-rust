use android_activity::{AndroidApp, MainEvent, PollEvent};
use ndk_sys::ANativeWindow;
use skia_safe::{Color, Font, Paint, Surface};
use std::time::Duration;

#[no_mangle]
fn android_main(app: AndroidApp) {
    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            // Solo nos interesa cuando la ventana está lista
            if let PollEvent::Main(MainEvent::InitWindow { .. }) = event {
                let ptr = app.native_window().unwrap().ptr().as_ptr();
                unsafe { pintar_con_skia(ptr); }
            }
        });
    }
}

unsafe fn pintar_con_skia(window: *mut ANativeWindow) {
    // 1. Tamaño de la ventana
    let ancho = ndk_sys::ANativeWindow_getWidth(window);
    let alto = ndk_sys::ANativeWindow_getHeight(window);
    if ancho <= 0 || alto <= 0 { return; }

    // 2. Superficie de dibujo en memoria (fondo negro por defecto)
    let mut superficie = Surface::new_raster_n32_premul((ancho, alto)).unwrap();
    let lienzo = superficie.canvas();

    // 3. Dibujar texto con fuente por defecto y pintura roja
    let fuente = Font::new(60.0);
    let pintura = Paint::new(Color::RED, None);
    lienzo.draw_str("Hola Mundo", (40.0, 120.0), &fuente, &pintura);

    // 4. Copiar los píxeles al buffer de la ventana nativa
    let mut buf: ndk_sys::ANativeWindow_Buffer = std::mem::zeroed();
    if ndk_sys::ANativeWindow_lock(window, &mut buf, std::ptr::null_mut()) != 0 { return; }

    let imagen = superficie.image_snapshot();
    let pixeles = imagen.peek_pixels().unwrap();
    let origen = pixeles.bytes();
    let fila_origen = pixeles.row_bytes();

    let destino = buf.bits as *mut u8;
    let salto_destino = buf.stride as usize * 4; // 4 bytes por píxel

    for y in 0..alto as usize {
        let src_inicio = y * fila_origen;
        let dst_inicio = y * salto_destino;
        std::ptr::copy_nonoverlapping(
            origen.as_ptr().add(src_inicio),
            destino.add(dst_inicio),
            fila_origen,
        );
    }

    ndk_sys::ANativeWindow_unlockAndPost(window);
}
