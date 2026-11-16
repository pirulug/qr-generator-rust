use windows::Win32::Foundation::{HINSTANCE, HWND};

use crate::ui::ids::*;
use crate::ui::state::AppState;
use crate::ui::widgets::*;

pub unsafe fn build_tab_colors(hwnd: HWND, _instance: HINSTANCE, state: *mut AppState) {
    let regular = (*state).hfont_regular;
    let bold = (*state).hfont_bold;

    let mut tab1 = Vec::new();

    // Panel Izquierdo: Ajustes de la matriz
    let lbl_ec = create_native_label(hwnd, "Correccion de error:", 48, 452, 150, 20, regular);
    tab1.push(lbl_ec);
    let combo_ec = create_native_combobox(hwnd, 205, 448, 240, 150, ID_COMBO_EC, regular, &["Bajo (L)", "Medio (M)", "Cuartil (Q)", "Alto (H)"], 0);
    (*state).hwnd_combo_ec = combo_ec;
    tab1.push(combo_ec);

    let lbl_border = create_native_label(hwnd, "Borde (modulos):", 48, 496, 150, 20, regular);
    tab1.push(lbl_border);
    let border_items = ["0", "1", "2", "3", "4", "5", "6", "7", "8"];
    let combo_border = create_native_combobox(hwnd, 205, 492, 240, 150, ID_COMBO_BORDER, regular, &border_items, 2);
    (*state).hwnd_combo_border = combo_border;
    tab1.push(combo_border);

    let lbl_scale = create_native_label(hwnd, "Factor de escala:", 48, 540, 150, 20, regular);
    tab1.push(lbl_scale);
    let scale_items = ["2 px", "3 px", "4 px", "5 px", "6 px", "7 px", "8 px", "9 px", "10 px", "12 px"];
    let combo_scale = create_native_combobox(hwnd, 205, 536, 240, 150, ID_COMBO_SCALE, regular, &scale_items, 5);
    (*state).hwnd_combo_scale = combo_scale;
    tab1.push(combo_scale);

    let lbl_size_title = create_native_label(hwnd, "Tamano final:", 48, 584, 150, 20, regular);
    tab1.push(lbl_size_title);
    let final_size_txt = format!("{} x {} px", (*state).last_computed_size.0, (*state).last_computed_size.1);
    let lbl_size_val = create_native_label(hwnd, &final_size_txt, 205, 584, 240, 20, bold);
    (*state).hwnd_lbl_final_size = lbl_size_val;
    tab1.push(lbl_size_val);

    let chk_save = create_native_checkbox(hwnd, "Guardar configuracion predeterminada", 48, 645, 380, 22, ID_CHK_SAVE_SETTINGS, regular, false);
    (*state).hwnd_chk_save_settings = chk_save;
    tab1.push(chk_save);

    // Panel Derecho: Color del codigo QR
    let btn_c_bg = create_native_btn(hwnd, "Color Fondo", 530, 448, 115, 28, ID_BTN_COLOR_BG, regular);
    (*state).hwnd_btn_color_bg = btn_c_bg;
    tab1.push(btn_c_bg);

    let hex_bg = format!("h00{:02X}{:02X}{:02X}", (*state).settings.bg_color[0], (*state).settings.bg_color[1], (*state).settings.bg_color[2]);
    let lbl_h_bg = create_native_label(hwnd, &hex_bg, 660, 452, 130, 20, bold);
    (*state).hwnd_lbl_hex_bg = lbl_h_bg;
    tab1.push(lbl_h_bg);

    let btn_c_fg = create_native_btn(hwnd, "Color Frente", 530, 490, 115, 28, ID_BTN_COLOR_FG, regular);
    (*state).hwnd_btn_color_fg = btn_c_fg;
    tab1.push(btn_c_fg);

    let hex_fg = format!("h00{:02X}{:02X}{:02X}", (*state).settings.fg_color[0], (*state).settings.fg_color[1], (*state).settings.fg_color[2]);
    let lbl_h_fg = create_native_label(hwnd, &hex_fg, 660, 494, 130, 20, bold);
    (*state).hwnd_lbl_hex_fg = lbl_h_fg;
    tab1.push(lbl_h_fg);

    let btn_rand = create_native_btn(hwnd, "Paleta aleatoria", 530, 532, 125, 28, ID_BTN_RANDOM_COLORS, regular);
    (*state).hwnd_btn_random_colors = btn_rand;
    tab1.push(btn_rand);

    let btn_swap = create_native_btn(hwnd, "Invertir colores", 665, 532, 125, 28, ID_BTN_SWAP_COLORS, regular);
    (*state).hwnd_btn_swap_colors = btn_swap;
    tab1.push(btn_swap);

    let lbl_p_title = create_native_label(hwnd, "Estilos rapidos de 1 clic:", 530, 574, 260, 18, regular);
    tab1.push(lbl_p_title);

    let btn_p1 = create_native_btn(hwnd, "Clasico", 530, 598, 60, 26, ID_BTN_PALETTE_CLASSIC, regular);
    tab1.push(btn_p1);

    let btn_p2 = create_native_btn(hwnd, "Azul", 596, 598, 60, 26, ID_BTN_PALETTE_TECH_BLUE, regular);
    tab1.push(btn_p2);

    let btn_p3 = create_native_btn(hwnd, "Verde", 662, 598, 60, 26, ID_BTN_PALETTE_BIO_GREEN, regular);
    tab1.push(btn_p3);

    let btn_p4 = create_native_btn(hwnd, "Oscuro", 728, 598, 62, 26, ID_BTN_PALETTE_DARK_MODE, regular);
    tab1.push(btn_p4);

    let lbl_color_hint = create_native_label(hwnd, "Sugerencia: Alto contraste garantiza un escaneo rapido.", 530, 640, 260, 36, regular);
    tab1.push(lbl_color_hint);

    (*state).tab_controls[1] = tab1;
}
