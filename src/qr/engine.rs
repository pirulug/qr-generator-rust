use image::{ImageBuffer, Rgba};
use qrcode::{types::QrError, Color, QrCode};

use super::export;
use super::logo;
use super::text;
use super::types::{LogoTemplate, QrSettings, TextCasing};

pub struct QrEngine;

impl QrEngine {
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

        if settings.logo_enabled {
            let logo_size = settings.logo_size_px.clamp(20, qr_px_size.saturating_sub(40));
            let center_x = ((qr_px_size / 2) as i32 + settings.logo_offset_x).max(logo_size as i32 / 2) as u32;
            let center_y = ((qr_px_size / 2) as i32 + settings.logo_offset_y).max(logo_size as i32 / 2) as u32;
            let half_logo = logo_size / 2;

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
                let resized = image::imageops::resize(
                    logo,
                    logo_size,
                    logo_size,
                    image::imageops::FilterType::Triangle,
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
                match settings.logo_template {
                    LogoTemplate::AppIcon => {
                        logo::draw_app_icon_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::HeartPulse => {
                        logo::draw_heart_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::Star => {
                        logo::draw_star_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::Shield => {
                        logo::draw_shield_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::Wifi => {
                        logo::draw_wifi_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::TwitterX => {
                        logo::draw_twitter_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::UserContact => {
                        logo::draw_user_logo(&mut img, center_x, center_y, logo_size);
                    }
                    LogoTemplate::None => {}
                }
            }
        }

        if settings.text_enabled && !settings.text_content.trim().is_empty() {
            let processed_text = match settings.text_casing {
                TextCasing::TextField => settings.text_content.clone(),
                TextCasing::Small => settings.text_content.to_lowercase(),
                TextCasing::Great => settings.text_content.to_uppercase(),
            };

            text::render_text_onto_image(
                &mut img,
                &processed_text,
                settings.text_font_size,
                settings.text_color,
                qr_px_size,
                settings.text_offset_x,
                settings.text_offset_y,
                settings.text_bold,
                settings.text_italic,
                settings.text_underline,
            );
        }

        Ok(img)
    }

    pub fn generate_svg(payload: &str, settings: &QrSettings) -> Result<String, QrError> {
        export::generate_svg(payload, settings)
    }

    pub fn generate_eps(payload: &str, settings: &QrSettings) -> Result<String, QrError> {
        export::generate_eps(payload, settings)
    }
}
