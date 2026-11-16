use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::Graphics::Gdi::HFONT;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;

use super::ids::BST_CHECKED;
use super::state::to_pcwstr;

pub fn get_instance() -> HINSTANCE {
    unsafe { GetModuleHandleW(None).unwrap_or_default().into() }
}

pub fn create_ownerdraw_btn(parent: HWND, text: &str, x: i32, y: i32, w: i32, h: i32, id: u16, font: HFONT) -> HWND {
    unsafe {
        let instance = get_instance();
        let wtext = to_pcwstr(text);
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_OWNERDRAW as u32),
            x, y, w, h,
            parent,
            HMENU(id as usize as isize),
            instance,
            None,
        );
        SendMessageW(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
        hwnd
    }
}

pub fn create_native_radio(parent: HWND, text: &str, x: i32, y: i32, w: i32, h: i32, id: u16, font: HFONT, checked: bool) -> HWND {
    unsafe {
        let instance = get_instance();
        let wtext = to_pcwstr(text);
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_AUTORADIOBUTTON as u32),
            x, y, w, h,
            parent,
            HMENU(id as usize as isize),
            instance,
            None,
        );
        SendMessageW(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
        if checked {
            SendMessageW(hwnd, BM_SETCHECK, WPARAM(BST_CHECKED), LPARAM(0));
        }
        hwnd
    }
}

pub fn create_native_btn(parent: HWND, text: &str, x: i32, y: i32, w: i32, h: i32, id: u16, font: HFONT) -> HWND {
    unsafe {
        let instance = get_instance();
        let wtext = to_pcwstr(text);
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_PUSHBUTTON as u32),
            x, y, w, h,
            parent,
            HMENU(id as usize as isize),
            instance,
            None,
        );
        SendMessageW(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
        hwnd
    }
}

pub fn create_native_label(parent: HWND, text: &str, x: i32, y: i32, w: i32, h: i32, font: HFONT) -> HWND {
    unsafe {
        let instance = get_instance();
        let wtext = to_pcwstr(text);
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("STATIC"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE,
            x, y, w, h,
            parent,
            HMENU(0),
            instance,
            None,
        );
        SendMessageW(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
        hwnd
    }
}

pub fn create_native_groupbox(parent: HWND, text: &str, x: i32, y: i32, w: i32, h: i32, font: HFONT) -> HWND {
    unsafe {
        let instance = get_instance();
        let wtext = to_pcwstr(text);
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_GROUPBOX as u32),
            x, y, w, h,
            parent,
            HMENU(0),
            instance,
            None,
        );
        SendMessageW(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
        hwnd
    }
}

pub fn create_native_checkbox(parent: HWND, text: &str, x: i32, y: i32, w: i32, h: i32, id: u16, font: HFONT, checked: bool) -> HWND {
    unsafe {
        let instance = get_instance();
        let wtext = to_pcwstr(text);
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            PCWSTR(wtext.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_AUTOCHECKBOX as u32),
            x, y, w, h,
            parent,
            HMENU(id as usize as isize),
            instance,
            None,
        );
        SendMessageW(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
        if checked {
            SendMessageW(hwnd, BM_SETCHECK, WPARAM(BST_CHECKED), LPARAM(0));
        }
        hwnd
    }
}

pub fn create_native_combobox(parent: HWND, x: i32, y: i32, w: i32, h: i32, id: u16, font: HFONT, items: &[&str], selected: usize) -> HWND {
    unsafe {
        let instance = get_instance();
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("COMBOBOX"),
            PCWSTR::null(),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_VSCROLL | WINDOW_STYLE(CBS_DROPDOWNLIST as u32),
            x, y, w, h,
            parent,
            HMENU(id as usize as isize),
            instance,
            None,
        );
        SendMessageW(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
        for item in items {
            let witem = to_pcwstr(item);
            let _ = SendMessageW(hwnd, CB_ADDSTRING, WPARAM(0), LPARAM(witem.as_ptr() as isize));
        }
        SendMessageW(hwnd, CB_SETCURSEL, WPARAM(selected), LPARAM(0));
        hwnd
    }
}
