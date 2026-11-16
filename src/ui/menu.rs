use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreateMenu, CreatePopupMenu, DestroyMenu, TrackPopupMenu, HMENU, MF_POPUP,
    MF_SEPARATOR, MF_STRING, TPM_LEFTALIGN, TPM_TOPALIGN,
};

use super::ids::*;

pub fn build_main_menu() -> HMENU {
    unsafe {
        let menu_bar = CreateMenu().unwrap_or(HMENU(0));

        // Submenu Archivo
        let menu_file = CreatePopupMenu().unwrap_or(HMENU(0));
        let menu_export = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu_export, MF_STRING, ID_EXPORT_PNG as usize, w!("PNG"));
        let _ = AppendMenuW(menu_export, MF_STRING, ID_EXPORT_JPEG as usize, w!("JPEG"));
        let _ = AppendMenuW(menu_export, MF_STRING, ID_EXPORT_SVG as usize, w!("SVG vectorial"));
        let _ = AppendMenuW(menu_export, MF_STRING, ID_EXPORT_EPS as usize, w!("EPS"));
        let _ = AppendMenuW(menu_export, MF_STRING, ID_EXPORT_BASE64 as usize, w!("Base64"));
        let _ = AppendMenuW(menu_export, MF_STRING, ID_EXPORT_MARKDOWN as usize, w!("MarkDown"));

        let _ = AppendMenuW(menu_file, MF_STRING, ID_FILE_NEW as usize, w!("Nuevo\tCtrl+N"));
        let _ = AppendMenuW(menu_file, MF_STRING, ID_FILE_OPEN as usize, w!("Abrir..."));
        let _ = AppendMenuW(menu_file, MF_STRING, ID_FILE_SAVE as usize, w!("Guardar como..."));
        let _ = AppendMenuW(menu_file, MF_SEPARATOR, 0, PCWSTR::null());
        let _ = AppendMenuW(menu_file, MF_POPUP, menu_export.0 as usize, w!("Exportar Codigo QR"));
        let _ = AppendMenuW(menu_file, MF_SEPARATOR, 0, PCWSTR::null());
        let _ = AppendMenuW(menu_file, MF_STRING, ID_FILE_PRINT as usize, w!("Imprimir\tCtrl+P"));
        let _ = AppendMenuW(menu_file, MF_STRING, ID_FILE_REFRESH as usize, w!("Actualizar\tF12"));
        let _ = AppendMenuW(menu_file, MF_SEPARATOR, 0, PCWSTR::null());
        let _ = AppendMenuW(menu_file, MF_STRING, ID_FILE_EXIT as usize, w!("Salir\tCtrl+Q"));

        // Submenu Edicion
        let menu_edit = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu_edit, MF_STRING, ID_EDIT_COPY as usize, w!("Copiar texto"));
        let _ = AppendMenuW(menu_edit, MF_STRING, ID_EDIT_CLEAR as usize, w!("Limpiar entrada"));

        // Submenu Plantillas QR
        let menu_templates = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_TEXT as usize, w!("Texto libre"));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_URL as usize, w!("Enlace (URL)"));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_TWITTER as usize, w!("Tweet de X (Twitter)"));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_VCARD as usize, w!("Contacto (vCard)"));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_WIFI as usize, w!("Red WiFi (WLAN)"));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_DATE as usize, w!("Fecha / Evento"));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_PAYPAL as usize, w!("Pago PayPal"));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_IBAN as usize, w!("Datos bancarios IBAN / BIC"));
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_TOTP as usize, w!("Codigo TOTP (2FA)"));
        let _ = AppendMenuW(menu_templates, MF_SEPARATOR, 0, PCWSTR::null());
        let _ = AppendMenuW(menu_templates, MF_STRING, ID_TEMPLATE_PROGRAMMING as usize, w!("Programar codigo QR"));

        // Submenu Ver
        let menu_view = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu_view, MF_STRING, ID_VIEW_RESET as usize, w!("Restablecer vista"));

        // Submenu Configuracion
        let menu_config = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu_config, MF_STRING, ID_CONFIG_LANG as usize, w!("Idioma: Espanol"));

        // Submenu Ayuda
        let menu_help = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu_help, MF_STRING, ID_HELP_ABOUT as usize, w!("Acerca de QR Generator"));

        let _ = AppendMenuW(menu_bar, MF_POPUP, menu_file.0 as usize, w!("Archivo"));
        let _ = AppendMenuW(menu_bar, MF_POPUP, menu_edit.0 as usize, w!("Edicion"));
        let _ = AppendMenuW(menu_bar, MF_POPUP, menu_templates.0 as usize, w!("Plantillas QR"));
        let _ = AppendMenuW(menu_bar, MF_POPUP, menu_view.0 as usize, w!("Ver"));
        let _ = AppendMenuW(menu_bar, MF_POPUP, menu_config.0 as usize, w!("Configuracion"));
        let _ = AppendMenuW(menu_bar, MF_POPUP, menu_help.0 as usize, w!("Ayuda"));

        menu_bar
    }
}

