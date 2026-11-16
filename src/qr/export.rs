use qrcode::{types::QrError, Color, QrCode};

use super::types::{LogoTemplate, QrSettings, TextCasing};

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

    if settings.logo_enabled {
        let logo_size = settings.logo_size_px as usize;
        let center_x = ((width / 2) as i32 + settings.logo_offset_x).max(logo_size as i32 / 2) as usize;
        let center_y = ((width / 2) as i32 + settings.logo_offset_y).max(logo_size as i32 / 2) as usize;
        let badge_size = (logo_size as f32 * 1.25) as usize;

        svg.push_str(&format!(
            r#"<circle cx="{}" cy="{}" r="{}" fill="{}" />"#,
            center_x, center_y, badge_size / 2, bg_hex
        ));

        if settings.logo_template == LogoTemplate::AppIcon {
            let ico_bytes = include_bytes!("../../img/icon.ico");
            if let Ok(dyn_img) = image::load_from_memory_with_format(ico_bytes, image::ImageFormat::Ico) {
                let mut png_bytes = Vec::new();
                let mut cursor = std::io::Cursor::new(&mut png_bytes);
                if dyn_img.write_to(&mut cursor, image::ImageFormat::Png).is_ok() {
                    let b64 = base64_encode(&png_bytes);
                    let top_left_x = center_x.saturating_sub(logo_size / 2);
                    let top_left_y = center_y.saturating_sub(logo_size / 2);
                    svg.push_str(&format!(
                        r#"<image href="data:image/png;base64,{}" x="{}" y="{}" width="{}" height="{}" />"#,
                        b64, top_left_x, top_left_y, logo_size, logo_size
                    ));
                }
            }
        }
    }

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

    let r = settings.bg_color[0] as f32 / 255.0;
    let g = settings.bg_color[1] as f32 / 255.0;
    let b = settings.bg_color[2] as f32 / 255.0;
    eps.push_str(&format!("{} {} {} setrgbcolor\n", r, g, b));
    eps.push_str(&format!("0 0 {} {} rectfill\n", width, width));

    let fr = settings.fg_color[0] as f32 / 255.0;
    let fg = settings.fg_color[1] as f32 / 255.0;
    let fb = settings.fg_color[2] as f32 / 255.0;
    eps.push_str(&format!("{} {} {} setrgbcolor\n", fr, fg, fb));

    for y in 0..matrix_size {
        for x in 0..matrix_size {
            if modules[y * matrix_size + x] == Color::Dark {
                let rx = (x + settings.border) * scale;
                let ry = width - ((y + settings.border + 1) * scale);
                eps.push_str(&format!("{} {} {} {} rectfill\n", rx, ry, scale, scale));
            }
        }
    }

    eps.push_str("%%EOF\n");
    Ok(eps)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if i + 1 < data.len() {
            out.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}
