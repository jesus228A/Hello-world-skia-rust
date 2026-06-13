// INDISPENSABLE: Importa el motor de eventos de Google para interactuar con el OS.
use android_activity::{AndroidApp, MainEvent, PollEvent};
// INDISPENSABLE: Importa el puntero estructurado de la pantalla física de Android.
use ndk_sys::ANativeWindow;
// INDISPENSABLE: Importa las herramientas mínimas de Skia para inyectar color y texto.
use skia_safe::{Color, Font, Paint, Surface, Typeface};
// INDISPENSABLE: Define el tipo de tiempo para que el procesador no trabaje al 100% en vacío.
use std::time::Duration;

// INDISPENSABLE: Punto de entrada obligado que busca el cargador nativo de Android.
#[no_mangle]
// INDISPENSABLE: Recibe la instancia de la aplicación que Android levantó en el cel.
fn android_main(app: AndroidApp) {
    // INDISPENSABLE: Bucle infinito para mantener la aplicación viva en la RAM.
    loop {
        // INDISPENSABLE: Lee los eventos del teléfono cada 16 milisegundos (60 FPS virtuales).
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            // INDISPENSABLE: Filtra el flujo para actuar solo cuando la pantalla ya existe.
            if let PollEvent::Main(MainEvent::InitWindow { .. }) = event {
                // INDISPENSABLE: Extrae de los objetos de Android la dirección de memoria de la ventana.
                let ptr = app.native_window().unwrap().ptr().as_ptr();
                // INDISPENSABLE: Llama al bloque inseguro de dibujo pasándole el puntero crudo.
                unsafe { pintarConSkia(ptr); }
            }
        });
    }
}

// INDISPENSABLE: Abre el bloque de C++ donde manejamos punteros físicos de memoria.
unsafe fn pintarConSkia(window: *mut ANativeWindow) {
    // INDISPENSABLE: Pregunta al OS el ancho físico en píxeles del panel del cel.
    let ancho = ndk_sys::ANativeWindow_getWidth(window);
    // INDISPENSABLE: Pregunta al OS el alto físico en píxeles del panel del cel.
    let alto = ndk_sys::ANativeWindow_getHeight(window);
    // INDISPENSABLE: Seguridad matemática; si la pantalla mide cero, aborta para no romper la RAM.
    if ancho <= 0 || alto <= 0 { return; }

    // INDISPENSABLE: Crea una estructura vacía en la pila para almacenar los datos del buffer.
    let mut buf: ndk_sys::ANativeWindow_Buffer = std::mem::zeroed();
    // INDISPENSABLE: Bloquea la pantalla para que nadie más dibuje y nos entrega sus píxeles.
    if ndk_sys::ANativeWindow_lock(window, &mut buf, std::ptr::null_mut()) != 0 { return; }

    // INDISPENSABLE: Crea la configuración de color (RGBA) y tamaño que Skia exige.
    let info = skia_safe::ImageInfo::new((ancho, alto), skia_safe::ColorType::RGBA8888, skia_safe::AlphaType::Premul, None);
    // INDISPENSABLE: Mide cuántos bytes reales ocupa una fila de píxeles en la pantalla (stride * 4).
    let salto = buf.stride as usize * 4;
    // SE CORRIGIÓ AQUÍ: Eliminamos los paréntesis pendejos para limpiar los warnings del compilador.
    let pixeles = std::slice::from_raw_parts_mut(buf.bits as *mut u8, salto * alto as usize);
    // INDISPENSABLE: Skia mapea directo la memoria física que nos prestó Android (buf.bits).
    let mut superficie = Surface::new_raster_direct(&info, pixeles, Some(salto), None).unwrap();

    // INDISPENSABLE: Extrae el lienzo activo que nos da acceso a los comandos gráficos de Skia.
    let lienzo = superficie.canvas();
    // SE QUEDA: Pinta toda la pantalla de negro para borrar residuos de memoria.
    lienzo.clear(Color::BLACK);

    // SE CORRIGIÓ AQUÍ: Forzamos el Typeface por defecto y quitamos el unwrap erróneo.
    let fuente = Font::from_typeface(Typeface::default(), 60.0);
    // SE CORRIGIÓ AQUÍ: Instanciamos una pintura mutable para poder inyectarle el color de forma legal.
    let mut pintura = Paint::default();
    // SE CORRIGIÓ AQUÍ: Modificamos el estado de la pintura usando la función correcta de la API.
    pintura.set_color(Color::RED);

    // INDISPENSABLE: Rasteriza la cadena "Hola Mundo" en las coordenadas (40, 120) usando la fuente y pintura.
    lienzo.draw_str("Hola Mundo", (40.0, 120.0), &fuente, &pintura);

    // INDISPENSABLE: Libera la pantalla de Android y manda los píxeles modificados directito al panel físico.
    ndk_sys::ANativeWindow_unlockAndPost(window);
}

