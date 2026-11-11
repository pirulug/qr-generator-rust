#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod qr;

use app::QrApp;
use eframe::egui;
use qr::{QrEngine, QrSettings};

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Modo CLI opcional si se pasan argumentos por consola
    if args.len() > 1 {
        let first = &args[1];
        if first == "--help" || first == "-h" {
            println!("=== QR Generator (Edicion Rust) ===");
            println!("Uso:");
            println!("  cargo run                      (Inicia la interfaz gráfica completa en español)");
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

        println!("Generando código QR para: {}", payload);
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

    // Modo GUI Nativo de Escritorio (en espanol, identico a las capturas)
    let icon_data = {
        let ico_bytes = include_bytes!("../img/icon.ico");
        match image::load_from_memory_with_format(ico_bytes, image::ImageFormat::Ico) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (width, height) = (rgba.width(), rgba.height());
                Some(std::sync::Arc::new(egui::IconData {
                    rgba: rgba.into_raw(),
                    width,
                    height,
                }))
            }
            Err(_) => None,
        }
    };

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([820.0, 780.0])
        .with_min_inner_size([650.0, 680.0])
        .with_title("QR Generator v0.0.1 (Windows) - Modulo Logo II");

    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "QR Generator v0.0.1 (Windows) - Modulo Logo II",
        native_options,
        Box::new(|cc| Ok(Box::new(QrApp::new(cc)))),
    )
}
