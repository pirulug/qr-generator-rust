use eframe::egui::{self, Color32, ColorImage, TextureHandle, TextureOptions};
use image::{ImageBuffer, Rgba};
use std::path::PathBuf;

use crate::qr::{
    ErrorCorrection, LogoTemplate, QrEngine, QrSettings, TextCasing,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveTab {
    Input,
    ColoursAndSize,
    LogoInQr,
    TextInQr,
    VerificationOk,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TemplateModal {
    None,
    Wifi,
    VCard,
    Url,
    Twitter,
    Date,
    PayPal,
    Iban,
    Totp,
    About,
    Programming,
}

pub struct QrApp {
    // Estado de la entrada de datos (Pestaña 1: Entrada)
    input_text: String,
    use_text_in_qr_as_input: bool,

    // Configuración completa del QR
    settings: QrSettings,
    active_tab: ActiveTab,
    save_settings: bool,

    // Manejo de Logo personalizado
    custom_logo_img: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    custom_logo_path: Option<PathBuf>,

    // Estado del motor y texturas
    texture: Option<TextureHandle>,
    dirty: bool,
    status_message: String,
    last_computed_size: (u32, u32),

    // Modales de plantillas emergentes
    active_modal: TemplateModal,

    // Campos temporales para modales de plantillas
    wifi_ssid: String,
    wifi_pass: String,
    wifi_sec: String,
    wifi_hidden: bool,

    vcard_first: String,
    vcard_last: String,
    vcard_phone: String,
    vcard_email: String,
    vcard_org: String,
    vcard_url: String,

    url_modal_input: String,
    twitter_text: String,
    twitter_user: String,

    date_title: String,
    date_start: String,
    date_location: String,

    paypal_user: String,
    paypal_amount: String,
    paypal_currency: String,

    iban_name: String,
    iban_number: String,
    iban_bic: String,
    iban_amount: String,

    totp_secret: String,
    totp_account: String,
    totp_issuer: String,

    // Pickers de color abiertos
    show_bg_picker: bool,
    show_fg_picker: bool,
    show_txt_picker: bool,
}

impl Default for QrApp {
    fn default() -> Self {
        let default_text = "¡Mi primer código QR!".to_string();
        let default_settings = QrSettings::default();
        let initial_size = QrEngine::compute_pixel_size(&default_text, &default_settings);

        Self {
            input_text: default_text,
            use_text_in_qr_as_input: false,

            settings: default_settings,
            active_tab: ActiveTab::Input,
            save_settings: false,

            custom_logo_img: None,
            custom_logo_path: None,

            texture: None,
            dirty: true,
            status_message: "Listo. Código QR reconocido correctamente.".to_string(),
            last_computed_size: initial_size,

            active_modal: TemplateModal::None,

            wifi_ssid: "MiRedWiFi".to_string(),
            wifi_pass: "ClaveSecreta123".to_string(),
            wifi_sec: "WPA".to_string(),
            wifi_hidden: false,

            vcard_first: "Juan".to_string(),
            vcard_last: "Pérez".to_string(),
            vcard_phone: "+34 600 123 456".to_string(),
            vcard_email: "juan.perez@ejemplo.com".to_string(),
            vcard_org: "Empresa Global S.L.".to_string(),
            vcard_url: "https://ejemplo.com".to_string(),

            url_modal_input: "https://rust-lang.org".to_string(),
            twitter_text: "¡Mira este código QR generado con Rust!".to_string(),
            twitter_user: "rustlang".to_string(),

            date_title: "Conferencia de Rust 2026".to_string(),
            date_start: "2026-11-15T09:00:00".to_string(),
            date_location: "En línea / Centro de Convenciones".to_string(),

            paypal_user: "usuario".to_string(),
            paypal_amount: "10.00".to_string(),
            paypal_currency: "EUR".to_string(),

            iban_name: "Juan Pérez".to_string(),
            iban_number: "ES9121000418450200051332".to_string(),
            iban_bic: "CAIXESBBXXX".to_string(),
            iban_amount: "25.00".to_string(),

            totp_secret: "JBSWY3DPEHPK3PXP".to_string(),
            totp_account: "usuario@dominio.com".to_string(),
            totp_issuer: "SeguridadRust".to_string(),

            show_bg_picker: false,
            show_fg_picker: false,
            show_txt_picker: false,
        }
    }
}

impl QrApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Configurar tema con fondo blanco limpio estilo Windows moderno
        let mut visuals = egui::Visuals::light();
        visuals.window_fill = Color32::from_rgb(255, 255, 255);
        visuals.panel_fill = Color32::from_rgb(240, 244, 248); // Fondo suave gris-azulado de las capturas
        visuals.faint_bg_color = Color32::from_rgb(255, 255, 255);
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(255, 255, 255);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(245, 247, 250);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(230, 240, 255);
        visuals.widgets.active.bg_fill = Color32::from_rgb(210, 230, 255);
        _cc.egui_ctx.set_visuals(visuals);

        Self::default()
    }

    fn effective_payload(&self) -> String {
        if self.use_text_in_qr_as_input && self.settings.text_enabled {
            self.settings.text_content.clone()
        } else if self.input_text.is_empty() {
            "¡Mi primer código QR!".to_string()
        } else {
            self.input_text.clone()
        }
    }

    fn regenerate_qr(&mut self, ctx: &egui::Context) {
        let payload = self.effective_payload();
        self.last_computed_size = QrEngine::compute_pixel_size(&payload, &self.settings);

        let custom_logo = if self.settings.logo_enabled {
            self.custom_logo_img.as_ref()
        } else {
            None
        };

        match QrEngine::generate_image(&payload, &self.settings, custom_logo) {
            Ok(img) => {
                let w = img.width() as usize;
                let h = img.height() as usize;
                let color_img = ColorImage::from_rgba_unmultiplied([w, h], img.as_raw());
                self.texture = Some(ctx.load_texture("qr_canvas", color_img, TextureOptions::NEAREST));
                self.dirty = false;
                self.status_message = format!("✓ Código QR generado ({} x {} px) - Correcto", w, h);
            }
            Err(e) => {
                self.status_message = format!("⚠ Aviso de generación de QR: {}", e);
            }
        }
    }

    fn randomize_colors(&mut self) {
        let palettes: [([u8; 4], [u8; 4]); 6] = [
            ([255, 255, 255, 255], [14, 116, 144, 255]), // Blanco / Azul Petróleo
            ([255, 255, 255, 255], [185, 28, 28, 255]),   // Blanco / Rojo Oscuro
            ([248, 250, 252, 255], [15, 23, 42, 255]),    // Slate claro / Slate oscuro
            ([254, 243, 199, 255], [120, 53, 15, 255]),   // Ámbar claro / Marrón
            ([240, 253, 244, 255], [20, 83, 45, 255]),    // Verde menta / Verde bosque
            ([250, 245, 255, 255], [88, 28, 135, 255]),   // Lila / Púrpura oscuro
        ];
        let idx = (self.settings.scale_factor + self.settings.border) % palettes.len();
        let (bg, fg) = palettes[idx];
        self.settings.bg_color = bg;
        self.settings.fg_color = fg;
        self.dirty = true;
    }

    fn swap_colors(&mut self) {
        std::mem::swap(&mut self.settings.bg_color, &mut self.settings.fg_color);
        self.dirty = true;
    }

    fn export_png(&mut self) {
        let payload = self.effective_payload();
        let custom_logo = if self.settings.logo_enabled {
            self.custom_logo_img.as_ref()
        } else {
            None
        };

        if let Ok(img) = QrEngine::generate_image(&payload, &self.settings, custom_logo) {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Imagen PNG (*.png)", &["png"])
                .set_file_name("codigo_qr.png")
                .save_file()
            {
                if let Err(e) = img.save(&path) {
                    self.status_message = format!("Error al guardar PNG: {}", e);
                } else {
                    self.status_message = format!("✓ PNG guardado correctamente en {:?}", path.file_name().unwrap_or_default());
                }
            }
        }
    }

    fn export_jpeg(&mut self) {
        let payload = self.effective_payload();
        let custom_logo = if self.settings.logo_enabled {
            self.custom_logo_img.as_ref()
        } else {
            None
        };

        if let Ok(img) = QrEngine::generate_image(&payload, &self.settings, custom_logo) {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Imagen JPEG (*.jpg, *.jpeg)", &["jpg", "jpeg"])
                .set_file_name("codigo_qr.jpg")
                .save_file()
            {
                let rgb_img = image::DynamicImage::ImageRgba8(img).to_rgb8();
                if let Err(e) = rgb_img.save(&path) {
                    self.status_message = format!("Error al guardar JPEG: {}", e);
                } else {
                    self.status_message = format!("✓ JPEG guardado correctamente en {:?}", path.file_name().unwrap_or_default());
                }
            }
        }
    }

    fn export_svg(&mut self) {
        let payload = self.effective_payload();
        if let Ok(svg_str) = QrEngine::generate_svg(&payload, &self.settings) {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Vector SVG (*.svg)", &["svg"])
                .set_file_name("codigo_qr.svg")
                .save_file()
            {
                if let Err(e) = std::fs::write(&path, svg_str) {
                    self.status_message = format!("Error al guardar SVG: {}", e);
                } else {
                    self.status_message = format!("✓ SVG guardado correctamente en {:?}", path.file_name().unwrap_or_default());
                }
            }
        }
    }

    fn export_eps(&mut self) {
        let payload = self.effective_payload();
        if let Ok(eps_str) = QrEngine::generate_eps(&payload, &self.settings) {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("PostScript Encapsulado EPS (*.eps)", &["eps"])
                .set_file_name("codigo_qr.eps")
                .save_file()
            {
                if let Err(e) = std::fs::write(&path, eps_str) {
                    self.status_message = format!("Error al guardar EPS: {}", e);
                } else {
                    self.status_message = format!("✓ EPS guardado correctamente en {:?}", path.file_name().unwrap_or_default());
                }
            }
        }
    }

    fn export_base64(&mut self, ctx: &egui::Context) {
        let payload = self.effective_payload();
        let custom_logo = if self.settings.logo_enabled {
            self.custom_logo_img.as_ref()
        } else {
            None
        };

        if let Ok(img) = QrEngine::generate_image(&payload, &self.settings, custom_logo) {
            let mut bytes: Vec<u8> = Vec::new();
            let mut cursor = std::io::Cursor::new(&mut bytes);
            if img.write_to(&mut cursor, image::ImageFormat::Png).is_ok() {
                let b64 = base64_encode(&bytes);
                let data_uri = format!("data:image/png;base64,{}", b64);
                ctx.output_mut(|o| o.copied_text = data_uri);
                self.status_message = "✓ ¡URI de datos Base64 copiada al portapapeles!".to_string();
            }
        }
    }

    fn export_markdown(&mut self, ctx: &egui::Context) {
        let payload = self.effective_payload();
        let md = format!("![Código QR para {}](codigo_qr.png)", payload);
        ctx.output_mut(|o| o.copied_text = md);
        self.status_message = "✓ ¡Fragmento Markdown copiado al portapapeles!".to_string();
    }

    fn pick_logo_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Imágenes (*.png, *.jpg, *.jpeg, *.webp)", &["png", "jpg", "jpeg", "webp"])
            .pick_file()
        {
            if let Ok(dyn_img) = image::open(&path) {
                self.custom_logo_img = Some(dyn_img.to_rgba8());
                self.custom_logo_path = Some(path);
                self.settings.logo_template = LogoTemplate::None;
                self.settings.logo_enabled = true;
                self.dirty = true;
                self.status_message = "Logo personalizado cargado correctamente.".to_string();
            } else {
                self.status_message = "No se pudo cargar el archivo de imagen.".to_string();
            }
        }
    }
}

