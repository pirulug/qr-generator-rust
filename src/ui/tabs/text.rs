use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::ui::ids::*;
use crate::ui::state::{to_pcwstr, AppState};
use crate::ui::widgets::*;

pub unsafe fn build_tab_text(hwnd: HWND, instance: HINSTANCE, state: *mut AppState) {
    let regular = (*state).hfont_regular;
    let bold = (*state).hfont_bold;

    let mut tab3 = Vec::new();

    // Panel Izquierdo: Configuracion de texto inferior
    let chk_t_en = create_native_checkbox(hwnd, "Habilitar texto inferior", 48, 448, 175, 22, ID_CHK_TEXT_ENABLED, regular, (*state).settings.text_enabled);
    (*state).hwnd_chk_text_enabled = chk_t_en;
    tab3.push(chk_t_en);

    let hwnd_t_content = {
        let wtext = to_pcwstr(&(*state).settings.text_content);
        CreateWindowExW(
            WINDOW_EX_STYLE(WS_EX_CLIENTEDGE.0),
            w!("EDIT"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
            230, 446, 220, 26,
            hwnd,
            HMENU(ID_EDIT_TEXT_CONTENT as usize as isize),
            instance,
            None,
        )
    };
    SendMessageW(hwnd_t_content, WM_SETFONT, WPARAM(regular.0 as usize), LPARAM(1));
    (*state).hwnd_edit_text_content = hwnd_t_content;
    tab3.push(hwnd_t_content);

    let lbl_f_name = create_native_label(hwnd, "Fuente:", 48, 486, 60, 20, regular);
    tab3.push(lbl_f_name);

    let combo_font = create_native_combobox(hwnd, 110, 484, 160, 100, ID_COMBO_TEXT_FONT, regular, &["Predeterminada (Segoe UI)", "Arial", "Consolas"], 0);
    (*state).hwnd_combo_text_font = combo_font;
    tab3.push(combo_font);

    let lbl_casing = create_native_label(hwnd, "Formato:", 280, 486, 60, 20, regular);
    tab3.push(lbl_casing);

    let combo_casing = create_native_combobox(hwnd, 345, 484, 105, 100, ID_COMBO_TEXT_CASING, regular, &["Original", "minusculas", "MAYUSCULAS"], 0);
    tab3.push(combo_casing);

    let chk_bold = create_native_checkbox(hwnd, "Negrita", 48, 525, 75, 20, ID_CHK_TEXT_BOLD, regular, (*state).settings.text_bold);
    (*state).hwnd_chk_text_bold = chk_bold;
    tab3.push(chk_bold);

    let chk_italic = create_native_checkbox(hwnd, "Cursiva", 130, 525, 80, 20, ID_CHK_TEXT_ITALIC, regular, (*state).settings.text_italic);
    (*state).hwnd_chk_text_italic = chk_italic;
    tab3.push(chk_italic);

    let chk_under = create_native_checkbox(hwnd, "Subrayado", 220, 525, 100, 20, ID_CHK_TEXT_UNDERLINE, regular, (*state).settings.text_underline);
    (*state).hwnd_chk_text_underline = chk_under;
    tab3.push(chk_under);

    let btn_t_color = create_native_btn(hwnd, "Color de texto", 48, 560, 125, 28, ID_BTN_TEXT_COLOR, regular);
    (*state).hwnd_btn_text_color = btn_t_color;
    tab3.push(btn_t_color);

    let hex_t_color = format!("h00{:02X}{:02X}{:02X}", (*state).settings.text_color[0], (*state).settings.text_color[1], (*state).settings.text_color[2]);
    let lbl_t_col = create_native_label(hwnd, &hex_t_color, 185, 564, 120, 20, bold);
    (*state).hwnd_lbl_text_color_hex = lbl_t_col;
    tab3.push(lbl_t_col);

    let f_size_txt = format!("Tamano: {:.0} pt", (*state).settings.text_font_size);
    let lbl_f_size = create_native_label(hwnd, &f_size_txt, 48, 602, 115, 20, regular);
    (*state).hwnd_lbl_text_font_size = lbl_f_size;
    tab3.push(lbl_f_size);

    let track_f_size = {
        let hwnd_track = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("msctls_trackbar32"),
            PCWSTR::null(),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            170, 600, 280, 26,
            hwnd,
            HMENU(ID_TRACK_TEXT_FONT_SIZE as usize as isize),
            instance,
            None,
        );
        SendMessageW(hwnd_track, TBM_SETRANGE, WPARAM(1), LPARAM(((8 & 0xFFFF) | ((36 & 0xFFFF) << 16)) as isize));
        SendMessageW(hwnd_track, TBM_SETPOS, WPARAM(1), LPARAM((*state).settings.text_font_size as isize));
        hwnd_track
    };
    (*state).hwnd_track_text_font_size = track_f_size;
    tab3.push(track_f_size);

    // Panel Derecho: Posicion del texto
    let btn_t_up = create_native_btn(hwnd, " ^ ", 645, 452, 42, 30, ID_BTN_TEXT_UP, bold);
    tab3.push(btn_t_up);
    let btn_t_left = create_native_btn(hwnd, " < ", 595, 488, 42, 30, ID_BTN_TEXT_LEFT, bold);
    tab3.push(btn_t_left);
    let btn_t_down = create_native_btn(hwnd, " v ", 645, 488, 42, 30, ID_BTN_TEXT_DOWN, bold);
    tab3.push(btn_t_down);
    let btn_t_right = create_native_btn(hwnd, " > ", 695, 488, 42, 30, ID_BTN_TEXT_RIGHT, bold);
    tab3.push(btn_t_right);
    let btn_t_c = create_native_btn(hwnd, "Centrar", 610, 528, 112, 28, ID_BTN_TEXT_CENTER, regular);
    tab3.push(btn_t_c);

    let lbl_t_step = create_native_label(hwnd, "Pixeles por clic:", 540, 580, 120, 20, regular);
    tab3.push(lbl_t_step);

    let combo_t_step = create_native_combobox(hwnd, 670, 578, 90, 100, ID_COMBO_TEXT_STEP, regular, &["1", "2", "5", "10", "20"], 2);
    (*state).hwnd_combo_text_step = combo_t_step;
    tab3.push(combo_t_step);

    let lbl_tip = create_native_label(hwnd, "Sugerencia: El texto inferior se adapta automaticamente a la dimension del codigo.", 530, 625, 260, 45, regular);
    tab3.push(lbl_tip);

    (*state).tab_controls[3] = tab3;
}
