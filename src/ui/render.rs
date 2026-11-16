use windows::core::PCWSTR;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::InvalidateRect;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, SetWindowTextW};

use super::state::{to_pcwstr, AppState};
use crate::qr::QrEngine;

pub fn regenerate_qr(state: &mut AppState) {
    let payload = if state.use_text_in_qr_as_input && state.settings.text_enabled {
        state.settings.text_content.clone()
    } else {
        state.input_text.clone()
    };

    let effective_payload = if payload.trim().is_empty() {
        " ".to_string()
    } else {
        payload
    };

    let custom_logo = if state.settings.logo_enabled {
        state.custom_logo_img.as_ref()
    } else {
        None
    };

    state.last_computed_size = QrEngine::compute_pixel_size(&effective_payload, &state.settings);

    match QrEngine::generate_image(&effective_payload, &state.settings, custom_logo) {
        Ok(img) => {
            let img_w = img.width();
            let img_h = img.height();

            let raw = img.as_raw();
            let mut bgra = Vec::with_capacity((img_w * img_h * 4) as usize);
            for chunk in raw.chunks_exact(4) {
                bgra.push(chunk[2]); // B
                bgra.push(chunk[1]); // G
                bgra.push(chunk[0]); // R
                bgra.push(chunk[3]); // A
            }
            state.current_qr_bgra = bgra;
            state.current_qr_dims = (img_w, img_h);
            state.current_qr_img = Some(img);
            state.status_message = "Listo. Codigo QR generado correctamente.".to_string();
        }
        Err(e) => {
            state.status_message = format!("Error al generar QR: {}", e);
        }
    }

    unsafe {
        if state.hwnd_status.0 != 0 {
            let msg = to_pcwstr(&state.status_message);
            SetWindowTextW(state.hwnd_status, PCWSTR(msg.as_ptr()));
        }

        if state.hwnd_lbl_final_size.0 != 0 {
            let txt = format!("{} x {} px", state.last_computed_size.0, state.last_computed_size.1);
            let wtxt = to_pcwstr(&txt);
            SetWindowTextW(state.hwnd_lbl_final_size, PCWSTR(wtxt.as_ptr()));
        }

        if state.hwnd_edit_verify_read.0 != 0 {
            let wtxt = to_pcwstr(&effective_payload);
            SetWindowTextW(state.hwnd_edit_verify_read, PCWSTR(wtxt.as_ptr()));
        }

        if state.hwnd_main.0 != 0 {
            let mut rect = RECT::default();
            GetClientRect(state.hwnd_main, &mut rect);
            rect.bottom = 380;
            let _ = InvalidateRect(state.hwnd_main, Some(&rect), false);
        }
    }
}