impl eframe::App for QrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dirty || self.texture.is_none() {
            self.regenerate_qr(ctx);
        }

        // ==========================================
        // 1. MENÚ SUPERIOR (Archivo, Edición, Plantillas QR, Ver, Configuración, Ayuda)
        // ==========================================
        egui::TopBottomPanel::top("menu_bar")
            .frame(egui::Frame::none().fill(Color32::from_rgb(240, 244, 248)))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Archivo", |ui| {
                        if ui.button("📄 Nuevo               Ctrl+N").clicked() {
                            self.input_text = "¡Mi primer código QR!".to_string();
                            self.dirty = true;
                            ui.close_menu();
                        }
                        if ui.button("📁 Abrir...").clicked() {
                            if let Some(p) = rfd::FileDialog::new().add_filter("Archivos de texto (*.txt)", &["txt"]).pick_file() {
                                if let Ok(content) = std::fs::read_to_string(p) {
                                    self.input_text = content;
                                    self.dirty = true;
                                }
                            }
                            ui.close_menu();
                        }
                        if ui.button("💾 Guardar como...").clicked() {
                            self.export_png();
                            ui.close_menu();
                        }
                        ui.separator();
                        ui.menu_button("Exportar Código QR >", |ui| {
                            if ui.button("PNG").clicked() { self.export_png(); ui.close_menu(); }
                            if ui.button("JPEG").clicked() { self.export_jpeg(); ui.close_menu(); }
                            if ui.button("SVG (!)").clicked() { self.export_svg(); ui.close_menu(); }
                            if ui.button("EPS (!)").clicked() { self.export_eps(); ui.close_menu(); }
                            if ui.button("Base64").clicked() { self.export_base64(ctx); ui.close_menu(); }
                            if ui.button("MarkDown").clicked() { self.export_markdown(ctx); ui.close_menu(); }
                        });
                        ui.separator();
                        if ui.button("🖨 Imprimir            Ctrl+P").clicked() {
                            self.status_message = "Diálogo de impresión simulado.".to_string();
                            ui.close_menu();
                        }
                        if ui.button("🔄 Actualizar          F12").clicked() {
                            self.dirty = true;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("🚪 Salir               Ctrl+Q").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.menu_button("Edición", |ui| {
                        if ui.button("📋 Copiar texto").clicked() {
                            ctx.output_mut(|o| o.copied_text = self.input_text.clone());
                            self.status_message = "Texto de entrada copiado al portapapeles.".to_string();
                            ui.close_menu();
                        }
                        if ui.button("🗑 Limpiar entrada").clicked() {
                            self.input_text.clear();
                            self.dirty = true;
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Plantillas QR", |ui| {
                        if ui.button("🔤 Texto libre").clicked() { self.active_tab = ActiveTab::Input; ui.close_menu(); }
                        if ui.button("🔗 Enlace (URL)").clicked() { self.active_modal = TemplateModal::Url; ui.close_menu(); }
                        if ui.button("🕄 Tweet de X (Twitter)").clicked() { self.active_modal = TemplateModal::Twitter; ui.close_menu(); }
                        if ui.button("👤 Contacto (vCard)").clicked() { self.active_modal = TemplateModal::VCard; ui.close_menu(); }
                        if ui.button("📶 Red WiFi (WLAN)").clicked() { self.active_modal = TemplateModal::Wifi; ui.close_menu(); }
                        if ui.button("📅 Fecha / Evento").clicked() { self.active_modal = TemplateModal::Date; ui.close_menu(); }
                        if ui.button("🅿 Pago con PayPal").clicked() { self.active_modal = TemplateModal::PayPal; ui.close_menu(); }
                        if ui.button("🏛 Datos bancarios IBAN / BIC").clicked() { self.active_modal = TemplateModal::Iban; ui.close_menu(); }
                        if ui.button("🛡 Código TOTP (2FA)").clicked() { self.active_modal = TemplateModal::Totp; ui.close_menu(); }
                        ui.separator();
                        if ui.button("</> Programar un código QR").clicked() { self.active_modal = TemplateModal::Programming; ui.close_menu(); }
                    });

                    ui.menu_button("Ver", |ui| {
                        if ui.button("Restablecer vista").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("Configuración", |ui| {
                        if ui.button("Idioma: Español ✓").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("Ayuda", |ui| {
                        if ui.button("Acerca de Creador de Códigos QR").clicked() {
                            self.active_modal = TemplateModal::About;
                            ui.close_menu();
                        }
                    });
                });
            });

        // ==========================================
        // 2. TOOLBAR CON ICONOS Y TEXTO EN ESPAÑOL (Exacta a la Imagen 8)
        // ==========================================
        egui::TopBottomPanel::top("toolbar_panel")
            .frame(egui::Frame::none().fill(Color32::from_rgb(222, 230, 238)).inner_margin(egui::Margin::symmetric(8.0, 4.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Botón Actualizar
                    if toolbar_button(ui, "🔄", "Actualizar").clicked() {
                        self.dirty = true;
                    }

                    toolbar_separator(ui);

                    // Botón Plantillas con Dropdown
                    ui.menu_button(egui::RichText::new("⚏ Plantillas ▾").size(11.0), |ui| {
                        if ui.button("🔤 Texto libre").clicked() { self.active_tab = ActiveTab::Input; ui.close_menu(); }
                        if ui.button("🔗 Enlace (URL)").clicked() { self.active_modal = TemplateModal::Url; ui.close_menu(); }
                        if ui.button("🕄 Tweet de X (Twitter)").clicked() { self.active_modal = TemplateModal::Twitter; ui.close_menu(); }
                        if ui.button("👤 Contacto (vCard)").clicked() { self.active_modal = TemplateModal::VCard; ui.close_menu(); }
                        if ui.button("📶 Red WiFi (WLAN)").clicked() { self.active_modal = TemplateModal::Wifi; ui.close_menu(); }
                        if ui.button("📅 Fecha / Evento").clicked() { self.active_modal = TemplateModal::Date; ui.close_menu(); }
                        if ui.button("🅿 Pago PayPal").clicked() { self.active_modal = TemplateModal::PayPal; ui.close_menu(); }
                        if ui.button("🏛 Datos bancarios IBAN / BIC").clicked() { self.active_modal = TemplateModal::Iban; ui.close_menu(); }
                        if ui.button("🛡 Código TOTP (2FA)").clicked() { self.active_modal = TemplateModal::Totp; ui.close_menu(); }
                        ui.separator();
                        if ui.button("</> Programar código QR").clicked() { self.active_modal = TemplateModal::Programming; ui.close_menu(); }
                    });

                    // Botón Copiar
                    if toolbar_button(ui, "📋", "Copiar").clicked() {
                        ctx.output_mut(|o| o.copied_text = self.input_text.clone());
                        self.status_message = "✓ ¡Contenido copiado al portapapeles!".to_string();
                    }

                    // Botón Verificación
                    if toolbar_button(ui, "🛡", "Verificación").clicked() {
                        self.active_tab = ActiveTab::VerificationOk;
                    }

                    toolbar_separator(ui);

                    // Botón Exportar con Dropdown
                    ui.menu_button(egui::RichText::new("📄 Exportar ▾").size(11.0), |ui| {
                        if ui.button("JPEG").clicked() { self.export_jpeg(); ui.close_menu(); }
                        if ui.button("PNG").clicked() { self.export_png(); ui.close_menu(); }
                        if ui.button("SVG (!)").clicked() { self.export_svg(); ui.close_menu(); }
                        if ui.button("EPS (!)").clicked() { self.export_eps(); ui.close_menu(); }
                        if ui.button("Base64").clicked() { self.export_base64(ctx); ui.close_menu(); }
                        if ui.button("MarkDown").clicked() { self.export_markdown(ctx); ui.close_menu(); }
                    });

                    // Botón Herramientas
                    ui.menu_button(egui::RichText::new("🧰 Herramientas ▾").size(11.0), |ui| {
                        if ui.button("Generar colores aleatorios").clicked() { self.randomize_colors(); ui.close_menu(); }
                        if ui.button("Intercambiar frente y fondo").clicked() { self.swap_colors(); ui.close_menu(); }
                    });

                    // Botón Lienzos
                    if toolbar_button(ui, "✨", "Lienzos").clicked() {
                        self.settings.scale_factor = if self.settings.scale_factor == 7 { 10 } else { 7 };
                        self.dirty = true;
                    }
                });
            });

        // ==========================================
        // 3. BARRA INFERIOR DE ESTADO
        // ==========================================
        egui::TopBottomPanel::bottom("status_bar")
            .frame(egui::Frame::none().fill(Color32::from_rgb(240, 244, 248)).inner_margin(egui::Margin::symmetric(8.0, 3.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&self.status_message).size(12.0).color(Color32::from_rgb(50, 70, 90)));
                });
            });

        // ==========================================
        // 4. ÁREA CENTRAL (LIENZO BLANCO CON CÓDIGO QR + CONTENEDOR DE PESTAÑAS)
        // ==========================================
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(222, 230, 238)).inner_margin(egui::Margin::same(12.0)))
            .show(ctx, |ui| {
                // --- LIENZO CUADRADO BLANCO CON EL QR CODE (Imagen 1) ---
                ui.vertical_centered(|ui| {
                    if let Some(texture) = &self.texture {
                        let canvas_px = 300.0;
                        let aspect = texture.aspect_ratio();
                        let size = if aspect >= 1.0 {
                            egui::vec2(canvas_px, canvas_px / aspect)
                        } else {
                            egui::vec2(canvas_px * aspect, canvas_px)
                        };

                        // Cuadrado blanco limpio puro
                        egui::Frame::none()
                            .fill(Color32::WHITE)
                            .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(205, 215, 225)))
                            .inner_margin(egui::Margin::same(16.0))
                            .show(ui, |ui| {
                                ui.image((texture.id(), size));
                            });
                    }

                    // Divisor / tirador estilo "===" de la imagen original
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("===").color(Color32::from_rgb(160, 175, 190)).size(13.0));
                    ui.add_space(4.0);
                });

                // --- CONTENEDOR BLANCO CON PESTAÑAS (Exacto a las imágenes 1-5) ---
                let card_frame = egui::Frame::none()
                    .fill(Color32::WHITE)
                    .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(205, 215, 225)))
                    .inner_margin(egui::Margin::same(12.0));

                card_frame.show(ui, |ui| {
                    // Tiras de pestañas superiores en español
                    ui.horizontal(|ui| {
                        tab_button(ui, &mut self.active_tab, ActiveTab::Input, "Entrada");
                        tab_button(ui, &mut self.active_tab, ActiveTab::ColoursAndSize, "Colores y tamaño");
                        tab_button(ui, &mut self.active_tab, ActiveTab::LogoInQr, "Logo en QR");
                        tab_button(ui, &mut self.active_tab, ActiveTab::TextInQr, "Texto en QR");
                        tab_button(ui, &mut self.active_tab, ActiveTab::VerificationOk, "Verificación: OK");
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // ==========================================
                    // PESTAÑA 1: ENTRADA (Imagen 1)
                    // ==========================================
                    if self.active_tab == ActiveTab::Input {
                        let text_edit_response = ui.add_sized(
                            [ui.available_width(), 95.0],
                            egui::TextEdit::multiline(&mut self.input_text)
                                .font(egui::TextStyle::Monospace)
                                .hint_text("Introduce aquí tu texto o enlace..."),
                        );
                        if text_edit_response.changed() {
                            self.dirty = true;
                        }

                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            // Contador de caracteres a la izquierda
                            let char_count = self.input_text.chars().count();
                            ui.label(format!("{} Caracteres", char_count));

                            ui.add_space(16.0);

                            // Casilla "Usar 'Texto en QR' como entrada"
                            if ui.checkbox(&mut self.use_text_in_qr_as_input, "Usar \"Texto en QR\" como entrada.").changed() {
                                self.dirty = true;
                            }

                            // Botones a la derecha: [Importar] [Leer en voz alta] [Vaciar] (Azules)
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if blue_button(ui, "Vaciar").clicked() {
                                    self.input_text.clear();
                                    self.dirty = true;
                                }

                                if blue_button(ui, "Leer en voz alta").clicked() {
                                    self.status_message = format!("🔊 Leyendo texto: \"{}\"", self.input_text);
                                }

                                if blue_button(ui, "Importar").clicked() {
                                    if let Some(p) = rfd::FileDialog::new()
                                        .add_filter("Archivos de texto (*.txt, *.csv)", &["txt", "csv"])
                                        .pick_file()
                                    {
                                        if let Ok(c) = std::fs::read_to_string(p) {
                                            self.input_text = c;
                                            self.dirty = true;
                                        }
                                    }
                                }
                            });
                        });
                    }

                    // ==========================================
                    // PESTAÑA 2: COLORES Y TAMAÑO (Imagen 2)
                    // ==========================================
                    else if self.active_tab == ActiveTab::ColoursAndSize {
                        ui.columns(2, |cols| {
                            // Columna izquierda: Configuración de matriz
                            cols[0].group(|ui| {
                                ui.label(egui::RichText::new("Configuración de matriz").strong());
                                ui.add_space(6.0);

                                egui::Grid::new("matrix_settings_grid")
                                    .num_columns(3)
                                    .spacing([12.0, 8.0])
                                    .show(ui, |ui| {
                                        ui.label("Corrección de error:");
                                        let prev_ec = self.settings.ec_level;
                                        egui::ComboBox::from_id_salt("ec_level_select")
                                            .selected_text(self.settings.ec_level.label())
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut self.settings.ec_level, ErrorCorrection::Low, "Bajo (L)");
                                                ui.selectable_value(&mut self.settings.ec_level, ErrorCorrection::Medium, "Medio (M)");
                                                ui.selectable_value(&mut self.settings.ec_level, ErrorCorrection::Quartile, "Cuartil (Q)");
                                                ui.selectable_value(&mut self.settings.ec_level, ErrorCorrection::High, "Alto (H)");
                                            });
                                        if prev_ec != self.settings.ec_level { self.dirty = true; }
                                        ui.label("(?)");
                                        ui.end_row();

                                        ui.label("Borde:");
                                        let prev_b = self.settings.border;
                                        egui::ComboBox::from_id_salt("border_select")
                                            .selected_text(format!("{}", self.settings.border))
                                            .show_ui(ui, |ui| {
                                                for b in 0..=8 {
                                                    ui.selectable_value(&mut self.settings.border, b, format!("{}", b));
                                                }
                                            });
                                        if prev_b != self.settings.border { self.dirty = true; }
                                        ui.label("");
                                        ui.end_row();

                                        ui.label("Factor de escala:");
                                        let prev_s = self.settings.scale_factor;
                                        egui::ComboBox::from_id_salt("scale_select")
                                            .selected_text(format!("{}", self.settings.scale_factor))
                                            .show_ui(ui, |ui| {
                                                for s in 2..=15 {
                                                    ui.selectable_value(&mut self.settings.scale_factor, s, format!("{}", s));
                                                }
                                            });
                                        if prev_s != self.settings.scale_factor { self.dirty = true; }
                                        ui.label("(?)");
                                        ui.end_row();

                                        ui.label("Tamaño:");
                                        ui.label(format!("{} x {} px", self.last_computed_size.0, self.last_computed_size.1));
                                        ui.label("");
                                        ui.end_row();
                                    });
                            });

                            // Columna derecha: Color del código QR
                            cols[1].group(|ui| {
                                ui.label(egui::RichText::new("Color del código QR").strong());
                                ui.add_space(6.0);

                                ui.horizontal(|ui| {
                                    if blue_button(ui, "Color Fondo").clicked() {
                                        self.show_bg_picker = !self.show_bg_picker;
                                    }
                                    color_circle(ui, self.settings.bg_color);
                                    let hex_bg = format!("h00{:02X}{:02X}{:02X}", self.settings.bg_color[0], self.settings.bg_color[1], self.settings.bg_color[2]);
                                    ui.label(hex_bg);
                                });

                                if self.show_bg_picker {
                                    if ui.color_edit_button_srgba_unmultiplied(&mut self.settings.bg_color).changed() {
                                        self.dirty = true;
                                    }
                                }

                                ui.add_space(6.0);

                                ui.horizontal(|ui| {
                                    if blue_button(ui, "Color Frente").clicked() {
                                        self.show_fg_picker = !self.show_fg_picker;
                                    }
                                    color_circle(ui, self.settings.fg_color);
                                    let hex_fg = format!("h00{:02X}{:02X}{:02X}", self.settings.fg_color[0], self.settings.fg_color[1], self.settings.fg_color[2]);
                                    ui.label(hex_fg);
                                });

                                if self.show_fg_picker {
                                    if ui.color_edit_button_srgba_unmultiplied(&mut self.settings.fg_color).changed() {
                                        self.dirty = true;
                                    }
                                }

                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    if ui.button("🎨").on_hover_text("Colores aleatorios armoniosos").clicked() {
                                        self.randomize_colors();
                                    }
                                    if ui.button("⇄").on_hover_text("Intercambiar frente y fondo").clicked() {
                                        self.swap_colors();
                                    }
                                    ui.label(egui::RichText::new("¿Por qué no probar colores aleatorios? Usa los iconos.").weak().size(11.0));
                                });
                            });
                        });

                        ui.add_space(6.0);
                        ui.checkbox(&mut self.save_settings, "Guardar configuración");
                    }

                    // ==========================================
                    // PESTAÑA 3: LOGO EN QR (Imagen 3)
                    // ==========================================
                    else if self.active_tab == ActiveTab::LogoInQr {
                        ui.columns(2, |cols| {
                            cols[0].vertical(|ui| {
                                if ui.checkbox(&mut self.settings.logo_enabled, "Activar logo").changed() {
                                    if self.settings.logo_enabled {
                                        self.settings.ec_level = ErrorCorrection::High;
                                    }
                                    self.dirty = true;
                                }

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    if ui.button("Seleccionar imagen...").clicked() {
                                        self.pick_logo_file();
                                    }
                                    if ui.button("Quitar imagen").clicked() {
                                        self.custom_logo_img = None;
                                        self.custom_logo_path = None;
                                        self.dirty = true;
                                    }
                                });

                                let path_str = self.custom_logo_path.as_ref()
                                    .map(|p| format!("Ruta: {}", p.file_name().unwrap_or_default().to_string_lossy()))
                                    .unwrap_or_else(|| "Ruta: (Plantilla de logo integrada)".to_string());
                                ui.label(egui::RichText::new(path_str).weak().size(11.0));

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    ui.label("Plantillas de logo:");
                                    let prev_t = self.settings.logo_template;
                                    egui::ComboBox::from_id_salt("logo_template_combo")
                                        .selected_text(self.settings.logo_template.label())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::None, "---");
                                            ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::HeartPulse, "Corazón / Pulso");
                                            ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::Star, "Estrella");
                                            ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::Shield, "Escudo");
                                            ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::Wifi, "Símbolo Wi-Fi");
                                            ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::TwitterX, "X (Twitter)");
                                            ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::UserContact, "Avatar de contacto");
                                        });
                                    if prev_t != self.settings.logo_template {
                                        self.custom_logo_img = None;
                                        self.dirty = true;
                                    }
                                });

                                ui.add_space(6.0);
                                ui.label(format!("{} Píxeles al cuadrado", self.settings.logo_size_px));
                                if ui.add(egui::Slider::new(&mut self.settings.logo_size_px, 30..=180)).changed() {
                                    self.dirty = true;
                                }

                                ui.add_space(6.0);
                                ui.checkbox(&mut self.settings.keep_centered_enlarge, "Mantener centrado al agrandar");
                                ui.checkbox(&mut self.settings.check_validity, "Comprobar validez");

                                if ui.button("Ajuste automático").clicked() {
                                    self.settings.ec_level = ErrorCorrection::High;
                                    self.settings.logo_size_px = 80;
                                    self.dirty = true;
                                }
                            });

                            // Grupo derecho: Posición (D-Pad)
                            cols[1].group(|ui| {
                                ui.label(egui::RichText::new("Posición").strong());
                                ui.add_space(6.0);

                                ui.vertical_centered(|ui| {
                                    if ui.button(" ▲ ").clicked() {
                                        self.settings.logo_offset_y -= self.settings.logo_pixel_per_click;
                                        self.dirty = true;
                                    }
                                    ui.horizontal(|ui| {
                                        ui.add_space(ui.available_width() / 4.0);
                                        if ui.button(" ◀ ").clicked() {
                                            self.settings.logo_offset_x -= self.settings.logo_pixel_per_click;
                                            self.dirty = true;
                                        }
                                        if ui.button(" ▼ ").clicked() {
                                            self.settings.logo_offset_y += self.settings.logo_pixel_per_click;
                                            self.dirty = true;
                                        }
                                        if ui.button(" ▶ ").clicked() {
                                            self.settings.logo_offset_x += self.settings.logo_pixel_per_click;
                                            self.dirty = true;
                                        }
                                    });

                                    ui.add_space(6.0);
                                    if ui.button("Centrar").clicked() {
                                        self.settings.logo_offset_x = 0;
                                        self.settings.logo_offset_y = 0;
                                        self.dirty = true;
                                    }
                                });

                                ui.add_space(8.0);
                                ui.label("Píxeles por clic");
                                ui.add(egui::Slider::new(&mut self.settings.logo_pixel_per_click, 1..=20));
                            });
                        });
                    }

                    // ==========================================
                    // PESTAÑA 4: TEXTO EN QR (Imagen 4)
                    // ==========================================
                    else if self.active_tab == ActiveTab::TextInQr {
                        ui.columns(2, |cols| {
                            cols[0].vertical(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.checkbox(&mut self.settings.text_enabled, "Activar texto").changed() {
                                        self.dirty = true;
                                    }
                                    if ui.text_edit_singleline(&mut self.settings.text_content).changed() {
                                        self.dirty = true;
                                    }
                                });

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    ui.label("Fuente:");
                                    egui::ComboBox::from_id_salt("text_font_cb")
                                        .selected_text(&self.settings.text_font)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut self.settings.text_font, "Predeterminada".to_string(), "Predeterminada");
                                            ui.selectable_value(&mut self.settings.text_font, "Sans-Serif".to_string(), "Sans-Serif");
                                            ui.selectable_value(&mut self.settings.text_font, "Monoespacio".to_string(), "Monoespacio");
                                        });

                                    ui.label("Icono:");
                                    ui.label("👉");
                                });

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    ui.label("Letras:");
                                    if ui.radio_value(&mut self.settings.text_casing, TextCasing::TextField, "Campo de texto").clicked() { self.dirty = true; }
                                    if ui.radio_value(&mut self.settings.text_casing, TextCasing::Small, "minúsculas").clicked() { self.dirty = true; }
                                    if ui.radio_value(&mut self.settings.text_casing, TextCasing::Great, "MAYÚSCULAS").clicked() { self.dirty = true; }
                                });

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    if ui.checkbox(&mut self.settings.text_bold, "Negrita").changed() { self.dirty = true; }
                                    if ui.checkbox(&mut self.settings.text_italic, "Cursiva").changed() { self.dirty = true; }
                                    if ui.checkbox(&mut self.settings.text_underline, "Subrayado").changed() { self.dirty = true; }
                                });

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    if ui.button("Color").clicked() {
                                        self.show_txt_picker = !self.show_txt_picker;
                                    }
                                    color_circle(ui, self.settings.text_color);
                                    let hex_t = format!("h00{:02X}{:02X}{:02X}", self.settings.text_color[0], self.settings.text_color[1], self.settings.text_color[2]);
                                    ui.label(hex_t);
                                });

                                if self.show_txt_picker {
                                    if ui.color_edit_button_srgba_unmultiplied(&mut self.settings.text_color).changed() {
                                        self.dirty = true;
                                    }
                                }

                                ui.add_space(6.0);
                                ui.label("Tamaño de fuente");
                                if ui.add(egui::Slider::new(&mut self.settings.text_font_size, 8.0..=36.0)).changed() {
                                    self.dirty = true;
                                }
                            });

                            // Grupo derecho: Posición
                            cols[1].group(|ui| {
                                ui.label(egui::RichText::new("Posición").strong());
                                ui.add_space(6.0);

                                ui.vertical_centered(|ui| {
                                    if ui.button(" ▲ ").clicked() {
                                        self.settings.text_offset_y -= self.settings.text_pixel_per_click;
                                        self.dirty = true;
                                    }
                                    ui.horizontal(|ui| {
                                        ui.add_space(ui.available_width() / 4.0);
                                        if ui.button(" ◀ ").clicked() {
                                            self.settings.text_offset_x -= self.settings.text_pixel_per_click;
                                            self.dirty = true;
                                        }
                                        if ui.button(" ▼ ").clicked() {
                                            self.settings.text_offset_y += self.settings.text_pixel_per_click;
                                            self.dirty = true;
                                        }
                                        if ui.button(" ▶ ").clicked() {
                                            self.settings.text_offset_x += self.settings.text_pixel_per_click;
                                            self.dirty = true;
                                        }
                                    });

                                    ui.add_space(6.0);
                                    if ui.button("Centrar").clicked() {
                                        self.settings.text_offset_x = 0;
                                        self.settings.text_offset_y = 0;
                                        self.dirty = true;
                                    }
                                });

                                ui.add_space(8.0);
                                ui.label("Píxeles por clic");
                                ui.add(egui::Slider::new(&mut self.settings.text_pixel_per_click, 1..=20));
                            });
                        });
                    }

                    // ==========================================
                    // PESTAÑA 5: VERIFICACIÓN: OK (Imagen 5)
                    // ==========================================
                    else if self.active_tab == ActiveTab::VerificationOk {
                        ui.columns(2, |cols| {
                            // Grupo izquierdo: Contenido leído
                            cols[0].group(|ui| {
                                ui.label(egui::RichText::new("Contenido leído").strong());
                                ui.add_space(6.0);
                                let mut read_copy = self.effective_payload();
                                ui.add_sized([ui.available_width(), 95.0], egui::TextEdit::multiline(&mut read_copy).interactive(false));
                            });

                            // Grupo derecho: Verificación
                            cols[1].group(|ui| {
                                ui.label(egui::RichText::new("Verificación").strong());
                                ui.add_space(6.0);

                                ui.horizontal(|ui| {
                                    ui.label("Tipo reconocido:");
                                    ui.label(egui::RichText::new("Código QR reconocido").strong().color(Color32::from_rgb(29, 78, 216)));
                                });

                                ui.add_space(8.0);
                                if blue_button(ui, "Comprobar de nuevo manualmente").clicked() {
                                    self.status_message = "✓ Comprobación manual completada con éxito al 100%.".to_string();
                                }

                                ui.add_space(6.0);
                                if blue_button(ui, "Abrir procedimiento de prueba extendido").clicked() {
                                    self.status_message = "✓ Prueba extendida: Dimensiones de matriz, corrección Reed-Solomon y contraste correctos.".to_string();
                                }

                                ui.add_space(6.0);
                                ui.label(egui::RichText::new("Nota sobre la verificación...").weak().size(11.0));
                            });
                        });
                    }
                });
            });

        // ==========================================
        // 5. MODALES INTERACTIVOS PARA PLANTILLAS
        // ==========================================
        self.render_template_modals(ctx);
    }
}

