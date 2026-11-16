use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Controls::TCM_SETCURSEL;
use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, SetWindowTextW};

use super::modal::show_single_input_dialog;
use crate::ui::render::regenerate_qr;
use crate::ui::state::{get_state, to_pcwstr};
use crate::ui::tabs::update_tab_visibility;

pub fn prompt_url_dialog(parent: HWND) {
    if let Some(url) = show_single_input_dialog(parent, "Plantilla: Enlace Web (URL)", "Introduce la direccion URL (ej: https://pirulug.com):", "https://") {
        apply_template_text(&url);
    }
}

pub fn prompt_twitter_dialog(parent: HWND) {
    if let Some(user) = show_single_input_dialog(parent, "Plantilla: Tweet de X (Twitter)", "Nombre de usuario o texto del tweet:", "pirulug") {
        let full = if user.starts_with('@') || !user.contains(' ') {
            format!("https://x.com/{}", user.trim_start_matches('@'))
        } else {
            format!("https://x.com/intent/tweet?text={}", url_encode(&user))
        };
        apply_template_text(&full);
    }
}

pub fn prompt_wifi_dialog(parent: HWND) {
    if let Some(ssid) = show_single_input_dialog(parent, "Plantilla: Red Wi-Fi (WLAN)", "Introduce el nombre de la red (SSID):", "MiRedWiFi") {
        let wifi_qr = format!("WIFI:T:WPA;S:{};P:ClaveSecreta123;;", ssid);
        apply_template_text(&wifi_qr);
    }
}

pub fn prompt_vcard_dialog(parent: HWND) {
    if let Some(name) = show_single_input_dialog(parent, "Plantilla: Contacto (vCard)", "Nombre completo del contacto:", "Pirulug Developer") {
        let vcard = format!(
            "BEGIN:VCARD\nVERSION:3.0\nFN:{}\nN:{};;;\nURL:https://pirulug.com\nEND:VCARD",
            name, name
        );
        apply_template_text(&vcard);
    }
}

pub fn prompt_date_dialog(parent: HWND) {
    if let Some(title) = show_single_input_dialog(parent, "Plantilla: Fecha / Evento", "Titulo del evento:", "Reunion Importante") {
        let ical = format!(
            "BEGIN:VEVENT\nSUMMARY:{}\nDTSTART:20261116T100000Z\nDTEND:20261116T110000Z\nEND:VEVENT",
            title
        );
        apply_template_text(&ical);
    }
}

pub fn prompt_paypal_dialog(parent: HWND) {
    if let Some(user) = show_single_input_dialog(parent, "Plantilla: Pago PayPal", "Usuario de PayPal.Me:", "pirulug") {
        let paypal = format!("https://paypal.me/{}", user);
        apply_template_text(&paypal);
    }
}

pub fn prompt_iban_dialog(parent: HWND) {
    if let Some(iban) = show_single_input_dialog(parent, "Plantilla: Datos bancarios IBAN", "Codigo IBAN (ej: ES91 2100 0418 4502 0005 1332):", "ES9121000418450200051332") {
        let epc = format!("BCD\n002\n1\nSCT\n\nPirulug\n{}\nEUR0.00\n\nPago QR", iban.replace(' ', ""));
        apply_template_text(&epc);
    }
}

pub fn prompt_totp_dialog(parent: HWND) {
    if let Some(account) = show_single_input_dialog(parent, "Plantilla: Codigo TOTP (2FA)", "Nombre de cuenta / email:", "usuario@pirulug.com") {
        let totp = format!("otpauth://totp/Pirulug:{}?secret=JBSWY3DPEHPK3PXP&issuer=Pirulug", account);
        apply_template_text(&totp);
    }
}

pub fn apply_template_text(text: &str) {
    if let Some(state) = unsafe { get_state() } {
        state.input_text = text.to_string();
        let wtxt = to_pcwstr(&state.input_text);
        unsafe {
            SetWindowTextW(state.hwnd_edit_input, PCWSTR(wtxt.as_ptr()));
        }
        state.active_tab = 0;
        unsafe {
            SendMessageW(state.hwnd_tab, TCM_SETCURSEL, WPARAM(0), LPARAM(0));
        }
        update_tab_visibility(state);
        regenerate_qr(state);
    }
}

pub fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}