pub fn show_templates_popup(hwnd: HWND) {
    unsafe {
        let menu = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_TEXT as usize, w!("Texto libre"));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_URL as usize, w!("Enlace (URL)"));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_TWITTER as usize, w!("Tweet de X (Twitter)"));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_VCARD as usize, w!("Contacto (vCard)"));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_WIFI as usize, w!("Red WiFi (WLAN)"));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_DATE as usize, w!("Fecha / Evento"));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_PAYPAL as usize, w!("Pago PayPal"));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_IBAN as usize, w!("Datos bancarios IBAN / BIC"));
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_TOTP as usize, w!("Codigo TOTP (2FA)"));
        let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
        let _ = AppendMenuW(menu, MF_STRING, ID_TEMPLATE_PROGRAMMING as usize, w!("Programar codigo QR"));

        let mut pt = POINT { x: 104, y: 32 };
        ClientToScreen(hwnd, &mut pt);
        TrackPopupMenu(menu, TPM_LEFTALIGN | TPM_TOPALIGN, pt.x, pt.y, 0, hwnd, None);
        let _ = DestroyMenu(menu);
    }
}

pub fn show_export_popup(hwnd: HWND) {
    unsafe {
        let menu = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu, MF_STRING, ID_EXPORT_PNG as usize, w!("PNG"));
        let _ = AppendMenuW(menu, MF_STRING, ID_EXPORT_JPEG as usize, w!("JPEG"));
        let _ = AppendMenuW(menu, MF_STRING, ID_EXPORT_SVG as usize, w!("SVG vectorial"));
        let _ = AppendMenuW(menu, MF_STRING, ID_EXPORT_EPS as usize, w!("EPS"));
        let _ = AppendMenuW(menu, MF_STRING, ID_EXPORT_BASE64 as usize, w!("Base64"));
        let _ = AppendMenuW(menu, MF_STRING, ID_EXPORT_MARKDOWN as usize, w!("MarkDown"));

        let mut pt = POINT { x: 377, y: 32 };
        ClientToScreen(hwnd, &mut pt);
        TrackPopupMenu(menu, TPM_LEFTALIGN | TPM_TOPALIGN, pt.x, pt.y, 0, hwnd, None);
        let _ = DestroyMenu(menu);
    }
}

pub fn show_tools_popup(hwnd: HWND) {
    unsafe {
        let menu = CreatePopupMenu().unwrap_or(HMENU(0));
        let _ = AppendMenuW(menu, MF_STRING, ID_BTN_RANDOM_COLORS as usize, w!("Generar colores aleatorios"));
        let _ = AppendMenuW(menu, MF_STRING, ID_BTN_SWAP_COLORS as usize, w!("Intercambiar frente y fondo"));

        let mut pt = POINT { x: 470, y: 32 };
        ClientToScreen(hwnd, &mut pt);
        TrackPopupMenu(menu, TPM_LEFTALIGN | TPM_TOPALIGN, pt.x, pt.y, 0, hwnd, None);
        let _ = DestroyMenu(menu);
    }
}
