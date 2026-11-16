use std::path::PathBuf;

use image::{ImageBuffer, Rgba};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{HBRUSH, HFONT};

use crate::qr::QrSettings;

pub fn to_pcwstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct AppState {
    pub input_text: String,
    pub use_text_in_qr_as_input: bool,
    pub settings: QrSettings,
    pub active_tab: usize,

    pub custom_logo_img: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    pub custom_logo_path: Option<PathBuf>,
    pub current_qr_img: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    pub current_qr_bgra: Vec<u8>,
    pub current_qr_dims: (u32, u32),
    pub status_message: String,
    pub last_computed_size: (u32, u32),

    pub hwnd_main: HWND,
    pub hfont_regular: HFONT,
    pub hfont_bold: HFONT,
    pub hfont_title: HFONT,
    pub hbrush_white: HBRUSH,
    pub hbrush_card: HBRUSH,
    pub hbrush_border: HBRUSH,

    // Controls
    pub hwnd_toolbar_btns: [HWND; 7],
    pub current_start_x: i32,
    pub hwnd_tab: HWND,
    pub hwnd_tab_btns: [HWND; 5],
    pub hwnd_status: HWND,

    // Tab 0
    pub hwnd_edit_input: HWND,
    pub hwnd_lbl_char_count: HWND,
    pub hwnd_chk_use_text_in_qr: HWND,
    pub hwnd_btn_import: HWND,
    pub hwnd_btn_clear: HWND,

    // Tab 1
    pub hwnd_combo_ec: HWND,
    pub hwnd_combo_border: HWND,
    pub hwnd_combo_scale: HWND,
    pub hwnd_lbl_final_size: HWND,
    pub hwnd_chk_save_settings: HWND,
    pub hwnd_btn_color_bg: HWND,
    pub hwnd_lbl_hex_bg: HWND,
    pub hwnd_btn_color_fg: HWND,
    pub hwnd_lbl_hex_fg: HWND,
    pub hwnd_btn_random_colors: HWND,
    pub hwnd_btn_swap_colors: HWND,

    // Tab 2
    pub hwnd_chk_logo_enabled: HWND,
    pub hwnd_btn_select_logo: HWND,
    pub hwnd_btn_remove_logo: HWND,
    pub hwnd_lbl_logo_path: HWND,
    pub hwnd_combo_logo_template: HWND,
    pub hwnd_lbl_logo_size: HWND,
    pub hwnd_track_logo_size: HWND,
    pub hwnd_chk_keep_centered: HWND,
    pub hwnd_chk_check_validity: HWND,
    pub hwnd_btn_auto_adjust: HWND,
    pub hwnd_combo_logo_step: HWND,

    // Tab 3
    pub hwnd_chk_text_enabled: HWND,
    pub hwnd_edit_text_content: HWND,
    pub hwnd_combo_text_font: HWND,
    pub hwnd_radio_casing_orig: HWND,
    pub hwnd_radio_casing_lower: HWND,
    pub hwnd_radio_casing_upper: HWND,
    pub hwnd_chk_text_bold: HWND,
    pub hwnd_chk_text_italic: HWND,
    pub hwnd_chk_text_underline: HWND,
    pub hwnd_btn_text_color: HWND,
    pub hwnd_lbl_text_color_hex: HWND,
    pub hwnd_lbl_text_font_size: HWND,
    pub hwnd_track_text_font_size: HWND,
    pub hwnd_combo_text_step: HWND,

    // Tab 4
    pub hwnd_edit_verify_read: HWND,
    pub hwnd_btn_verify_manual: HWND,
    pub hwnd_btn_verify_extended: HWND,

    // Groups & controls per tab
    pub tab_controls: [Vec<HWND>; 5],
}

pub static mut GLOBAL_STATE: *mut AppState = std::ptr::null_mut();

#[inline]
pub unsafe fn get_state() -> Option<&'static mut AppState> {
    if GLOBAL_STATE.is_null() {
        None
    } else {
        Some(&mut *GLOBAL_STATE)
    }
}
