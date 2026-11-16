use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::ui::ids::*;
use crate::ui::state::{to_pcwstr, AppState};
use crate::ui::widgets::*;

pub unsafe fn build_tab_input(hwnd: HWND, instance: HINSTANCE, state: *mut AppState) {
    let regular = (*state).hfont_regular;

    let mut tab0 = Vec::new();

    // Panel Izquierdo: Campo de texto de entrada
    let hwnd_edit = {
        let wtext = to_pcwstr(&(*state).input_text);
        CreateWindowExW(
            WINDOW_EX_STYLE(WS_EX_CLIENTEDGE.0),
            w!("EDIT"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_VSCROLL | WINDOW_STYLE((ES_MULTILINE | ES_AUTOVSCROLL | ES_WANTRETURN) as u32),
            38, 445, 444, 180,
            hwnd,
            HMENU(ID_EDIT_INPUT as usize as isize),
            instance,
            None,
        )
    };
    SendMessageW(hwnd_edit, WM_SETFONT, WPARAM(regular.0 as usize), LPARAM(1));
    (*state).hwnd_edit_input = hwnd_edit;
    tab0.push(hwnd_edit);

    let char_count = (*state).input_text.chars().count();
    let word_count = (*state).input_text.split_whitespace().count();
    let char_count_txt = format!("{} caracteres ({} palabras)", char_count, word_count);
    let lbl_count = create_native_label(hwnd, &char_count_txt, 38, 634, 210, 20, regular);
    (*state).hwnd_lbl_char_count = lbl_count;
    tab0.push(lbl_count);

    let chk_use_qr = create_native_checkbox(
        hwnd,
        "Usar \"Texto en QR\" como entrada",
        255, 632, 227, 22,
        ID_CHK_USE_TEXT_AS_INPUT,
        regular,
        (*state).use_text_in_qr_as_input,
    );
    (*state).hwnd_chk_use_text_in_qr = chk_use_qr;
    tab0.push(chk_use_qr);

    let btn_paste = create_native_btn(hwnd, "Pegar", 38, 660, 78, 28, ID_BTN_PASTE_INPUT, regular);
    tab0.push(btn_paste);

    let btn_copy = create_native_btn(hwnd, "Copiar", 122, 660, 78, 28, ID_BTN_COPY_INPUT, regular);
    tab0.push(btn_copy);

    let btn_import = create_native_btn(hwnd, "Importar...", 206, 660, 110, 28, ID_BTN_IMPORT_FILE, regular);
    (*state).hwnd_btn_import = btn_import;
    tab0.push(btn_import);

    let btn_clear = create_native_btn(hwnd, "Vaciar", 322, 660, 78, 28, ID_BTN_CLEAR_INPUT, regular);
    (*state).hwnd_btn_clear = btn_clear;
    tab0.push(btn_clear);

    // Panel Derecho: Acceso directo a plantillas
    let btn_t_url = create_native_btn(hwnd, "Enlace Web (URL)", 530, 445, 260, 32, ID_TEMPLATE_URL, regular);
    tab0.push(btn_t_url);

    let btn_t_wifi = create_native_btn(hwnd, "Red Wi-Fi (WLAN)", 530, 485, 260, 32, ID_TEMPLATE_WIFI, regular);
    tab0.push(btn_t_wifi);

    let btn_t_vcard = create_native_btn(hwnd, "Contacto (vCard)", 530, 525, 260, 32, ID_TEMPLATE_VCARD, regular);
    tab0.push(btn_t_vcard);

    let btn_t_twitter = create_native_btn(hwnd, "Tweet de X (Twitter)", 530, 565, 260, 32, ID_TEMPLATE_TWITTER, regular);
    tab0.push(btn_t_twitter);

    let btn_t_totp = create_native_btn(hwnd, "Codigo TOTP (2FA)", 530, 605, 260, 32, ID_TEMPLATE_TOTP, regular);
    tab0.push(btn_t_totp);

    let btn_t_prog = create_native_btn(hwnd, "Programar codigo QR...", 530, 645, 260, 32, ID_TEMPLATE_PROGRAMMING, regular);
    tab0.push(btn_t_prog);

    (*state).tab_controls[0] = tab0;
}
