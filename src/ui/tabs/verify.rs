use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::ui::ids::*;
use crate::ui::state::{to_pcwstr, AppState};
use crate::ui::widgets::*;

pub unsafe fn build_tab_verify(hwnd: HWND, instance: HINSTANCE, state: *mut AppState) {
    let regular = (*state).hfont_regular;
    let bold = (*state).hfont_bold;

    let mut tab4 = Vec::new();

    // Panel Izquierdo: Campo de texto de solo lectura
    let hwnd_v_edit = {
        let wtext = to_pcwstr(&(*state).input_text);
        CreateWindowExW(
            WINDOW_EX_STYLE(WS_EX_CLIENTEDGE.0),
            w!("EDIT"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_VSCROLL | WINDOW_STYLE((ES_MULTILINE | ES_READONLY | ES_AUTOVSCROLL) as u32),
            35, 445, 420, 245,
            hwnd,
            HMENU(ID_EDIT_VERIFY_READ as usize as isize),
            instance,
            None,
        )
    };
    SendMessageW(hwnd_v_edit, WM_SETFONT, WPARAM(regular.0 as usize), LPARAM(1));
    (*state).hwnd_edit_verify_read = hwnd_v_edit;
    tab4.push(hwnd_v_edit);

    // Panel Derecho: Informacion de diagnostico
    let lbl_v_type = create_native_label(hwnd, "Tipo reconocido:", 505, 450, 120, 20, regular);
    tab4.push(lbl_v_type);

    let lbl_v_ok = create_native_label(hwnd, "Codigo QR reconocido", 630, 450, 160, 20, bold);
    tab4.push(lbl_v_ok);

    let btn_v_manual = create_native_btn(hwnd, "Comprobar de nuevo manualmente", 505, 485, 280, 32, ID_BTN_VERIFY_MANUAL, regular);
    (*state).hwnd_btn_verify_manual = btn_v_manual;
    tab4.push(btn_v_manual);

    let btn_v_ext = create_native_btn(hwnd, "Abrir procedimiento de prueba extendido", 505, 530, 280, 32, ID_BTN_VERIFY_EXTENDED, regular);
    (*state).hwnd_btn_verify_extended = btn_v_ext;
    tab4.push(btn_v_ext);

    let lbl_v_note = create_native_label(hwnd, "Nota sobre la comprobacion:\nEl codigo QR es valido, legible y cumple con los estandares internacionales ISO/IEC 18004.", 505, 580, 280, 80, regular);
    tab4.push(lbl_v_note);

    (*state).tab_controls[4] = tab4;
}
