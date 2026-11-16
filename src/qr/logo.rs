use image::{ImageBuffer, Rgba};

pub fn draw_app_icon_logo(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cx: u32, cy: u32, size: u32) {
    let ico_bytes = include_bytes!("../../img/icon.ico");
    if let Ok(dyn_img) = image::load_from_memory_with_format(ico_bytes, image::ImageFormat::Ico) {
        let rgba = dyn_img.to_rgba8();
        let half = size / 2;
        let start_x = cx.saturating_sub(half);
        let start_y = cy.saturating_sub(half);

        let pad = (size as f32 * 0.08).max(3.0) as i32;
        let badge_half = half as i32 + pad;
        let badge_corner_r = (badge_half as f32 * 0.35) as i32;
        let white = Rgba([255, 255, 255, 255]);

        for dy in -badge_half..=badge_half {
            for dx in -badge_half..=badge_half {
                let px = (cx as i32 + dx) as u32;
                let py = (cy as i32 + dy) as u32;
                if px >= img.width() || py >= img.height() {
                    continue;
                }

                let inner_x = dx.abs() - (badge_half - badge_corner_r);
                let inner_y = dy.abs() - (badge_half - badge_corner_r);
                if inner_x > 0 && inner_y > 0 && inner_x * inner_x + inner_y * inner_y > badge_corner_r * badge_corner_r {
                    continue;
                }
                img.put_pixel(px, py, white);
            }
        }

        let resized = image::imageops::resize(
            &rgba,
            size,
            size,
            image::imageops::FilterType::Lanczos3,
        );

        for (lx, ly, pixel) in resized.enumerate_pixels() {
            let tx = start_x + lx;
            let ty = start_y + ly;
            if tx < img.width() && ty < img.height() && pixel[3] > 0 {
                let alpha = pixel[3] as f32 / 255.0;
                let bg = img.get_pixel(tx, ty);
                let r = (pixel[0] as f32 * alpha + bg[0] as f32 * (1.0 - alpha)) as u8;
                let g = (pixel[1] as f32 * alpha + bg[1] as f32 * (1.0 - alpha)) as u8;
                let b = (pixel[2] as f32 * alpha + bg[2] as f32 * (1.0 - alpha)) as u8;
                img.put_pixel(tx, ty, Rgba([r, g, b, 255]));
            }
        }
    }
}

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

    let check_size = (size as f32 * 0.15) as i32;
    for t in -check_size..=check_size {
        let px = (cx as i32 + t) as u32;
        let py = (cy as i32 + t.abs() / 2) as u32;
        if px < img.width() && py < img.height() {
            img.put_pixel(px, py, white);
        }
    }
}

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
