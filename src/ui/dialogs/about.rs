use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::ui::state::to_pcwstr;

pub fn show_about_dialog(parent: HWND) {
    unsafe {
        let text = "QR Generator v0.0.1 (Edicion Rust)\n\n\
                    Generador nativo de codigos QR escrito 100% en Rust con windows-rs.\n\
                    Diseno limpio optimizado para fondo blanco, con soporte para logotipos, paletas de colores y exportacion vectorial.\n\n\
                    Programador: Pirulug\n\
                    Web: https://pirulug.com\n\
                    GitHub: https://github.com/pirulug\n\n\
                    Deseas abrir la web del desarrollador (pirulug.com)?";

        let wtext = to_pcwstr(text);
        let res = MessageBoxW(parent, PCWSTR(wtext.as_ptr()), w!("Acerca de QR Generator"), MB_YESNO | MB_ICONINFORMATION);
        if res == IDYES {
            ShellExecuteW(parent, w!("open"), w!("https://pirulug.com"), PCWSTR::null(), PCWSTR::null(), SW_SHOWNORMAL);
        }
    }
}
