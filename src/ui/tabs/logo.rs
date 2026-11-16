use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::ui::ids::*;
use crate::ui::state::AppState;
use crate::ui::widgets::*;

pub unsafe fn build_tab_logo(hwnd: HWND, instance: HINSTANCE, state: *mut AppState) {
    let regular = (*state).hfont_regular;
    let bold = (*state).hfont_bold;

    let mut tab2 = Vec::new();

    // Panel Izquierdo: Configuracion de logotipo
    let chk_logo = create_native_checkbox(hwnd, "Habilitar logo central", 48, 448, 200, 22, ID_CHK_LOGO_ENABLED, regular, (*state).settings.logo_enabled);
    (*state).hwnd_chk_logo_enabled = chk_logo;
    tab2.push(chk_logo);

    let btn_pick_logo = create_native_btn(hwnd, "Seleccionar imagen...", 48, 478, 175, 28, ID_BTN_SELECT_LOGO, regular);
    (*state).hwnd_btn_select_logo = btn_pick_logo;
    tab2.push(btn_pick_logo);

    let btn_rem_logo = create_native_btn(hwnd, "Quitar imagen", 230, 478, 130, 28, ID_BTN_REMOVE_LOGO, regular);
    (*state).hwnd_btn_remove_logo = btn_rem_logo;
    tab2.push(btn_rem_logo);

    let lbl_l_path = create_native_label(hwnd, "Logo: (Plantilla predefinida)", 48, 514, 420, 18, regular);
    (*state).hwnd_lbl_logo_path = lbl_l_path;
    tab2.push(lbl_l_path);

    let lbl_tpl = create_native_label(hwnd, "Plantilla:", 48, 542, 80, 20, regular);
    tab2.push(lbl_tpl);

    let logo_tpl_items = [
        "---",
        "Icono de la aplicacion (icon.ico)",
        "Corazon / Pulso",
        "Estrella",
        "Escudo",
        "Simbolo Wi-Fi",
        "X (Twitter)",
        "Avatar de contacto",
    ];
    let combo_tpl = create_native_combobox(hwnd, 135, 540, 310, 180, ID_COMBO_LOGO_TEMPLATE, regular, &logo_tpl_items, 0);
    (*state).hwnd_combo_logo_template = combo_tpl;
    tab2.push(combo_tpl);

    let size_txt = format!("Tamano: {} px", (*state).settings.logo_size_px);
    let lbl_l_size = create_native_label(hwnd, &size_txt, 48, 582, 120, 20, regular);
    (*state).hwnd_lbl_logo_size = lbl_l_size;
    tab2.push(lbl_l_size);

    let track_l_size = {
        let hwnd_track = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("msctls_trackbar32"),
            PCWSTR::null(),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            170, 580, 275, 26,
            hwnd,
            HMENU(ID_TRACK_LOGO_SIZE as usize as isize),
            instance,
            None,
        );
        SendMessageW(hwnd_track, TBM_SETRANGE, WPARAM(1), LPARAM(((30 & 0xFFFF) | ((180 & 0xFFFF) << 16)) as isize));
        SendMessageW(hwnd_track, TBM_SETPOS, WPARAM(1), LPARAM((*state).settings.logo_size_px as isize));
        hwnd_track
    };
    (*state).hwnd_track_logo_size = track_l_size;
    tab2.push(track_l_size);

    let chk_center = create_native_checkbox(hwnd, "Mantener centrado al redimensionar", 48, 620, 270, 20, ID_CHK_KEEP_CENTERED, regular, (*state).settings.keep_centered_enlarge);
    (*state).hwnd_chk_keep_centered = chk_center;
    tab2.push(chk_center);

    let chk_valid = create_native_checkbox(hwnd, "Comprobar validez de lectura", 48, 646, 270, 20, ID_CHK_CHECK_VALIDITY, regular, (*state).settings.check_validity);
    (*state).hwnd_chk_check_validity = chk_valid;
    tab2.push(chk_valid);

    let btn_auto = create_native_btn(hwnd, "Ajuste automatico", 325, 632, 135, 28, ID_BTN_AUTO_ADJUST, regular);
    (*state).hwnd_btn_auto_adjust = btn_auto;
    tab2.push(btn_auto);

    // Panel Derecho: Posicion del logo
    let btn_up = create_native_btn(hwnd, " ^ ", 645, 452, 42, 30, ID_BTN_LOGO_UP, bold);
    tab2.push(btn_up);

    let btn_left = create_native_btn(hwnd, " < ", 595, 488, 42, 30, ID_BTN_LOGO_LEFT, bold);
    tab2.push(btn_left);

    let btn_down = create_native_btn(hwnd, " v ", 645, 488, 42, 30, ID_BTN_LOGO_DOWN, bold);
    tab2.push(btn_down);

    let btn_right = create_native_btn(hwnd, " > ", 695, 488, 42, 30, ID_BTN_LOGO_RIGHT, bold);
    tab2.push(btn_right);

    let btn_c = create_native_btn(hwnd, "Centrar", 610, 528, 112, 28, ID_BTN_LOGO_CENTER, regular);
    tab2.push(btn_c);

    let lbl_step = create_native_label(hwnd, "Pixeles por clic:", 540, 580, 120, 20, regular);
    tab2.push(lbl_step);

    let combo_l_step = create_native_combobox(hwnd, 670, 578, 90, 100, ID_COMBO_LOGO_STEP, regular, &["1", "2", "5", "10", "20"], 2);
    (*state).hwnd_combo_logo_step = combo_l_step;
    tab2.push(combo_l_step);

    let lbl_tip = create_native_label(hwnd, "Sugerencia: Nivel Alto (H) y tamano entre 60 y 90 px garantizan un escaneo optimo.", 530, 625, 260, 45, regular);
    tab2.push(lbl_tip);

    (*state).tab_controls[2] = tab2;
}
