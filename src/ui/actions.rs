use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HANDLE, HGLOBAL, HWND};
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use windows::Win32::UI::Controls::Dialogs::{ChooseColorW, CC_FULLOPEN, CC_RGBINIT, CHOOSECOLORW};
use windows::Win32::UI::WindowsAndMessaging::SetWindowTextW;

use super::ids::*;
use super::render::regenerate_qr;
use super::state::{get_state, to_pcwstr};
use crate::qr::QrEngine;

pub fn offset_logo(dx_mult: i32, dy_mult: i32) {
    if let Some(state) = unsafe { get_state() } {
        let step = state.settings.logo_pixel_per_click;
        state.settings.logo_offset_x += dx_mult * step;
        state.settings.logo_offset_y += dy_mult * step;
        regenerate_qr(state);
    }
}

pub fn offset_text(dx_mult: i32, dy_mult: i32) {
    if let Some(state) = unsafe { get_state() } {
        let step = state.settings.text_pixel_per_click;
        state.settings.text_offset_x += dx_mult * step;
        state.settings.text_offset_y += dy_mult * step;
        regenerate_qr(state);
    }
}

pub fn pick_color_dialog(hwnd: HWND, is_bg: bool) {
    unsafe {
        let mut custom_colors = [COLORREF(0); 16];
        let current_color = if let Some(state) = get_state() {
            let c = if is_bg { state.settings.bg_color } else { state.settings.fg_color };
            COLORREF((c[0] as u32) | ((c[1] as u32) << 8) | ((c[2] as u32) << 16))
        } else {
            COLORREF(0)
        };

        let mut cc = CHOOSECOLORW {
            lStructSize: std::mem::size_of::<CHOOSECOLORW>() as u32,
            hwndOwner: hwnd,
            rgbResult: current_color,
            lpCustColors: custom_colors.as_mut_ptr(),
            Flags: CC_FULLOPEN | CC_RGBINIT,
            ..Default::default()
        };

        if ChooseColorW(&mut cc).as_bool() {
            let rgb = cc.rgbResult.0;
            let r = (rgb & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = ((rgb >> 16) & 0xFF) as u8;

            if let Some(state) = get_state() {
                if is_bg {
                    state.settings.bg_color = [r, g, b, 255];
                    let hex = format!("h00{:02X}{:02X}{:02X}", r, g, b);
                    let whex = to_pcwstr(&hex);
                    SetWindowTextW(state.hwnd_lbl_hex_bg, PCWSTR(whex.as_ptr()));
                } else {
                    state.settings.fg_color = [r, g, b, 255];
                    let hex = format!("h00{:02X}{:02X}{:02X}", r, g, b);
                    let whex = to_pcwstr(&hex);
                    SetWindowTextW(state.hwnd_lbl_hex_fg, PCWSTR(whex.as_ptr()));
                }
                regenerate_qr(state);
            }
        }
    }
}

pub fn pick_text_color_dialog(hwnd: HWND) {
    unsafe {
        let mut custom_colors = [COLORREF(0); 16];
        let current_color = if let Some(state) = get_state() {
            let c = state.settings.text_color;
            COLORREF((c[0] as u32) | ((c[1] as u32) << 8) | ((c[2] as u32) << 16))
        } else {
            COLORREF(0)
        };

        let mut cc = CHOOSECOLORW {
            lStructSize: std::mem::size_of::<CHOOSECOLORW>() as u32,
            hwndOwner: hwnd,
            rgbResult: current_color,
            lpCustColors: custom_colors.as_mut_ptr(),
            Flags: CC_FULLOPEN | CC_RGBINIT,
            ..Default::default()
        };

        if ChooseColorW(&mut cc).as_bool() {
            let rgb = cc.rgbResult.0;
            let r = (rgb & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = ((rgb >> 16) & 0xFF) as u8;

            if let Some(state) = get_state() {
                state.settings.text_color = [r, g, b, 255];
                let hex = format!("h00{:02X}{:02X}{:02X}", r, g, b);
                let whex = to_pcwstr(&hex);
                SetWindowTextW(state.hwnd_lbl_text_color_hex, PCWSTR(whex.as_ptr()));
                regenerate_qr(state);
            }
        }
    }
}

pub fn randomize_colors() {
    if let Some(state) = unsafe { get_state() } {
        let palettes: [([u8; 4], [u8; 4]); 6] = [
            ([255, 255, 255, 255], [0, 0, 0, 255]),
            ([255, 255, 255, 255], [37, 99, 235, 255]),
            ([255, 255, 255, 255], [5, 150, 105, 255]),
            ([255, 255, 255, 255], [220, 38, 38, 255]),
            ([255, 255, 255, 255], [124, 58, 237, 255]),
            ([255, 255, 255, 255], [217, 119, 6, 255]),
        ];
        let idx = (state.settings.scale_factor as usize) % palettes.len();
        state.settings.bg_color = palettes[idx].0;
        state.settings.fg_color = palettes[idx].1;

        unsafe {
            let hex_bg = format!("h00{:02X}{:02X}{:02X}", state.settings.bg_color[0], state.settings.bg_color[1], state.settings.bg_color[2]);
            let whex_bg = to_pcwstr(&hex_bg);
            SetWindowTextW(state.hwnd_lbl_hex_bg, PCWSTR(whex_bg.as_ptr()));

            let hex_fg = format!("h00{:02X}{:02X}{:02X}", state.settings.fg_color[0], state.settings.fg_color[1], state.settings.fg_color[2]);
            let whex_fg = to_pcwstr(&hex_fg);
            SetWindowTextW(state.hwnd_lbl_hex_fg, PCWSTR(whex_fg.as_ptr()));
        }

        regenerate_qr(state);
    }
}

pub fn swap_colors() {
    if let Some(state) = unsafe { get_state() } {
        let tmp = state.settings.bg_color;
        state.settings.bg_color = state.settings.fg_color;
        state.settings.fg_color = tmp;

        unsafe {
            let hex_bg = format!("h00{:02X}{:02X}{:02X}", state.settings.bg_color[0], state.settings.bg_color[1], state.settings.bg_color[2]);
            let whex_bg = to_pcwstr(&hex_bg);
            SetWindowTextW(state.hwnd_lbl_hex_bg, PCWSTR(whex_bg.as_ptr()));

            let hex_fg = format!("h00{:02X}{:02X}{:02X}", state.settings.fg_color[0], state.settings.fg_color[1], state.settings.fg_color[2]);
            let whex_fg = to_pcwstr(&hex_fg);
            SetWindowTextW(state.hwnd_lbl_hex_fg, PCWSTR(whex_fg.as_ptr()));
        }

        regenerate_qr(state);
    }
}

pub fn export_qr_file(fmt_name: &str, filter_desc: &str, extensions: &[&str]) {
    if let Some(state) = unsafe { get_state() } {
        if let Some(img) = &state.current_qr_img {
            let default_name = format!("codigo_qr.{}", extensions[0]);
            if let Some(path) = rfd::FileDialog::new()
                .add_filter(filter_desc, extensions)
                .set_file_name(&default_name)
                .save_file()
            {
                if let Err(e) = img.save(&path) {
                    update_status(&format!("Error al guardar {}: {}", fmt_name, e));
                } else {
                    update_status(&format!("Archivo guardado en {:?}", path.file_name().unwrap_or_default()));
                }
            }
        }
    }
}

pub fn export_svg_file() {
    if let Some(state) = unsafe { get_state() } {
        let payload = if state.input_text.trim().is_empty() { " " } else { &state.input_text };
        if let Ok(svg_str) = QrEngine::generate_svg(payload, &state.settings) {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Graficos Vectoriales Escalables SVG (*.svg)", &["svg"])
                .set_file_name("codigo_qr.svg")
                .save_file()
            {
                if let Err(e) = std::fs::write(&path, svg_str) {
                    update_status(&format!("Error al guardar SVG: {}", e));
                } else {
                    update_status(&format!("SVG guardado en {:?}", path.file_name().unwrap_or_default()));
                }
            }
        }
    }
}

pub fn export_eps_file() {
    if let Some(state) = unsafe { get_state() } {
        let payload = if state.input_text.trim().is_empty() { " " } else { &state.input_text };
        if let Ok(eps_str) = QrEngine::generate_eps(payload, &state.settings) {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("PostScript Encapsulado EPS (*.eps)", &["eps"])
                .set_file_name("codigo_qr.eps")
                .save_file()
            {
                if let Err(e) = std::fs::write(&path, eps_str) {
                    update_status(&format!("Error al guardar EPS: {}", e));
                } else {
                    update_status(&format!("EPS guardado en {:?}", path.file_name().unwrap_or_default()));
                }
            }
        }
    }
}

pub fn export_base64_clipboard(hwnd: HWND) {
    if let Some(state) = unsafe { get_state() } {
        if let Some(img) = &state.current_qr_img {
            let mut bytes = Vec::new();
            let mut cursor = std::io::Cursor::new(&mut bytes);
            if img.write_to(&mut cursor, image::ImageFormat::Png).is_ok() {
                let b64 = crate::qr::base64_encode(&bytes);
                let uri = format!("data:image/png;base64,{}", b64);
                copy_text_to_clipboard(hwnd, &uri);
                update_status("URI de datos Base64 copiada al portapapeles.");
            }
        }
    }
}

pub fn export_markdown_clipboard(hwnd: HWND) {
    if let Some(state) = unsafe { get_state() } {
        let payload = state.input_text.clone();
        let md = format!("![Codigo QR para {}](codigo_qr.png)", payload);
        copy_text_to_clipboard(hwnd, &md);
        update_status("Fragmento Markdown copiado al portapapeles.");
    }
}

pub fn copy_text_to_clipboard(hwnd: HWND, text: &str) {
    unsafe {
        let utf16: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes = utf16.len() * 2;
        if OpenClipboard(hwnd).is_ok() {
            let _ = EmptyClipboard();
            if let Ok(hmem) = GlobalAlloc(GMEM_MOVEABLE, bytes) {
                let ptr = GlobalLock(hmem) as *mut u16;
                if !ptr.is_null() {
                    std::ptr::copy_nonoverlapping(utf16.as_ptr(), ptr, utf16.len());
                    let _ = GlobalUnlock(hmem);
                    let _ = SetClipboardData(CF_UNICODETEXT, HANDLE(hmem.0 as isize));
                }
            }
            let _ = CloseClipboard();
        }
    }
}

pub fn paste_text_from_clipboard(hwnd: HWND) -> Option<String> {
    unsafe {
        if OpenClipboard(hwnd).is_ok() {
            let res = if let Ok(handle) = GetClipboardData(CF_UNICODETEXT) {
                let ptr = GlobalLock(HGLOBAL(handle.0 as *mut _)) as *const u16;
                if !ptr.is_null() {
                    let mut len = 0;
                    while *ptr.add(len) != 0 {
                        len += 1;
                    }
                    let slice = std::slice::from_raw_parts(ptr, len);
                    let s = String::from_utf16_lossy(slice);
                    let _ = GlobalUnlock(HGLOBAL(handle.0 as *mut _));
                    Some(s)
                } else {
                    None
                }
            } else {
                None
            };
            let _ = CloseClipboard();
            res
        } else {
            None
        }
    }
}

pub fn update_status(msg: &str) {
    if let Some(state) = unsafe { get_state() } {
        state.status_message = msg.to_string();
        unsafe {
            if state.hwnd_status.0 != 0 {
                let wmsg = to_pcwstr(msg);
                SetWindowTextW(state.hwnd_status, PCWSTR(wmsg.as_ptr()));
            }
        }
    }
}
