use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    GetStockObject, SetBkMode, SetTextColor, HBRUSH, HDC, HFONT, TRANSPARENT, WHITE_BRUSH,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{EnableWindow, SetFocus};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::ui::state::{get_state, to_pcwstr};
use crate::ui::widgets::get_instance;

pub fn show_single_input_dialog(parent: HWND, title: &str, prompt: &str, default_val: &str) -> Option<String> {
    unsafe {
        let instance = get_instance();
        let dlg_class = w!("QRGenInputDlgClass");

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(dlg_wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance.into(),
            hIcon: HICON(0),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or(HCURSOR(0)),
            hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: dlg_class,
            hIconSm: HICON(0),
        };
        RegisterClassExW(&wc);

        let wtitle = to_pcwstr(title);
        let hwnd_dlg = CreateWindowExW(
            WINDOW_EX_STYLE(WS_EX_DLGMODALFRAME.0),
            dlg_class,
            PCWSTR(wtitle.as_ptr()),
            WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 420, 190,
            parent,
            None,
            instance,
            None,
        );

        let hfont = if let Some(state) = get_state() {
            state.hfont_regular
        } else {
            HFONT(0)
        };

        let wprompt = to_pcwstr(prompt);
        let lbl = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("STATIC"),
            PCWSTR(wprompt.as_ptr()),
            WS_CHILD | WS_VISIBLE,
            20, 20, 365, 38,
            hwnd_dlg,
            HMENU(0),
            instance,
            None,
        );
        SendMessageW(lbl, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(1));

        let wdef = to_pcwstr(default_val);
        let edit = CreateWindowExW(
            WINDOW_EX_STYLE(WS_EX_CLIENTEDGE.0),
            w!("EDIT"),
            PCWSTR(wdef.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
            20, 65, 365, 24,
            hwnd_dlg,
            HMENU(2001),
            instance,
            None,
        );
        SendMessageW(edit, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(1));
        let _ = SetFocus(edit);

        let btn_ok = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            w!("Aplicar al QR"),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
            185, 105, 105, 28,
            hwnd_dlg,
            HMENU(1),
            instance,
            None,
        );
        SendMessageW(btn_ok, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(1));

        let btn_cancel = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            w!("Cancelar"),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            300, 105, 85, 28,
            hwnd_dlg,
            HMENU(2),
            instance,
            None,
        );
        SendMessageW(btn_cancel, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(1));

        let _ = EnableWindow(parent, false);

        let mut msg = MSG::default();
        let mut result = None;

        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if msg.hwnd == hwnd_dlg || IsChild(hwnd_dlg, msg.hwnd).as_bool() {
                if msg.message == WM_KEYDOWN && msg.wParam.0 == 13 {
                    // Enter
                    let len = GetWindowTextLengthW(edit) as usize;
                    let mut buf = vec![0u16; len + 1];
                    GetWindowTextW(edit, &mut buf);
                    result = Some(String::from_utf16_lossy(&buf[..len]));
                    break;
                } else if msg.message == WM_KEYDOWN && msg.wParam.0 == 27 {
                    // Escape
                    break;
                }
            }

            if msg.message == WM_COMMAND && (msg.wParam.0 & 0xFFFF) == 1 {
                // Click OK
                let len = GetWindowTextLengthW(edit) as usize;
                let mut buf = vec![0u16; len + 1];
                GetWindowTextW(edit, &mut buf);
                result = Some(String::from_utf16_lossy(&buf[..len]));
                break;
            } else if msg.message == WM_COMMAND && (msg.wParam.0 & 0xFFFF) == 2 {
                // Click Cancel
                break;
            }

            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);

            if !IsWindow(hwnd_dlg).as_bool() {
                break;
            }
        }

        let _ = EnableWindow(parent, true);
        let _ = DestroyWindow(hwnd_dlg);
        let _ = SetFocus(parent);

        result
    }
}

pub unsafe extern "system" fn dlg_wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            let _ = DestroyWindow(hwnd);
            LRESULT(0)
        }
        WM_CTLCOLORSTATIC => {
            let hdc = HDC(wparam.0 as isize);
            SetBkMode(hdc, TRANSPARENT);
            SetTextColor(hdc, COLORREF(0x001E293B));
            LRESULT(GetStockObject(WHITE_BRUSH).0 as isize)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
