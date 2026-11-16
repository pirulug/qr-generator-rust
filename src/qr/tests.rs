use super::*;

#[test]
fn test_compute_pixel_size() {
    let settings = QrSettings {
        border: 2,
        scale_factor: 7,
        ..Default::default()
    };
    let (w, h) = QrEngine::compute_pixel_size("Mi primer codigo QR", &settings);
    assert!(w > 0);
    assert_eq!(w, h);
}

#[test]
fn test_generate_image_basic() {
    let settings = QrSettings::default();
    let img = QrEngine::generate_image("Mi primer codigo QR", &settings, None).unwrap();
    assert!(img.width() > 0);
    assert!(img.height() > 0);
}

#[test]
fn test_generate_image_with_heart_logo() {
    let settings = QrSettings {
        logo_enabled: true,
        logo_template: LogoTemplate::HeartPulse,
        logo_size_px: 80,
        ..Default::default()
    };
    let img = QrEngine::generate_image("https://rust-lang.org", &settings, None).unwrap();
    assert!(img.width() > 0);
}

#[test]
fn test_generate_svg() {
    let settings = QrSettings::default();
    let svg = QrEngine::generate_svg("https://rust-lang.org", &settings).unwrap();
    assert!(svg.starts_with("<svg"));
    assert!(svg.ends_with("</svg>"));
    assert!(svg.contains("<rect"));
}

#[test]
fn test_generate_eps() {
    let settings = QrSettings::default();
    let eps = QrEngine::generate_eps("https://rust-lang.org", &settings).unwrap();
    assert!(eps.starts_with("%!PS-Adobe-3.0 EPSF-3.0"));
    assert!(eps.contains("%%EOF"));
}

#[test]
fn test_generate_image_with_bottom_text() {
    let settings = QrSettings {
        text_enabled: true,
        text_content: "Texto de ejemplo".to_string(),
        text_color: [255, 0, 128, 255],
        text_font_size: 16.0,
        ..Default::default()
    };
    let img = QrEngine::generate_image("https://rust-lang.org", &settings, None).unwrap();
    assert!(img.height() > img.width());

    let qr_size = img.width();
    let mut found_text_pixel = false;
    for y in qr_size..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            if pixel[0] != 255 || pixel[1] != 255 || pixel[2] != 255 {
                found_text_pixel = true;
                break;
            }
        }
        if found_text_pixel {
            break;
        }
    }
    assert!(found_text_pixel, "El texto inferior debe ser renderizado en la imagen");
}

#[test]
fn test_load_ico_bytes() {
    let ico_bytes = include_bytes!("../../img/icon.ico");
    let img = image::load_from_memory_with_format(ico_bytes, image::ImageFormat::Ico).expect("Debe cargar icon.ico");
    let rgba = img.to_rgba8();
    assert!(rgba.width() > 0 && rgba.height() > 0);
}

#[test]
fn test_generate_image_with_app_icon_logo() {
    let settings = QrSettings {
        logo_enabled: true,
        logo_template: LogoTemplate::AppIcon,
        logo_size_px: 80,
        ..Default::default()
    };
    let img = QrEngine::generate_image("https://pirulug.com", &settings, None).unwrap();
    assert!(img.width() > 0);

    let svg = QrEngine::generate_svg("https://pirulug.com", &settings).unwrap();
    assert!(svg.contains("<image href=\"data:image/png;base64,"));
}

#[test]
fn test_generate_image_with_custom_logo_performance() {
    let logo = image::ImageBuffer::from_pixel(512, 512, image::Rgba([100, 150, 200, 255]));
    let settings = QrSettings {
        logo_enabled: true,
        logo_size_px: 80,
        ..Default::default()
    };
    let start = std::time::Instant::now();
    let img = QrEngine::generate_image("https://pirulug.com", &settings, Some(&logo)).unwrap();
    let elapsed = start.elapsed();
    assert!(img.width() > 0);
    assert!(elapsed.as_millis() < 100, "La generacion con logo debe ser rapida (menor a 100ms), tomo: {:?}", elapsed);
}