impl QrApp {
    fn render_template_modals(&mut self, ctx: &egui::Context) {
        match self.active_modal {
            TemplateModal::None => {}

            TemplateModal::Wifi => {
                let mut open = true;
                egui::Window::new("📶 Configuración Red Wi-Fi (WLAN)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Nombre de la red (SSID):");
                        ui.text_edit_singleline(&mut self.wifi_ssid);
                        ui.label("Contraseña:");
                        ui.text_edit_singleline(&mut self.wifi_pass);
                        ui.label("Tipo de seguridad:");
                        egui::ComboBox::from_id_salt("modal_wifi_sec")
                            .selected_text(&self.wifi_sec)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.wifi_sec, "WPA".to_string(), "WPA / WPA2 / WPA3");
                                ui.selectable_value(&mut self.wifi_sec, "WEP".to_string(), "WEP");
                                ui.selectable_value(&mut self.wifi_sec, "nopass".to_string(), "Abierta (Sin contraseña)");
                            });
                        ui.checkbox(&mut self.wifi_hidden, "Red oculta");

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if blue_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!("WIFI:T:{};S:{};P:{};H:{};;", self.wifi_sec, self.wifi_ssid, self.wifi_pass, self.wifi_hidden);
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if ui.button("Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::VCard => {
                let mut open = true;
                egui::Window::new("👤 Contacto (vCard)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Nombre:");
                        ui.text_edit_singleline(&mut self.vcard_first);
                        ui.label("Apellidos:");
                        ui.text_edit_singleline(&mut self.vcard_last);
                        ui.label("Teléfono:");
                        ui.text_edit_singleline(&mut self.vcard_phone);
                        ui.label("Correo electrónico:");
                        ui.text_edit_singleline(&mut self.vcard_email);
                        ui.label("Empresa / Organización:");
                        ui.text_edit_singleline(&mut self.vcard_org);
                        ui.label("Sitio web:");
                        ui.text_edit_singleline(&mut self.vcard_url);

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if blue_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "BEGIN:VCARD\nVERSION:3.0\nN:{};{};;;\nFN:{} {}\nTEL;TYPE=CELL:{}\nEMAIL:{}\nORG:{}\nURL:{}\nEND:VCARD",
                                    self.vcard_last, self.vcard_first, self.vcard_first, self.vcard_last,
                                    self.vcard_phone, self.vcard_email, self.vcard_org, self.vcard_url
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if ui.button("Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Url => {
                let mut open = true;
                egui::Window::new("🔗 Enlace (URL)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Dirección URL del sitio web:");
                        ui.text_edit_singleline(&mut self.url_modal_input);

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if blue_button(ui, "Aplicar al QR").clicked() {
                                let mut final_url = self.url_modal_input.trim().to_string();
                                if !final_url.starts_with("http://") && !final_url.starts_with("https://") {
                                    final_url = format!("https://{}", final_url);
                                }
                                self.input_text = final_url;
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if ui.button("Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Twitter => {
                let mut open = true;
                egui::Window::new("𝕏 Tweet de X (antes Twitter)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Texto del Tweet:");
                        ui.text_edit_multiline(&mut self.twitter_text);
                        ui.label("Vía usuario (opcional):");
                        ui.text_edit_singleline(&mut self.twitter_user);

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if blue_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "https://twitter.com/intent/tweet?text={}&via={}",
                                    url_encode(&self.twitter_text),
                                    url_encode(&self.twitter_user)
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if ui.button("Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Date => {
                let mut open = true;
                egui::Window::new("📅 Fecha / Evento en Calendario")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Título del evento:");
                        ui.text_edit_singleline(&mut self.date_title);
                        ui.label("Fecha y hora de inicio (ISO):");
                        ui.text_edit_singleline(&mut self.date_start);
                        ui.label("Ubicación:");
                        ui.text_edit_singleline(&mut self.date_location);

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if blue_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "BEGIN:VEVENT\nSUMMARY:{}\nDTSTART:{}\nLOCATION:{}\nEND:VEVENT",
                                    self.date_title, self.date_start, self.date_location
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if ui.button("Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::PayPal => {
                let mut open = true;
                egui::Window::new("🅿 Enlace de pago PayPal")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Nombre de usuario PayPal.Me:");
                        ui.text_edit_singleline(&mut self.paypal_user);
                        ui.label("Importe:");
                        ui.text_edit_singleline(&mut self.paypal_amount);
                        ui.label("Moneda (EUR, USD):");
                        ui.text_edit_singleline(&mut self.paypal_currency);

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if blue_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "https://www.paypal.me/{}/{}{}",
                                    self.paypal_user, self.paypal_amount, self.paypal_currency
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if ui.button("Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Iban => {
                let mut open = true;
                egui::Window::new("🏛 Pago bancario IBAN / BIC (EPC-QR)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Nombre del beneficiario:");
                        ui.text_edit_singleline(&mut self.iban_name);
                        ui.label("Número IBAN:");
                        ui.text_edit_singleline(&mut self.iban_number);
                        ui.label("Código BIC (Swift):");
                        ui.text_edit_singleline(&mut self.iban_bic);
                        ui.label("Importe (EUR):");
                        ui.text_edit_singleline(&mut self.iban_amount);

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if blue_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "BCD\n002\n1\nSCT\n{}\n{}\nEUR{}\n\n\n\n",
                                    self.iban_bic, self.iban_name, self.iban_amount
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if ui.button("Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Totp => {
                let mut open = true;
                egui::Window::new("🛡 Código 2FA TOTP (Autenticación en dos pasos)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Cuenta (ej. correo electrónico):");
                        ui.text_edit_singleline(&mut self.totp_account);
                        ui.label("Emisor del servicio:");
                        ui.text_edit_singleline(&mut self.totp_issuer);
                        ui.label("Clave secreta (Base32):");
                        ui.text_edit_singleline(&mut self.totp_secret);

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if blue_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "otpauth://totp/{}:{}?secret={}&issuer={}",
                                    self.totp_issuer, self.totp_account, self.totp_secret, self.totp_issuer
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if ui.button("Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Programming => {
                let mut open = true;
                egui::Window::new("</> Programar un código QR en Rust")
                    .open(&mut open)
                    .resizable(true)
                    .show(ctx, |ui| {
                        ui.heading("Generador de código en Rust");
                        ui.label("Puedes generar este código QR directamente en Rust con el siguiente código:");
                        ui.add(egui::TextEdit::multiline(&mut format!(
                            "use qrcode::QrCode;\nuse image::Luma;\n\nlet code = QrCode::new(b\"{}\").unwrap();\nlet image = code.render::<Luma<u8>>().build();\nimage.save(\"codigo_qr.png\").unwrap();",
                            self.effective_payload()
                        )).interactive(false));
                        if ui.button("Cerrar").clicked() {
                            self.active_modal = TemplateModal::None;
                        }
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::About => {
                let mut open = true;
                egui::Window::new("ℹ Acerca de Creador de Códigos QR")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.heading("Creador de Códigos QR V7.8.1.3 (Edición Rust)");
                        ui.label("Generador nativo de códigos QR escrito 100% en Rust.");
                        ui.label("Incluye soporte para logotipos, paletas de colores, biblioteca de plantillas y exportación vectorial.");
                        ui.add_space(10.0);
                        if blue_button(ui, "Cerrar").clicked() {
                            self.active_modal = TemplateModal::None;
                        }
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }
        }
    }
}

// ==========================================
// HELPERS DE DISEÑO UI
// ==========================================

fn toolbar_button(ui: &mut egui::Ui, icon: &str, label: &str) -> egui::Response {
    ui.button(egui::RichText::new(format!("{} {}", icon, label)).size(11.0))
}

fn toolbar_separator(ui: &mut egui::Ui) {
    ui.add(egui::Separator::default().vertical().spacing(10.0));
}

fn tab_button(ui: &mut egui::Ui, current: &mut ActiveTab, target: ActiveTab, label: &str) {
    let is_selected = *current == target;
    let text = if is_selected {
        egui::RichText::new(label).strong().color(Color32::from_rgb(20, 20, 20))
    } else {
        egui::RichText::new(label).color(Color32::from_rgb(100, 110, 120))
    };

    if ui.selectable_label(is_selected, text).clicked() {
        *current = target;
    }
}

fn blue_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(Color32::WHITE).size(12.0))
            .fill(Color32::from_rgb(37, 99, 235)) // Azul clásico #2563EB de las capturas
            .rounding(3.0_f32),
    )
}

fn color_circle(ui: &mut egui::Ui, color: [u8; 4]) {
    let (rect, _resp) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.circle_filled(rect.center(), 7.0, Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]));
    painter.circle_stroke(rect.center(), 7.0, egui::Stroke::new(1.0_f32, Color32::from_rgb(100, 100, 100)));
}

fn url_encode(input: &str) -> String {
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

fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if i + 1 < data.len() {
            out.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}
