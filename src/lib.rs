use android_activity::{AndroidApp, MainEvent, PollEvent};
use skia_safe::{backend_render_targets, gpu, Color, Font, Paint, Surface, Typeface};
use ndk_sys::ANativeWindow;

#[no_mangle]
fn android_main(app: AndroidApp) {
    // El Gerente inicializa y se queda esperando a que SurfaceFlinger cree la ventana
    let mut ventana_nativa: Option<*mut ANativeWindow> = None;

    loop {
        // El ciclo de eventos del sistema (El Looper)
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::InitWindow) => {
                    // ¡Cámaras! Android nos dice que la ventana física ya existe
                    ventana_nativa = app.native_window().map(|w| w.as_raw());
                    
                    if let Some(window) = ventana_nativa {
                        unsafe { pintar_con_skia(window); }
                    }
                }
                PollEvent::Main(MainEvent::DestroyWindow) => {
                    ventana_nativa = None;
                }
                _ => {}
            }
        });
    }
}

unsafe fn pintar_con_skia(window: *mut ANativeWindow) {
    // 1. LOCK: Gralloc nos da acceso al buffer de la pantalla
    let mut buffer = ndk_sys::ANativeWindow_Buffer {
        width: 0, height: 0, stride: 0, format: 0, bits: std::ptr::null_mut(), reserved: [0; 6],
    };
    
    if ndk_sys::ANativeWindow_lock(window, &mut buffer, std::ptr::null_mut()) == 0 {
        let ancho = buffer.width;
        let alto = buffer.height;

        // 2. MAQUINISTA: Creamos un lienzo de Skia apuntando directo a la RAM de la pantalla
        let info = skia_safe::ImageInfo::new_n32_premul((ancho, alto), None);
        let mut surface = Surface::from_raster_direct(
            &info,
            buffer.bits,
            (buffer.stride * 4) as usize, // 4 bytes por píxel (RGBA)
            None,
        ).unwrap();

        let canvas = surface.canvas();

        // 3. LAS ÓRDENES GRÁFICAS
        canvas.clear(Color::from_argb(255, 20, 20, 20)); // Fondo gris obscuro

        let mut pintura = Paint::default();
        pintura.set_color(Color::from_argb(255, 0, 255, 0)); // Texto Verde Cholo / Matrix
        pintura.set_anti_alias(true);

        let fuente = Font::from_typeface(Typeface::default(), 64.0);
        
        // Pintamos el texto en el lienzo
        canvas.draw_str("Hola Mundo desde Skia Nativo", (100.0, 300.0), &fuente, &pintura);

        // Fuerza a Skia a terminar de calcar los bytes en la RAM
        canvas.flush(); 

        // 4. UNLOCK: Le regresamos el control a SurfaceFlinger para que lo mande al monitor
        ndk_sys::ANativeWindow_unlockAndPost(window);
    }
}

use android_activity::{AndroidApp, MainEvent, PollEvent};
use skia_safe::{backend_render_targets, gpu, Color, Font, Paint, Surface, Typeface};
use ndk_sys::ANativeWindow;

#[no_mangle]
fn android_main(app: AndroidApp) {
    // El Gerente inicializa y se queda esperando a que SurfaceFlinger cree la ventana
    let mut ventana_nativa: Option<*mut ANativeWindow> = None;

    loop {
        // El ciclo de eventos del sistema (El Looper)
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::InitWindow) => {
                    // ¡Cámaras! Android nos dice que la ventana física ya existe
                    ventana_nativa = app.native_window().map(|w| w.as_raw());
                    
                    if let Some(window) = ventana_nativa {
                        unsafe { pintar_con_skia(window); }
                    }
                }
                PollEvent::Main(MainEvent::DestroyWindow) => {
                    ventana_nativa = None;
                }
                _ => {}
            }
        });
    }
}

unsafe fn pintar_con_skia(window: *mut ANativeWindow) {
    // 1. LOCK: Gralloc nos da acceso al buffer de la pantalla
    let mut buffer = ndk_sys::ANativeWindow_Buffer {
        width: 0, height: 0, stride: 0, format: 0, bits: std::ptr::null_mut(), reserved: [0; 6],
    };
    
    if ndk_sys::ANativeWindow_lock(window, &mut buffer, std::ptr::null_mut()) == 0 {
        let ancho = buffer.width;
        let alto = buffer.height;

        // 2. MAQUINISTA: Creamos un lienzo de Skia apuntando directo a la RAM de la pantalla
        let info = skia_safe::ImageInfo::new_n32_premul((ancho, alto), None);
        let mut surface = Surface::from_raster_direct(
            &info,
            buffer.bits,
            (buffer.stride * 4) as usize, // 4 bytes por píxel (RGBA)
            None,
        ).unwrap();

        let canvas = surface.canvas();

        // 3. LAS ÓRDENES GRÁFICAS
        canvas.clear(Color::from_argb(255, 20, 20, 20)); // Fondo gris obscuro

        let mut pintura = Paint::default();
        pintura.set_color(Color::from_argb(255, 0, 255, 0)); // Texto Verde Cholo / Matrix
        pintura.set_anti_alias(true);

        let fuente = Font::from_typeface(Typeface::default(), 64.0);
        
        // Pintamos el texto en el lienzo
        canvas.draw_str("Hola Mundo desde Skia Nativo", (100.0, 300.0), &fuente, &pintura);

        // Fuerza a Skia a terminar de calcar los bytes en la RAM
        canvas.flush(); 

        // 4. UNLOCK: Le regresamos el control a SurfaceFlinger para que lo mande al monitor
        ndk_sys::ANativeWindow_unlockAndPost(window);
    }
}

