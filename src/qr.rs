use image::{ImageBuffer, Rgba};
use qrcode::{types::QrError, Color, EcLevel, QrCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCorrection {
    Low,
    Medium,
    Quartile,
    High,
}

impl ErrorCorrection {
    pub fn to_ec_level(self) -> EcLevel {
        match self {
            Self::Low => EcLevel::L,
            Self::Medium => EcLevel::M,
            Self::Quartile => EcLevel::Q,
            Self::High => EcLevel::H,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Low => "Bajo (L)",
            Self::Medium => "Medio (M)",
            Self::Quartile => "Cuartil (Q)",
            Self::High => "Alto (H)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogoTemplate {
    None,
    HeartPulse,
    Star,
    Shield,
    Wifi,
    TwitterX,
    UserContact,
}

impl LogoTemplate {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "---",
            Self::HeartPulse => "Corazón / Pulso",
            Self::Star => "Estrella",
            Self::Shield => "Escudo",
            Self::Wifi => "Símbolo Wi-Fi",
            Self::TwitterX => "X (Twitter)",
            Self::UserContact => "Avatar de contacto",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextCasing {
    TextField,
    Small,
    Great,
}

#[derive(Clone, Debug)]
pub struct QrSettings {
    // Matrix settings
    pub ec_level: ErrorCorrection,
    pub border: usize,       // 0..8
    pub scale_factor: usize, // 2..15
    pub fg_color: [u8; 4],   // default: black [0, 0, 0, 255]
    pub bg_color: [u8; 4],   // default: white [255, 255, 255, 255]

    // Logo settings
    pub logo_enabled: bool,
    pub logo_template: LogoTemplate,
    pub logo_size_px: u32,
    pub keep_centered_enlarge: bool,
    pub check_validity: bool,
    pub logo_offset_x: i32,
    pub logo_offset_y: i32,
    pub logo_pixel_per_click: i32,

    // Text settings
    pub text_enabled: bool,
    pub text_content: String,
    pub text_font: String,
    pub _text_icon: String,
    pub text_casing: TextCasing,
    pub text_bold: bool,
    pub text_italic: bool,
    pub text_underline: bool,
    pub text_color: [u8; 4],
    pub text_font_size: f32,
    pub text_offset_x: i32,
    pub text_offset_y: i32,
    pub text_pixel_per_click: i32,
}

impl Default for QrSettings {
    fn default() -> Self {
        Self {
            ec_level: ErrorCorrection::Low,
            border: 2,
            scale_factor: 7,
            fg_color: [0, 0, 0, 255],         // Black (h00000000)
            bg_color: [255, 255, 255, 255], // White (h00FFFFFF)

            logo_enabled: false,
            logo_template: LogoTemplate::HeartPulse,
            logo_size_px: 100,
            keep_centered_enlarge: true,
            check_validity: true,
            logo_offset_x: 0,
            logo_offset_y: 0,
            logo_pixel_per_click: 5,

            text_enabled: false,
            text_content: "Texto de ejemplo".to_string(),
            text_font: "Predeterminada".to_string(),
            _text_icon: "👉 - Dedo apuntando a la derecha".to_string(),
            text_casing: TextCasing::TextField,
            text_bold: true,
            text_italic: false,
            text_underline: false,
            text_color: [255, 0, 128, 255], // Pink / Magenta h00FF80FF
            text_font_size: 16.0,
            text_offset_x: 0,
            text_offset_y: 0,
            text_pixel_per_click: 5,
        }
    }
}

pub struct QrEngine;

impl QrEngine {
    /// Calcula el tamaño total de la matriz incluyendo bordes y escala
    pub fn compute_pixel_size(payload: &str, settings: &QrSettings) -> (u32, u32) {
        let code = match QrCode::with_error_correction_level(payload.as_bytes(), settings.ec_level.to_ec_level()) {
            Ok(c) => c,
            Err(_) => return (350, 350),
        };
        let matrix_size = code.width() as u32;
        let total_modules = matrix_size + (settings.border as u32 * 2);
        let px = total_modules * (settings.scale_factor as u32);
        (px, px)
    }

    /// Genera la imagen completa RGBA del código QR
    pub fn generate_image(
        payload: &str,
        settings: &QrSettings,
        custom_logo: Option<&ImageBuffer<Rgba<u8>, Vec<u8>>>,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, QrError> {
        let code = QrCode::with_error_correction_level(payload.as_bytes(), settings.ec_level.to_ec_level())?;
        let modules = code.to_colors();
        let matrix_size = code.width();

        let total_modules = matrix_size + settings.border * 2;
        let qr_px_size = (total_modules * settings.scale_factor) as u32;

        // Si el texto bajo el QR está habilitado, añade una franja inferior
        let text_height = if settings.text_enabled && !settings.text_content.trim().is_empty() {
            (settings.text_font_size * 2.2).max(36.0) as u32
        } else {
            0
        };

        let total_width = qr_px_size;
        let total_height = qr_px_size + text_height;

        let bg_pixel = Rgba(settings.bg_color);
        let fg_pixel = Rgba(settings.fg_color);

        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_pixel(total_width, total_height, bg_pixel);

        // Renderizar módulos oscuros
        for y in 0..matrix_size {
            for x in 0..matrix_size {
                if modules[y * matrix_size + x] == Color::Dark {
                    let start_x = ((x + settings.border) * settings.scale_factor) as u32;
                    let start_y = ((y + settings.border) * settings.scale_factor) as u32;

                    for dy in 0..settings.scale_factor as u32 {
                        for dx in 0..settings.scale_factor as u32 {
                            if start_x + dx < total_width && start_y + dy < total_height {
                                img.put_pixel(start_x + dx, start_y + dy, fg_pixel);
                            }
                        }
                    }
                }
            }
        }

        // Renderizar Logo si está habilitado
        if settings.logo_enabled {
            let logo_size = settings.logo_size_px.clamp(20, qr_px_size.saturating_sub(40));
            let center_x = ((qr_px_size / 2) as i32 + settings.logo_offset_x).max(logo_size as i32 / 2) as u32;
            let center_y = ((qr_px_size / 2) as i32 + settings.logo_offset_y).max(logo_size as i32 / 2) as u32;
            let half_logo = logo_size / 2;

            // Fondo protector blanco redondeado detrás del logo
            let badge_padding = (logo_size as f32 * 0.12).round() as u32;
            let badge_half = half_logo + badge_padding;

            let min_x = center_x.saturating_sub(badge_half);
            let max_x = (center_x + badge_half).min(qr_px_size);
            let min_y = center_y.saturating_sub(badge_half);
            let max_y = (center_y + badge_half).min(qr_px_size);

            for py in min_y..max_y {
                for px in min_x..max_x {
                    let dx = px as f32 - center_x as f32;
                    let dy = py as f32 - center_y as f32;
                    if dx * dx + dy * dy <= (badge_half as f32) * (badge_half as f32) {
                        img.put_pixel(px, py, bg_pixel);
                    }
                }
            }

            if let Some(logo) = custom_logo {
                // Logo personalizado cargado desde archivo
                let resized = image::imageops::resize(
                    logo,
                    logo_size,
                    logo_size,
                    image::imageops::FilterType::Lanczos3,
                );
                let start_x = center_x.saturating_sub(half_logo);
                let start_y = center_y.saturating_sub(half_logo);

                for (lx, ly, pixel) in resized.enumerate_pixels() {
                    let tx = start_x + lx;
                    let ty = start_y + ly;
                    if tx < qr_px_size && ty < qr_px_size && pixel[3] > 0 {
                        let alpha = pixel[3] as f32 / 255.0;
                        let bg = img.get_pixel(tx, ty);
                        let r = (pixel[0] as f32 * alpha + bg[0] as f32 * (1.0 - alpha)) as u8;
                        let g = (pixel[1] as f32 * alpha + bg[1] as f32 * (1.0 - alpha)) as u8;
                        let b = (pixel[2] as f32 * alpha + bg[2] as f32 * (1.0 - alpha)) as u8;
                        img.put_pixel(tx, ty, Rgba([r, g, b, 255]));
                    }
                }
            } else {
                // Plantilla de logo integrada
                match settings.logo_template {
                    LogoTemplate::HeartPulse => {
                        Self::draw_heart_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::Star => {
                        Self::draw_star_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::Shield => {
                        Self::draw_shield_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::Wifi => {
                        Self::draw_wifi_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::TwitterX => {
                        Self::draw_twitter_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::UserContact => {
                        Self::draw_user_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::None => {}
                }
            }
        }

        Ok(img)
    }

    /// Dibuja el logo de corazón con onda de pulso (idéntico a la imagen original)
    pub fn draw_heart_logo(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cx: u32, cy: u32, size: u32) {
        let red = Rgba([239, 68, 68, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let radius = size as f32 / 2.0;

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let px = (cx as i32 + dx) as u32;
                let py = (cy as i32 + dy) as u32;

                if px >= img.width() || py >= img.height() {
                    continue;
                }

                let nx = dx as f32 / (radius * 0.85);
                let ny = -dy as f32 / (radius * 0.85);

                let term = nx * nx + ny * ny - 0.8;
                let heart_val = term * term * term - nx * nx * ny * ny * ny;

                if heart_val <= 0.0 {
                    img.put_pixel(px, py, red);
                }
            }
        }

        // Línea de pulso blanco
        let pulse_w = (radius * 0.65) as i32;
        let line_thickness = (size as f32 * 0.07).max(2.0) as i32;

        for dx in -pulse_w..=pulse_w {
            let rel = dx as f32 / pulse_w as f32;
            let offset_y = if rel > -0.25 && rel < -0.05 {
                (rel + 0.15) * 16.0 * (radius * 0.25)
            } else if rel >= -0.05 && rel < 0.1 {
                -(rel - 0.025) * 18.0 * (radius * 0.3)
            } else if rel >= 0.1 && rel < 0.25 {
                (rel - 0.175) * 12.0 * (radius * 0.2)
            } else {
                0.0
            };

            let py = (cy as f32 + offset_y).round() as i32;
            let px = cx as i32 + dx;

            for t in -(line_thickness / 2)..=(line_thickness / 2) {
                let final_py = (py + t) as u32;
                let final_px = px as u32;
                if final_px < img.width() && final_py < img.height() {
                    img.put_pixel(final_px, final_py, white);
                }
            }
        }
    }

    /// Dibuja un logo de estrella dorada
    pub fn draw_star_logo(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cx: u32, cy: u32, size: u32) {
        let gold = Rgba([245, 158, 11, 255]);
        let r_outer = size as f32 * 0.45;
        let r_inner = r_outer * 0.4;

        for dy in -(r_outer as i32)..=(r_outer as i32) {
            for dx in -(r_outer as i32)..=(r_outer as i32) {
                let px = (cx as i32 + dx) as u32;
                let py = (cy as i32 + dy) as u32;
                if px >= img.width() || py >= img.height() {
                    continue;
                }

                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                let angle = (-(dy as f32)).atan2(dx as f32);
                let angle_deg = (angle.to_degrees() + 360.0 + 90.0) % 360.0;
                let point_angle = (angle_deg % 72.0) - 36.0;
                let max_r = r_inner + (r_outer - r_inner) * (1.0 - (point_angle.to_radians().cos().abs() * 0.5));

                if dist <= max_r {
                    img.put_pixel(px, py, gold);
                }
            }
        }
    }

    /// Dibuja un logo de escudo azul
    pub fn draw_shield_logo(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cx: u32, cy: u32, size: u32) {
        let blue = Rgba([37, 99, 235, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let half = (size as f32 * 0.4) as i32;

        for dy in -half..=half {
            for dx in -half..=half {
                let px = (cx as i32 + dx) as u32;
                let py = (cy as i32 + dy) as u32;
                if px >= img.width() || py >= img.height() {
                    continue;
                }
                let norm_x = dx.abs() as f32 / half as f32;
                let norm_y = dy as f32 / half as f32;
                if norm_x <= 1.0 && (norm_y <= 0.2 || (norm_y > 0.2 && norm_x <= 1.0 - (norm_y - 0.2) * 0.8)) {
                    img.put_pixel(px, py, blue);
                }
            }
        }

        // Marca de verificación blanca dentro del escudo
        let check_size = (size as f32 * 0.15) as i32;
        for t in -check_size..=check_size {
            let px = (cx as i32 + t) as u32;
            let py = (cy as i32 + t.abs() / 2) as u32;
            if px < img.width() && py < img.height() {
                img.put_pixel(px, py, white);
            }
        }
    }

    /// Dibuja el icono de WiFi
    pub fn draw_wifi_logo(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cx: u32, cy: u32, size: u32) {
        let green = Rgba([16, 185, 129, 255]);
        let radius = size as f32 * 0.4;

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let px = (cx as i32 + dx) as u32;
                let py = (cy as i32 + dy) as u32;
                if px >= img.width() || py >= img.height() {
                    continue;
                }

                let d = ((dx * dx + dy * dy) as f32).sqrt();
                if dy <= 0 {
                    let in_band1 = (d - radius * 0.9).abs() < 2.5;
                    let in_band2 = (d - radius * 0.6).abs() < 2.5;
                    let in_band3 = (d - radius * 0.3).abs() < 2.5;
                    if in_band1 || in_band2 || in_band3 {
                        img.put_pixel(px, py, green);
                    }
                } else if d < 4.0 {
                    img.put_pixel(px, py, green);
                }
            }
        }
    }

    /// Dibuja el icono de X (Twitter)
    pub fn draw_twitter_logo(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cx: u32, cy: u32, size: u32) {
        let dark = Rgba([15, 20, 25, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let half = (size as f32 * 0.42) as i32;

        for dy in -half..=half {
            for dx in -half..=half {
                let px = (cx as i32 + dx) as u32;
                let py = (cy as i32 + dy) as u32;
                if px >= img.width() || py >= img.height() {
                    continue;
                }
                if dx * dx + dy * dy <= half * half {
                    img.put_pixel(px, py, dark);
                }
            }
        }

        let cross_size = (size as f32 * 0.22) as i32;
        for i in -cross_size..=cross_size {
            for t in -1..=1 {
                let px1 = (cx as i32 + i + t) as u32;
                let py1 = (cy as i32 + i) as u32;
                let px2 = (cx as i32 + i + t) as u32;
                let py2 = (cy as i32 - i) as u32;
                if px1 < img.width() && py1 < img.height() {
                    img.put_pixel(px1, py1, white);
                }
                if px2 < img.width() && py2 < img.height() {
                    img.put_pixel(px2, py2, white);
                }
            }
        }
    }

    /// Dibuja el avatar de usuario
    pub fn draw_user_logo(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cx: u32, cy: u32, size: u32) {
        let blue = Rgba([59, 130, 246, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let half = (size as f32 * 0.42) as i32;

        for dy in -half..=half {
            for dx in -half..=half {
                let px = (cx as i32 + dx) as u32;
                let py = (cy as i32 + dy) as u32;
                if px >= img.width() || py >= img.height() {
                    continue;
                }
                if dx * dx + dy * dy <= half * half {
                    img.put_pixel(px, py, blue);
                }
            }
        }

        // Cabeza
        let head_r = (size as f32 * 0.12) as i32;
        let head_cy = cy as i32 - (size as f32 * 0.1) as i32;
        for dy in -head_r..=head_r {
            for dx in -head_r..=head_r {
                if dx * dx + dy * dy <= head_r * head_r {
                    let px = (cx as i32 + dx) as u32;
                    let py = (head_cy + dy) as u32;
                    if px < img.width() && py < img.height() {
                        img.put_pixel(px, py, white);
                    }
                }
            }
        }
    }

    /// Genera código SVG vectorial con soporte para texto y logo
    pub fn generate_svg(payload: &str, settings: &QrSettings) -> Result<String, QrError> {
        let code = QrCode::with_error_correction_level(payload.as_bytes(), settings.ec_level.to_ec_level())?;
        let modules = code.to_colors();
        let matrix_size = code.width();
        let total_modules = matrix_size + settings.border * 2;
        let scale = settings.scale_factor;
        let width = total_modules * scale;

        let has_text = settings.text_enabled && !settings.text_content.trim().is_empty();
        let text_height = if has_text { (settings.text_font_size * 2.2) as usize } else { 0 };
        let height = width + text_height;

        let bg_hex = format!("#{:02X}{:02X}{:02X}", settings.bg_color[0], settings.bg_color[1], settings.bg_color[2]);
        let fg_hex = format!("#{:02X}{:02X}{:02X}", settings.fg_color[0], settings.fg_color[1], settings.fg_color[2]);
        let txt_hex = format!("#{:02X}{:02X}{:02X}", settings.text_color[0], settings.text_color[1], settings.text_color[2]);

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
            width, height, width, height
        ));
        svg.push_str(&format!(
            r#"<rect width="{}" height="{}" fill="{}" />"#,
            width, height, bg_hex
        ));

        // Módulos
        for y in 0..matrix_size {
            for x in 0..matrix_size {
                if modules[y * matrix_size + x] == Color::Dark {
                    let rx = (x + settings.border) * scale;
                    let ry = (y + settings.border) * scale;
                    svg.push_str(&format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"#,
                        rx, ry, scale, scale, fg_hex
                    ));
                }
            }
        }

        // Logo badge si está activo
        if settings.logo_enabled {
            let logo_size = settings.logo_size_px as usize;
            let center_x = ((width / 2) as i32 + settings.logo_offset_x).max(logo_size as i32 / 2) as usize;
            let center_y = ((width / 2) as i32 + settings.logo_offset_y).max(logo_size as i32 / 2) as usize;
            let badge_size = (logo_size as f32 * 1.25) as usize;
            let _badge_x = center_x.saturating_sub(badge_size / 2);
            let _badge_y = center_y.saturating_sub(badge_size / 2);

            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="{}" />"#,
                center_x, center_y, badge_size / 2, bg_hex
            ));
        }

        // Texto inferior
        if has_text {
            let text_val = match settings.text_casing {
                TextCasing::TextField => settings.text_content.clone(),
                TextCasing::Small => settings.text_content.to_lowercase(),
                TextCasing::Great => settings.text_content.to_uppercase(),
            };
            let text_y = width + (text_height * 2 / 3);
            let font_weight = if settings.text_bold { "bold" } else { "normal" };
            let font_style = if settings.text_italic { "italic" } else { "normal" };
            let text_decor = if settings.text_underline { "underline" } else { "none" };

            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="system-ui, sans-serif" font-weight="{}" font-style="{}" text-decoration="{}" font-size="{}" fill="{}" text-anchor="middle">{}</text>"#,
                width / 2 + settings.text_offset_x as usize,
                text_y + settings.text_offset_y as usize,
                font_weight,
                font_style,
                text_decor,
                settings.text_font_size,
                txt_hex,
                html_escape(&text_val)
            ));
        }

        svg.push_str("</svg>");
        Ok(svg)
    }

    /// Genera código EPS (Encapsulated PostScript)
    pub fn generate_eps(payload: &str, settings: &QrSettings) -> Result<String, QrError> {
        let code = QrCode::with_error_correction_level(payload.as_bytes(), settings.ec_level.to_ec_level())?;
        let modules = code.to_colors();
        let matrix_size = code.width();
        let total_modules = matrix_size + settings.border * 2;
        let scale = settings.scale_factor;
        let width = total_modules * scale;

        let mut eps = String::new();
        eps.push_str("%!PS-Adobe-3.0 EPSF-3.0\n");
        eps.push_str(&format!("%%BoundingBox: 0 0 {} {}\n", width, width));
        eps.push_str("%%Pages: 1\n");
        eps.push_str("%%EndComments\n");

        // Fondo
        let r = settings.bg_color[0] as f32 / 255.0;
        let g = settings.bg_color[1] as f32 / 255.0;
        let b = settings.bg_color[2] as f32 / 255.0;
        eps.push_str(&format!("{} {} {} setrgbcolor\n", r, g, b));
        eps.push_str(&format!("0 0 {} {} rectfill\n", width, width));

        // Frente
        let fr = settings.fg_color[0] as f32 / 255.0;
        let fg = settings.fg_color[1] as f32 / 255.0;
        let fb = settings.fg_color[2] as f32 / 255.0;
        eps.push_str(&format!("{} {} {} setrgbcolor\n", fr, fg, fb));

        for y in 0..matrix_size {
            for x in 0..matrix_size {
                if modules[y * matrix_size + x] == Color::Dark {
                    let rx = (x + settings.border) * scale;
                    let ry = width - ((y + settings.border + 1) * scale); // Invertir eje Y para PostScript
                    eps.push_str(&format!("{} {} {} {} rectfill\n", rx, ry, scale, scale));
                }
            }
        }

        eps.push_str("%%EOF\n");
        Ok(eps)
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pixel_size() {
        let settings = QrSettings {
            border: 2,
            scale_factor: 7,
            ..Default::default()
        };
        let (w, h) = QrEngine::compute_pixel_size("¡Mi primer código QR!", &settings);
        assert!(w > 0);
        assert_eq!(w, h);
    }

    #[test]
    fn test_generate_image_basic() {
        let settings = QrSettings::default();
        let img = QrEngine::generate_image("¡Mi primer código QR!", &settings, None).unwrap();
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
}

