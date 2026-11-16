use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::ui::state::{get_state, to_pcwstr};

pub fn show_programming_dialog(parent: HWND) {
    unsafe {
        let payload = if let Some(state) = get_state() {
            state.input_text.clone()
        } else {
            "https://pirulug.com".to_string()
        };

        let text = format!(
            "Generador de codigo en Rust:\n\n\
            use qrcode::QrCode;\n\
            use image::Luma;\n\n\
            let code = QrCode::new(b\"{}\").unwrap();\n\
            let image = code.render::<Luma<u8>>().build();\n\
            image.save(\"codigo_qr.png\").unwrap();",
            payload
        );

        let wtext = to_pcwstr(&text);
        MessageBoxW(parent, PCWSTR(wtext.as_ptr()), w!("Programar un codigo QR en Rust"), MB_OK | MB_ICONINFORMATION);
    }
}
