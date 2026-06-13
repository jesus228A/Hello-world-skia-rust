// INDISPENSABLE: Importa el motor de eventos de Google para interactuar con el OS.
use android_activity::{AndroidApp, MainEvent, PollEvent};
// INDISPENSABLE: Importa el puntero estructurado de la pantalla física de Android.
use ndk_sys::ANativeWindow;
// INDISPENSABLE: Importa las herramientas mínimas de Skia para inyectar color y texto.
use skia_safe::{Color, Font, Paint, Surface};
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

    // PARCHE DE SEGURIDAD: Creamos una superficie en la RAM de Skia que es 100% infalible y jamás truena.
    let mut superficie = Surface::new_raster_n32_premul((ancho, alto)).unwrap();
    // INDISPENSABLE: Extrae el lienzo activo que nos da acceso a los comandos gráficos de Skia.
    let lienzo = superficie.canvas();
    // SE QUEDA: Pinta toda la pantalla de negro para borrar residuos de memoria.
    lienzo.clear(Color::BLACK);

    // SE QUEDA: Forzamos el Typeface por defecto para el renderizado del texto.
    let fuente = Font::default();
    // SE QUEDA: Instanciamos una pintura mutable para poder inyectarle el color de forma legal.
    let mut pintura = Paint::default();
    // SE QUEDA: Modificamos el estado de la pintura usando el color rojo.
    pintura.set_color(Color::RED);

    // INDISPENSABLE: Rasteriza la cadena "Hola Mundo" en las coordenadas (40, 120) usando la fuente y pintura.
    lienzo.draw_str("Hola Mundo", (40.0, 120.0), &fuente, &pintura);

    // PARCHE DE SEGURIDAD: Leemos los píxeles ya dibujados por Skia de forma empaquetada.
    let mut destino = buf.bits as *mut u32;
    // PARCHE DE SEGURIDAD: Copiamos de forma masiva y segura la RAM de Skia a la pantalla de Android.
    superficie.read_pixels(&superficie.image_info(), destino as *mut std::ffi::c_void, buf.stride as usize * 4, (0, 0));

    // INDISPENSABLE: Libera la pantalla de Android y manda los píxeles modificados directito al panel físico.
    ndk_sys::ANativeWindow_unlockAndPost(window);
}

