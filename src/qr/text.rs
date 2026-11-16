use std::sync::OnceLock;
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{ImageBuffer, Rgba};

static FONT_REGULAR: OnceLock<Vec<u8>> = OnceLock::new();
static FONT_BOLD: OnceLock<Vec<u8>> = OnceLock::new();
static FONT_ITALIC: OnceLock<Vec<u8>> = OnceLock::new();
static FONT_BOLD_ITALIC: OnceLock<Vec<u8>> = OnceLock::new();

fn get_font_bytes(bold: bool, italic: bool) -> Option<&'static [u8]> {
    let cell = match (bold, italic) {
        (true, true) => &FONT_BOLD_ITALIC,
        (true, false) => &FONT_BOLD,
        (false, true) => &FONT_ITALIC,
        (false, false) => &FONT_REGULAR,
    };

    let bytes = cell.get_or_init(|| {
        let path = match (bold, italic) {
            (true, true) => "C:\\Windows\\Fonts\\segoeuiz.ttf",
            (true, false) => "C:\\Windows\\Fonts\\segoeuib.ttf",
            (false, true) => "C:\\Windows\\Fonts\\segoeuii.ttf",
            (false, false) => "C:\\Windows\\Fonts\\segoeui.ttf",
        };
        std::fs::read(path)
            .or_else(|_| std::fs::read("C:\\Windows\\Fonts\\arial.ttf"))
            .unwrap_or_default()
    });

    if bytes.is_empty() {
        None
    } else {
        Some(bytes.as_slice())
    }
}

pub fn render_text_onto_image(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    text: &str,
    requested_font_size: f32,
    color: [u8; 4],
    start_y: u32,
    offset_x: i32,
    offset_y: i32,
    bold: bool,
    italic: bool,
    underline: bool,
) {
    if text.trim().is_empty() {
        return;
    }

    let font_bytes = get_font_bytes(bold, italic);

    if let Some(bytes) = font_bytes {
        if let Ok(font) = FontRef::try_from_slice(bytes) {
            let img_w = img.width() as f32;
            let mut font_size = requested_font_size.max(8.0);

            let initial_scale = PxScale::from(font_size);
            let initial_scaled = font.as_scaled(initial_scale);
            let measured_width: f32 = text
                .chars()
                .map(|c| initial_scaled.h_advance(initial_scaled.glyph_id(c)))
                .sum();

            if measured_width > (img_w - 16.0) && measured_width > 0.0 {
                let ratio = (img_w - 16.0) / measured_width;
                font_size = (font_size * ratio).max(8.0);
            }

            let scale = PxScale::from(font_size);
            let scaled_font = font.as_scaled(scale);

            let mut total_width = 0.0_f32;
            let mut glyphs = Vec::new();
            for ch in text.chars() {
                let glyph_id = scaled_font.glyph_id(ch);
                let advance = scaled_font.h_advance(glyph_id);
                glyphs.push((glyph_id, total_width));
                total_width += advance;
            }

            let text_start_x = ((img_w - total_width) / 2.0 + offset_x as f32).max(1.0);
            let available_h = (img.height().saturating_sub(start_y)) as f32;
            let text_baseline_y = start_y as f32 + (available_h * 0.65) + offset_y as f32;

            for (glyph_id, x_off) in glyphs {
                let glyph = glyph_id.with_scale_and_position(
                    scale,
                    ab_glyph::point(text_start_x + x_off, text_baseline_y),
                );

                if let Some(outlined) = font.outline_glyph(glyph) {
                    let bounds = outlined.px_bounds();
                    outlined.draw(|gx, gy, coverage| {
                        let px = bounds.min.x as i32 + gx as i32;
                        let py = bounds.min.y as i32 + gy as i32;

                        if px >= 0 && px < img.width() as i32 && py >= 0 && py < img.height() as i32 && coverage > 0.01 {
                            let upx = px as u32;
                            let upy = py as u32;
                            let alpha = coverage * (color[3] as f32 / 255.0);
                            let bg = img.get_pixel(upx, upy);

                            let r = (color[0] as f32 * alpha + bg[0] as f32 * (1.0 - alpha)) as u8;
                            let g = (color[1] as f32 * alpha + bg[1] as f32 * (1.0 - alpha)) as u8;
                            let b = (color[2] as f32 * alpha + bg[2] as f32 * (1.0 - alpha)) as u8;

                            img.put_pixel(upx, upy, Rgba([r, g, b, 255]));
                        }
                    });
                }
            }

            if underline {
                let ul_y = (text_baseline_y + 3.0) as u32;
                let ul_h = (font_size * 0.08).max(1.0) as u32;
                let ul_start = text_start_x as u32;
                let ul_end = ((text_start_x + total_width) as u32).min(img.width());
                for uy in ul_y..(ul_y + ul_h) {
                    if uy < img.height() {
                        for ux in ul_start..ul_end {
                            img.put_pixel(ux, uy, Rgba(color));
                        }
                    }
                }
            }
        }
    }
}
