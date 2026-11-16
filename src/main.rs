#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_must_use, dead_code, unused_imports)]

mod qr;
mod ui;

use qr::{QrEngine, QrSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // Modo CLI opcional si se pasan argumentos por consola
    if args.len() > 1 {
        let first = &args[1];
        if first == "--help" || first == "-h" {
            println!("=== QR Generator (Edicion Rust) ===");
            println!("Uso:");
            println!("  cargo run                      (Inicia la interfaz grafica nativa Windows en espanol)");
            println!("  cargo run -- <texto_o_url> [archivo_salida.png]");
            println!("Ejemplo:");
            println!("  cargo run -- \"https://rust-lang.org\" codigo_qr.png");
            return Ok(());
        }

        let payload = first;
        let output_path = if args.len() > 2 {
            &args[2]
        } else {
            "codigo_qr.png"
        };

        println!("Generando codigo QR para: {}", payload);
        let settings = QrSettings::default();

        match QrEngine::generate_image(payload, &settings, None) {
            Ok(img) => {
                if let Err(e) = img.save(output_path) {
                    eprintln!("Error al guardar imagen {}: {}", output_path, e);
                } else {
                    println!("Imagen guardada exitosamente en {}", output_path);
                }
            }
            Err(e) => {
                eprintln!("Error al generar QR: {}", e);
            }
        }
        return Ok(());
    }

    // Modo GUI Nativo de Escritorio con windows-rs (puro Win32)
    ui::run_windows_app()?;
    Ok(())
}
