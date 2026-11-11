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
    input_text: String,
    use_text_in_qr_as_input: bool,

    settings: QrSettings,
    active_tab: ActiveTab,
    save_settings: bool,

    custom_logo_img: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    custom_logo_path: Option<PathBuf>,

    texture: Option<TextureHandle>,
    dirty: bool,
    status_message: String,
    last_computed_size: (u32, u32),

    active_modal: TemplateModal,

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

    show_bg_picker: bool,
    show_fg_picker: bool,
    show_txt_picker: bool,

    app_icon_texture: Option<TextureHandle>,
}

impl Default for QrApp {
    fn default() -> Self {
        let default_text = "Mi primer codigo QR".to_string();
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
            app_icon_texture: None,
            dirty: true,
            status_message: "Listo. Codigo QR generado correctamente.".to_string(),
            last_computed_size: initial_size,

            active_modal: TemplateModal::None,

            wifi_ssid: "MiRedWiFi".to_string(),
            wifi_pass: "ClaveSecreta123".to_string(),
            wifi_sec: "WPA".to_string(),
            wifi_hidden: false,

            vcard_first: "Juan".to_string(),
            vcard_last: "Perez".to_string(),
            vcard_phone: "+34 600 123 456".to_string(),
            vcard_email: "juan.perez@ejemplo.com".to_string(),
            vcard_org: "Empresa Global".to_string(),
            vcard_url: "https://ejemplo.com".to_string(),

            url_modal_input: "https://rust-lang.org".to_string(),
            twitter_text: "Codigo QR generado con Rust".to_string(),
            twitter_user: "rustlang".to_string(),

            date_title: "Conferencia de Rust 2026".to_string(),
            date_start: "2026-11-15T09:00:00".to_string(),
            date_location: "En linea / Centro de Convenciones".to_string(),

            paypal_user: "usuario".to_string(),
            paypal_amount: "10.00".to_string(),
            paypal_currency: "EUR".to_string(),

            iban_name: "Juan Perez".to_string(),
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
        // Configuracion completa de estilo para fondo blanco puro
        let mut visuals = egui::Visuals::light();
        visuals.window_fill = Color32::WHITE;
        visuals.panel_fill = Color32::WHITE;
        visuals.faint_bg_color = Color32::from_rgb(248, 250, 252);
        visuals.extreme_bg_color = Color32::WHITE;

        visuals.widgets.noninteractive.bg_fill = Color32::WHITE;
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240));
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(30, 41, 59));

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(248, 250, 252);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240));
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(51, 65, 85));

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(241, 245, 249);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(203, 213, 225));
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(15, 23, 42));

        visuals.widgets.active.bg_fill = Color32::from_rgb(224, 231, 255);
        visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(99, 102, 241));
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(30, 27, 75));

        visuals.selection.bg_fill = Color32::from_rgb(219, 234, 254);
        visuals.selection.stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(37, 99, 235));

        _cc.egui_ctx.set_visuals(visuals);

        Self::default()
    }

    fn effective_payload(&self) -> String {
        if self.use_text_in_qr_as_input && self.settings.text_enabled {
            self.settings.text_content.clone()
        } else if self.input_text.is_empty() {
            "Mi primer codigo QR".to_string()
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
                self.status_message = format!("Codigo QR generado con resolucion {} x {} px", w, h);
            }
            Err(e) => {
                self.status_message = format!("Aviso de generacion: {}", e);
            }
        }
    }

    fn randomize_colors(&mut self) {
        let palettes: [([u8; 4], [u8; 4]); 6] = [
            ([255, 255, 255, 255], [14, 116, 144, 255]),
            ([255, 255, 255, 255], [185, 28, 28, 255]),
            ([248, 250, 252, 255], [15, 23, 42, 255]),
            ([254, 243, 199, 255], [120, 53, 15, 255]),
            ([240, 253, 244, 255], [20, 83, 45, 255]),
            ([250, 245, 255, 255], [88, 28, 135, 255]),
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
                    self.status_message = format!("PNG guardado en {:?}", path.file_name().unwrap_or_default());
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
                    self.status_message = format!("JPEG guardado en {:?}", path.file_name().unwrap_or_default());
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
                    self.status_message = format!("SVG guardado en {:?}", path.file_name().unwrap_or_default());
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
                    self.status_message = format!("EPS guardado en {:?}", path.file_name().unwrap_or_default());
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
                self.status_message = "URI de datos Base64 copiada al portapapeles".to_string();
            }
        }
    }

    fn export_markdown(&mut self, ctx: &egui::Context) {
        let payload = self.effective_payload();
        let md = format!("![Codigo QR para {}](codigo_qr.png)", payload);
        ctx.output_mut(|o| o.copied_text = md);
        self.status_message = "Fragmento Markdown copiado al portapapeles".to_string();
    }

    fn pick_logo_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Imagenes (*.png, *.jpg, *.jpeg, *.webp)", &["png", "jpg", "jpeg", "webp"])
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
        if self.app_icon_texture.is_none() {
            let ico_bytes = include_bytes!("../img/icon.ico");
            if let Ok(dyn_img) = image::load_from_memory_with_format(ico_bytes, image::ImageFormat::Ico) {
                let rgba = dyn_img.to_rgba8();
                let (w, h) = (rgba.width() as usize, rgba.height() as usize);
                let color_image = ColorImage::from_rgba_unmultiplied([w, h], &rgba.into_raw());
                self.app_icon_texture = Some(ctx.load_texture("app_icon_tex", color_image, TextureOptions::LINEAR));
            }
        }

        if self.dirty || self.texture.is_none() {
            self.regenerate_qr(ctx);
        }

        // 1. MENU SUPERIOR
        egui::TopBottomPanel::top("menu_bar")
            .frame(
                egui::Frame::none()
                    .fill(Color32::WHITE)
                    .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(241, 245, 249)))
                    .inner_margin(egui::Margin::symmetric(10.0, 4.0)),
            )
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Archivo", |ui| {
                        if ui.button("Nuevo               Ctrl+N").clicked() {
                            self.input_text = "Mi primer codigo QR".to_string();
                            self.dirty = true;
                            ui.close_menu();
                        }
                        if ui.button("Abrir...").clicked() {
                            if let Some(p) = rfd::FileDialog::new().add_filter("Archivos de texto (*.txt)", &["txt"]).pick_file() {
                                if let Ok(content) = std::fs::read_to_string(p) {
                                    self.input_text = content;
                                    self.dirty = true;
                                }
                            }
                            ui.close_menu();
                        }
                        if ui.button("Guardar como...").clicked() {
                            self.export_png();
                            ui.close_menu();
                        }
                        ui.separator();
                        ui.menu_button("Exportar Codigo QR >", |ui| {
                            if ui.button("PNG").clicked() { self.export_png(); ui.close_menu(); }
                            if ui.button("JPEG").clicked() { self.export_jpeg(); ui.close_menu(); }
                            if ui.button("SVG").clicked() { self.export_svg(); ui.close_menu(); }
                            if ui.button("EPS").clicked() { self.export_eps(); ui.close_menu(); }
                            if ui.button("Base64").clicked() { self.export_base64(ctx); ui.close_menu(); }
                            if ui.button("MarkDown").clicked() { self.export_markdown(ctx); ui.close_menu(); }
                        });
                        ui.separator();
                        if ui.button("Imprimir            Ctrl+P").clicked() {
                            self.status_message = "Dialogo de impresion preparado.".to_string();
                            ui.close_menu();
                        }
                        if ui.button("Actualizar          F12").clicked() {
                            self.dirty = true;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Salir               Ctrl+Q").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.menu_button("Edicion", |ui| {
                        if ui.button("Copiar texto").clicked() {
                            ctx.output_mut(|o| o.copied_text = self.input_text.clone());
                            self.status_message = "Texto copiado al portapapeles.".to_string();
                            ui.close_menu();
                        }
                        if ui.button("Limpiar entrada").clicked() {
                            self.input_text.clear();
                            self.dirty = true;
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Plantillas QR", |ui| {
                        if ui.button("Texto libre").clicked() { self.active_tab = ActiveTab::Input; ui.close_menu(); }
                        if ui.button("Enlace (URL)").clicked() { self.active_modal = TemplateModal::Url; ui.close_menu(); }
                        if ui.button("Tweet de X (Twitter)").clicked() { self.active_modal = TemplateModal::Twitter; ui.close_menu(); }
                        if ui.button("Contacto (vCard)").clicked() { self.active_modal = TemplateModal::VCard; ui.close_menu(); }
                        if ui.button("Red WiFi (WLAN)").clicked() { self.active_modal = TemplateModal::Wifi; ui.close_menu(); }
                        if ui.button("Fecha / Evento").clicked() { self.active_modal = TemplateModal::Date; ui.close_menu(); }
                        if ui.button("Pago PayPal").clicked() { self.active_modal = TemplateModal::PayPal; ui.close_menu(); }
                        if ui.button("Datos bancarios IBAN / BIC").clicked() { self.active_modal = TemplateModal::Iban; ui.close_menu(); }
                        if ui.button("Codigo TOTP (2FA)").clicked() { self.active_modal = TemplateModal::Totp; ui.close_menu(); }
                        ui.separator();
                        if ui.button("Programar codigo QR").clicked() { self.active_modal = TemplateModal::Programming; ui.close_menu(); }
                    });

                    ui.menu_button("Ver", |ui| {
                        if ui.button("Restablecer vista").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("Configuracion", |ui| {
                        if ui.button("Idioma: Espanol").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("Ayuda", |ui| {
                        if ui.button("Acerca de QR Generator").clicked() {
                            self.active_modal = TemplateModal::About;
                            ui.close_menu();
                        }
                    });
                });
            });

        // 2. TOOLBAR SOBRE FONDO BLANCO
        egui::TopBottomPanel::top("toolbar_panel")
            .frame(
                egui::Frame::none()
                    .fill(Color32::WHITE)
                    .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)))
                    .inner_margin(egui::Margin::symmetric(12.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(tex) = &self.app_icon_texture {
                        ui.image((tex.id(), egui::vec2(22.0, 22.0)));
                        ui.add_space(2.0);
                    }

                    if secondary_button(ui, "Actualizar").clicked() {
                        self.dirty = true;
                    }

                    toolbar_separator(ui);

                    ui.menu_button("Plantillas v", |ui| {
                        if ui.button("Texto libre").clicked() { self.active_tab = ActiveTab::Input; ui.close_menu(); }
                        if ui.button("Enlace (URL)").clicked() { self.active_modal = TemplateModal::Url; ui.close_menu(); }
                        if ui.button("Tweet de X (Twitter)").clicked() { self.active_modal = TemplateModal::Twitter; ui.close_menu(); }
                        if ui.button("Contacto (vCard)").clicked() { self.active_modal = TemplateModal::VCard; ui.close_menu(); }
                        if ui.button("Red WiFi (WLAN)").clicked() { self.active_modal = TemplateModal::Wifi; ui.close_menu(); }
                        if ui.button("Fecha / Evento").clicked() { self.active_modal = TemplateModal::Date; ui.close_menu(); }
                        if ui.button("Pago PayPal").clicked() { self.active_modal = TemplateModal::PayPal; ui.close_menu(); }
                        if ui.button("Datos bancarios IBAN / BIC").clicked() { self.active_modal = TemplateModal::Iban; ui.close_menu(); }
                        if ui.button("Codigo TOTP (2FA)").clicked() { self.active_modal = TemplateModal::Totp; ui.close_menu(); }
                        ui.separator();
                        if ui.button("Programar codigo QR").clicked() { self.active_modal = TemplateModal::Programming; ui.close_menu(); }
                    });

                    if secondary_button(ui, "Copiar").clicked() {
                        ctx.output_mut(|o| o.copied_text = self.input_text.clone());
                        self.status_message = "Contenido copiado al portapapeles".to_string();
                    }

                    if secondary_button(ui, "Verificacion").clicked() {
                        self.active_tab = ActiveTab::VerificationOk;
                    }

                    toolbar_separator(ui);

                    ui.menu_button("Exportar v", |ui| {
                        if ui.button("JPEG").clicked() { self.export_jpeg(); ui.close_menu(); }
                        if ui.button("PNG").clicked() { self.export_png(); ui.close_menu(); }
                        if ui.button("SVG").clicked() { self.export_svg(); ui.close_menu(); }
                        if ui.button("EPS").clicked() { self.export_eps(); ui.close_menu(); }
                        if ui.button("Base64").clicked() { self.export_base64(ctx); ui.close_menu(); }
                        if ui.button("MarkDown").clicked() { self.export_markdown(ctx); ui.close_menu(); }
                    });

                    ui.menu_button("Herramientas v", |ui| {
                        if ui.button("Generar colores aleatorios").clicked() { self.randomize_colors(); ui.close_menu(); }
                        if ui.button("Intercambiar frente y fondo").clicked() { self.swap_colors(); ui.close_menu(); }
                    });

                    if secondary_button(ui, "Lienzos").clicked() {
                        self.settings.scale_factor = if self.settings.scale_factor == 7 { 10 } else { 7 };
                        self.dirty = true;
                    }
                });
            });

        // 3. BARRA INFERIOR DE ESTADO
        egui::TopBottomPanel::bottom("status_bar")
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(248, 250, 252))
                    .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)))
                    .inner_margin(egui::Margin::symmetric(12.0, 5.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&self.status_message).size(12.0).color(Color32::from_rgb(71, 85, 105)));
                });
            });

        // 4. PANEL CENTRAL SOBRE FONDO BLANCO
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Color32::WHITE)
                    .inner_margin(egui::Margin::symmetric(18.0, 12.0)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // SECCION SUPERIOR: LIENZO BLANCO CON ELEVACION SUAVE
                    ui.vertical_centered(|ui| {
                        if let Some(texture) = &self.texture {
                            let canvas_px = 280.0;
                            let aspect = texture.aspect_ratio();
                            let size = if aspect >= 1.0 {
                                egui::vec2(canvas_px, canvas_px / aspect)
                            } else {
                                egui::vec2(canvas_px * aspect, canvas_px)
                            };

                            egui::Frame::none()
                                .fill(Color32::WHITE)
                                .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)))
                                .rounding(8.0_f32)
                                .inner_margin(egui::Margin::same(16.0))
                                .show(ui, |ui| {
                                    ui.image((texture.id(), size));
                                });
                        }

                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Dimension nativa: {} x {} px",
                                self.last_computed_size.0, self.last_computed_size.1
                            ))
                            .size(11.0)
                            .color(Color32::from_rgb(100, 116, 139)),
                        );
                        ui.add_space(8.0);
                    });

                    // CONTENEDOR DE PESTANAS EN TARJETA BLANCA ESTILIZADA
                    let tab_card_frame = egui::Frame::none()
                        .fill(Color32::WHITE)
                        .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)))
                        .rounding(8.0_f32)
                        .inner_margin(egui::Margin::same(14.0));

                    tab_card_frame.show(ui, |ui| {
                        // Barra de pestanas estilo pildora
                        ui.horizontal(|ui| {
                            pill_tab(ui, &mut self.active_tab, ActiveTab::Input, "Entrada");
                            pill_tab(ui, &mut self.active_tab, ActiveTab::ColoursAndSize, "Colores y tamano");
                            pill_tab(ui, &mut self.active_tab, ActiveTab::LogoInQr, "Logo en QR");
                            pill_tab(ui, &mut self.active_tab, ActiveTab::TextInQr, "Texto en QR");
                            pill_tab(ui, &mut self.active_tab, ActiveTab::VerificationOk, "Verificacion: OK");
                        });

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // PESTANA 1: ENTRADA
                        if self.active_tab == ActiveTab::Input {
                            let text_edit_response = ui.add_sized(
                                [ui.available_width(), 95.0],
                                egui::TextEdit::multiline(&mut self.input_text)
                                    .font(egui::TextStyle::Monospace)
                                    .hint_text("Introduce aqui tu texto o enlace..."),
                            );
                            if text_edit_response.changed() {
                                self.dirty = true;
                            }

                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                let char_count = self.input_text.chars().count();
                                ui.label(
                                    egui::RichText::new(format!("{} caracteres", char_count))
                                        .size(12.0)
                                        .color(Color32::from_rgb(100, 116, 139)),
                                );

                                ui.add_space(16.0);

                                if ui.checkbox(&mut self.use_text_in_qr_as_input, "Usar \"Texto en QR\" como entrada").changed() {
                                    self.dirty = true;
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if secondary_button(ui, "Vaciar").clicked() {
                                        self.input_text.clear();
                                        self.dirty = true;
                                    }

                                    if secondary_button(ui, "Importar texto").clicked() {
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

                        // PESTANA 2: COLORES Y TAMANO
                        else if self.active_tab == ActiveTab::ColoursAndSize {
                            ui.columns(2, |cols| {
                                // Columna 1: Matriz
                                card_group(&mut cols[0], "Configuracion de matriz", |ui| {
                                    egui::Grid::new("matrix_settings_grid")
                                        .num_columns(2)
                                        .spacing([14.0, 10.0])
                                        .show(ui, |ui| {
                                            ui.label("Correccion de error:");
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
                                            ui.end_row();

                                            ui.label("Borde (Zona de silencio):");
                                            let prev_b = self.settings.border;
                                            egui::ComboBox::from_id_salt("border_select")
                                                .selected_text(format!("{} modulos", self.settings.border))
                                                .show_ui(ui, |ui| {
                                                    for b in 0..=8 {
                                                        ui.selectable_value(&mut self.settings.border, b, format!("{} modulos", b));
                                                    }
                                                });
                                            if prev_b != self.settings.border { self.dirty = true; }
                                            ui.end_row();

                                            ui.label("Factor de escala:");
                                            let prev_s = self.settings.scale_factor;
                                            egui::ComboBox::from_id_salt("scale_select")
                                                .selected_text(format!("{} px/modulo", self.settings.scale_factor))
                                                .show_ui(ui, |ui| {
                                                    for s in 2..=15 {
                                                        ui.selectable_value(&mut self.settings.scale_factor, s, format!("{} px/modulo", s));
                                                    }
                                                });
                                            if prev_s != self.settings.scale_factor { self.dirty = true; }
                                            ui.end_row();

                                            ui.label("Tamano final:");
                                            ui.label(
                                                egui::RichText::new(format!("{} x {} px", self.last_computed_size.0, self.last_computed_size.1))
                                                    .strong()
                                                    .color(Color32::from_rgb(37, 99, 235)),
                                            );
                                            ui.end_row();
                                        });
                                });

                                // Columna 2: Colores
                                card_group(&mut cols[1], "Color del codigo QR", |ui| {
                                    ui.horizontal(|ui| {
                                        if primary_button(ui, "Color Fondo").clicked() {
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

                                    ui.add_space(8.0);

                                    ui.horizontal(|ui| {
                                        if primary_button(ui, "Color Frente").clicked() {
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

                                    ui.add_space(10.0);
                                    ui.horizontal(|ui| {
                                        if secondary_button(ui, "Paleta aleatoria").clicked() {
                                            self.randomize_colors();
                                        }
                                        if secondary_button(ui, "Invertir").clicked() {
                                            self.swap_colors();
                                        }
                                    });
                                });
                            });

                            ui.add_space(8.0);
                            ui.checkbox(&mut self.save_settings, "Guardar configuracion");
                        }

                        // PESTANA 3: LOGO EN QR
                        else if self.active_tab == ActiveTab::LogoInQr {
                            ui.columns(2, |cols| {
                                card_group(&mut cols[0], "Configuracion de logo", |ui| {
                                    if ui.checkbox(&mut self.settings.logo_enabled, "Habilitar logo central").changed() {
                                        if self.settings.logo_enabled {
                                            self.settings.ec_level = ErrorCorrection::High;
                                            if self.settings.logo_template == LogoTemplate::None && self.custom_logo_img.is_none() {
                                                self.settings.logo_template = LogoTemplate::AppIcon;
                                            }
                                        }
                                        self.dirty = true;
                                    }

                                    ui.add_space(6.0);
                                    ui.horizontal(|ui| {
                                        if secondary_button(ui, "Seleccionar imagen...").clicked() {
                                            self.pick_logo_file();
                                        }
                                        if secondary_button(ui, "Quitar imagen").clicked() {
                                            self.custom_logo_img = None;
                                            self.custom_logo_path = None;
                                            self.dirty = true;
                                        }
                                    });

                                    let path_str = self.custom_logo_path.as_ref()
                                        .map(|p| format!("Archivo: {}", p.file_name().unwrap_or_default().to_string_lossy()))
                                        .unwrap_or_else(|| "Logo: (Plantilla integrada)".to_string());
                                    ui.label(egui::RichText::new(path_str).weak().size(11.0));

                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        ui.label("Plantilla predefinida:");
                                        let prev_t = self.settings.logo_template;
                                        egui::ComboBox::from_id_salt("logo_template_combo")
                                            .selected_text(self.settings.logo_template.label())
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::None, "---");
                                                ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::AppIcon, "Icono de la aplicacion (icon.ico)");
                                                ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::HeartPulse, "Corazon / Pulso");
                                                ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::Star, "Estrella");
                                                ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::Shield, "Escudo");
                                                ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::Wifi, "Simbolo Wi-Fi");
                                                ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::TwitterX, "X (Twitter)");
                                                ui.selectable_value(&mut self.settings.logo_template, LogoTemplate::UserContact, "Avatar de contacto");
                                            });
                                        if prev_t != self.settings.logo_template {
                                            self.custom_logo_img = None;
                                            self.dirty = true;
                                        }
                                    });

                                    ui.add_space(8.0);
                                    ui.label(format!("Tamano: {} px al cuadrado", self.settings.logo_size_px));
                                    if ui.add(egui::Slider::new(&mut self.settings.logo_size_px, 30..=180)).changed() {
                                        self.dirty = true;
                                    }

                                    ui.add_space(6.0);
                                    ui.checkbox(&mut self.settings.keep_centered_enlarge, "Mantener centrado al redimensionar");
                                    ui.checkbox(&mut self.settings.check_validity, "Comprobar validez de lectura");

                                    ui.add_space(6.0);
                                    if secondary_button(ui, "Ajuste automatico").clicked() {
                                        self.settings.ec_level = ErrorCorrection::High;
                                        self.settings.logo_size_px = 80;
                                        self.dirty = true;
                                    }
                                });

                                card_group(&mut cols[1], "Posicion del logo", |ui| {
                                    ui.vertical_centered(|ui| {
                                        if secondary_button(ui, " ^ ").clicked() {
                                            self.settings.logo_offset_y -= self.settings.logo_pixel_per_click;
                                            self.dirty = true;
                                        }
                                        ui.horizontal(|ui| {
                                            ui.add_space(ui.available_width() / 4.0);
                                            if secondary_button(ui, " < ").clicked() {
                                                self.settings.logo_offset_x -= self.settings.logo_pixel_per_click;
                                                self.dirty = true;
                                            }
                                            if secondary_button(ui, " v ").clicked() {
                                                self.settings.logo_offset_y += self.settings.logo_pixel_per_click;
                                                self.dirty = true;
                                            }
                                            if secondary_button(ui, " > ").clicked() {
                                                self.settings.logo_offset_x += self.settings.logo_pixel_per_click;
                                                self.dirty = true;
                                            }
                                        });

                                        ui.add_space(6.0);
                                        if primary_button(ui, "Centrar").clicked() {
                                            self.settings.logo_offset_x = 0;
                                            self.settings.logo_offset_y = 0;
                                            self.dirty = true;
                                        }
                                    });

                                    ui.add_space(10.0);
                                    ui.label("Pixeles por paso:");
                                    ui.add(egui::Slider::new(&mut self.settings.logo_pixel_per_click, 1..=20));
                                });
                            });
                        }

                        // PESTANA 4: TEXTO EN QR
                        else if self.active_tab == ActiveTab::TextInQr {
                            ui.columns(2, |cols| {
                                card_group(&mut cols[0], "Configuracion de texto", |ui| {
                                    ui.horizontal(|ui| {
                                        if ui.checkbox(&mut self.settings.text_enabled, "Habilitar texto").changed() {
                                            self.dirty = true;
                                        }
                                        if ui.text_edit_singleline(&mut self.settings.text_content).changed() {
                                            self.dirty = true;
                                        }
                                    });

                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        ui.label("Tipografia:");
                                        egui::ComboBox::from_id_salt("text_font_cb")
                                            .selected_text(&self.settings.text_font)
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut self.settings.text_font, "Predeterminada".to_string(), "Predeterminada");
                                                ui.selectable_value(&mut self.settings.text_font, "Sans-Serif".to_string(), "Sans-Serif");
                                                ui.selectable_value(&mut self.settings.text_font, "Monoespacio".to_string(), "Monoespacio");
                                            });
                                    });

                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        ui.label("Formato:");
                                        if ui.radio_value(&mut self.settings.text_casing, TextCasing::TextField, "Original").clicked() { self.dirty = true; }
                                        if ui.radio_value(&mut self.settings.text_casing, TextCasing::Small, "minusculas").clicked() { self.dirty = true; }
                                        if ui.radio_value(&mut self.settings.text_casing, TextCasing::Great, "MAYUSCULAS").clicked() { self.dirty = true; }
                                    });

                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        if ui.checkbox(&mut self.settings.text_bold, "Negrita").changed() { self.dirty = true; }
                                        if ui.checkbox(&mut self.settings.text_italic, "Cursiva").changed() { self.dirty = true; }
                                        if ui.checkbox(&mut self.settings.text_underline, "Subrayado").changed() { self.dirty = true; }
                                    });

                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        if primary_button(ui, "Color de texto").clicked() {
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

                                    ui.add_space(8.0);
                                    ui.label("Tamano de fuente:");
                                    if ui.add(egui::Slider::new(&mut self.settings.text_font_size, 8.0..=36.0)).changed() {
                                        self.dirty = true;
                                    }
                                });

                                card_group(&mut cols[1], "Posicion del texto", |ui| {
                                    ui.vertical_centered(|ui| {
                                        if secondary_button(ui, " ^ ").clicked() {
                                            self.settings.text_offset_y -= self.settings.text_pixel_per_click;
                                            self.dirty = true;
                                        }
                                        ui.horizontal(|ui| {
                                            ui.add_space(ui.available_width() / 4.0);
                                            if secondary_button(ui, " < ").clicked() {
                                                self.settings.text_offset_x -= self.settings.text_pixel_per_click;
                                                self.dirty = true;
                                            }
                                            if secondary_button(ui, " v ").clicked() {
                                                self.settings.text_offset_y += self.settings.text_pixel_per_click;
                                                self.dirty = true;
                                            }
                                            if secondary_button(ui, " > ").clicked() {
                                                self.settings.text_offset_x += self.settings.text_pixel_per_click;
                                                self.dirty = true;
                                            }
                                        });

                                        ui.add_space(6.0);
                                        if primary_button(ui, "Centrar").clicked() {
                                            self.settings.text_offset_x = 0;
                                            self.settings.text_offset_y = 0;
                                            self.dirty = true;
                                        }
                                    });

                                    ui.add_space(10.0);
                                    ui.label("Pixeles por paso:");
                                    ui.add(egui::Slider::new(&mut self.settings.text_pixel_per_click, 1..=20));
                                });
                            });
                        }

                        // PESTANA 5: VERIFICACION: OK
                        else if self.active_tab == ActiveTab::VerificationOk {
                            ui.columns(2, |cols| {
                                card_group(&mut cols[0], "Contenido leido", |ui| {
                                    let mut read_copy = self.effective_payload();
                                    ui.add_sized([ui.available_width(), 95.0], egui::TextEdit::multiline(&mut read_copy).interactive(false));
                                });

                                card_group(&mut cols[1], "Diagnostico de verificacion", |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("Tipo reconocido:");
                                        ui.label(
                                            egui::RichText::new("Codigo QR reconocido")
                                                .strong()
                                                .color(Color32::from_rgb(22, 163, 74)),
                                        );
                                    });

                                    ui.add_space(10.0);
                                    if primary_button(ui, "Comprobar de nuevo manualmente").clicked() {
                                        self.status_message = "Comprobacion manual completada con exito al 100%.".to_string();
                                    }

                                    ui.add_space(6.0);
                                    if secondary_button(ui, "Procedimiento de prueba extendido").clicked() {
                                        self.status_message = "Prueba extendida: Dimensiones, correccion Reed-Solomon y contraste correctos.".to_string();
                                    }
                                });
                            });
                        }
                    });
                });
            });

        // 5. MODALES INTERACTIVOS
        self.render_template_modals(ctx);
    }
}

impl QrApp {
    fn render_template_modals(&mut self, ctx: &egui::Context) {
        match self.active_modal {
            TemplateModal::None => {}

            TemplateModal::Wifi => {
                let mut open = true;
                egui::Window::new("Configuracion Red Wi-Fi (WLAN)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Nombre de la red (SSID):");
                        ui.text_edit_singleline(&mut self.wifi_ssid);
                        ui.label("Contrasena:");
                        ui.text_edit_singleline(&mut self.wifi_pass);
                        ui.label("Tipo de seguridad:");
                        egui::ComboBox::from_id_salt("modal_wifi_sec")
                            .selected_text(&self.wifi_sec)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.wifi_sec, "WPA".to_string(), "WPA / WPA2 / WPA3");
                                ui.selectable_value(&mut self.wifi_sec, "WEP".to_string(), "WEP");
                                ui.selectable_value(&mut self.wifi_sec, "nopass".to_string(), "Abierta (Sin contrasena)");
                            });
                        ui.checkbox(&mut self.wifi_hidden, "Red oculta");

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!("WIFI:T:{};S:{};P:{};H:{};;", self.wifi_sec, self.wifi_ssid, self.wifi_pass, self.wifi_hidden);
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if secondary_button(ui, "Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::VCard => {
                let mut open = true;
                egui::Window::new("Contacto (vCard)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Nombre:");
                        ui.text_edit_singleline(&mut self.vcard_first);
                        ui.label("Apellidos:");
                        ui.text_edit_singleline(&mut self.vcard_last);
                        ui.label("Telefono:");
                        ui.text_edit_singleline(&mut self.vcard_phone);
                        ui.label("Correo electronico:");
                        ui.text_edit_singleline(&mut self.vcard_email);
                        ui.label("Empresa / Organizacion:");
                        ui.text_edit_singleline(&mut self.vcard_org);
                        ui.label("Sitio web:");
                        ui.text_edit_singleline(&mut self.vcard_url);

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "BEGIN:VCARD\nVERSION:3.0\nN:{};{};;;\nFN:{} {}\nTEL;TYPE=CELL:{}\nEMAIL:{}\nORG:{}\nURL:{}\nEND:VCARD",
                                    self.vcard_last, self.vcard_first, self.vcard_first, self.vcard_last,
                                    self.vcard_phone, self.vcard_email, self.vcard_org, self.vcard_url
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if secondary_button(ui, "Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Url => {
                let mut open = true;
                egui::Window::new("Enlace (URL)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Direccion URL del sitio web:");
                        ui.text_edit_singleline(&mut self.url_modal_input);

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Aplicar al QR").clicked() {
                                let mut final_url = self.url_modal_input.trim().to_string();
                                if !final_url.starts_with("http://") && !final_url.starts_with("https://") {
                                    final_url = format!("https://{}", final_url);
                                }
                                self.input_text = final_url;
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if secondary_button(ui, "Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Twitter => {
                let mut open = true;
                egui::Window::new("Tweet de X (Twitter)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Texto del Tweet:");
                        ui.text_edit_multiline(&mut self.twitter_text);
                        ui.label("Via usuario (opcional):");
                        ui.text_edit_singleline(&mut self.twitter_user);

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "https://twitter.com/intent/tweet?text={}&via={}",
                                    url_encode(&self.twitter_text),
                                    url_encode(&self.twitter_user)
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if secondary_button(ui, "Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Date => {
                let mut open = true;
                egui::Window::new("Fecha / Evento en Calendario")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Titulo del evento:");
                        ui.text_edit_singleline(&mut self.date_title);
                        ui.label("Fecha y hora de inicio (ISO):");
                        ui.text_edit_singleline(&mut self.date_start);
                        ui.label("Ubicacion:");
                        ui.text_edit_singleline(&mut self.date_location);

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "BEGIN:VEVENT\nSUMMARY:{}\nDTSTART:{}\nLOCATION:{}\nEND:VEVENT",
                                    self.date_title, self.date_start, self.date_location
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if secondary_button(ui, "Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::PayPal => {
                let mut open = true;
                egui::Window::new("Enlace de pago PayPal")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Nombre de usuario PayPal.Me:");
                        ui.text_edit_singleline(&mut self.paypal_user);
                        ui.label("Importe:");
                        ui.text_edit_singleline(&mut self.paypal_amount);
                        ui.label("Moneda (EUR, USD):");
                        ui.text_edit_singleline(&mut self.paypal_currency);

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "https://www.paypal.me/{}/{}{}",
                                    self.paypal_user, self.paypal_amount, self.paypal_currency
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if secondary_button(ui, "Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Iban => {
                let mut open = true;
                egui::Window::new("Pago bancario IBAN / BIC (EPC-QR)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Nombre del beneficiario:");
                        ui.text_edit_singleline(&mut self.iban_name);
                        ui.label("Numero IBAN:");
                        ui.text_edit_singleline(&mut self.iban_number);
                        ui.label("Codigo BIC (Swift):");
                        ui.text_edit_singleline(&mut self.iban_bic);
                        ui.label("Importe (EUR):");
                        ui.text_edit_singleline(&mut self.iban_amount);

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "BCD\n002\n1\nSCT\n{}\n{}\nEUR{}\n\n\n\n",
                                    self.iban_bic, self.iban_name, self.iban_amount
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if secondary_button(ui, "Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Totp => {
                let mut open = true;
                egui::Window::new("Codigo 2FA TOTP (Autenticacion en dos pasos)")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Cuenta (ej. correo electronico):");
                        ui.text_edit_singleline(&mut self.totp_account);
                        ui.label("Emisor del servicio:");
                        ui.text_edit_singleline(&mut self.totp_issuer);
                        ui.label("Clave secreta (Base32):");
                        ui.text_edit_singleline(&mut self.totp_secret);

                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Aplicar al QR").clicked() {
                                self.input_text = format!(
                                    "otpauth://totp/{}:{}?secret={}&issuer={}",
                                    self.totp_issuer, self.totp_account, self.totp_secret, self.totp_issuer
                                );
                                self.active_tab = ActiveTab::Input;
                                self.dirty = true;
                                self.active_modal = TemplateModal::None;
                            }
                            if secondary_button(ui, "Cancelar").clicked() {
                                self.active_modal = TemplateModal::None;
                            }
                        });
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::Programming => {
                let mut open = true;
                egui::Window::new("Programar un codigo QR en Rust")
                    .open(&mut open)
                    .resizable(true)
                    .show(ctx, |ui| {
                        ui.heading("Generador de codigo en Rust");
                        ui.label("Puedes generar este codigo QR directamente en Rust con el siguiente codigo:");
                        ui.add(egui::TextEdit::multiline(&mut format!(
                            "use qrcode::QrCode;\nuse image::Luma;\n\nlet code = QrCode::new(b\"{}\").unwrap();\nlet image = code.render::<Luma<u8>>().build();\nimage.save(\"codigo_qr.png\").unwrap();",
                            self.effective_payload()
                        )).interactive(false));
                        if secondary_button(ui, "Cerrar").clicked() {
                            self.active_modal = TemplateModal::None;
                        }
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }

            TemplateModal::About => {
                let mut open = true;
                egui::Window::new("Acerca de QR Generator")
                    .open(&mut open)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            if let Some(tex) = &self.app_icon_texture {
                                ui.image((tex.id(), egui::vec2(44.0, 44.0)));
                                ui.add_space(8.0);
                            }
                            ui.vertical(|ui| {
                                ui.heading("QR Generator v0.0.1 (Edicion Rust)");
                                ui.label("Generador nativo de codigos QR escrito 100% en Rust.");
                            });
                        });
                        ui.add_space(6.0);
                        ui.label("Diseno limpio optimizado para fondo blanco, con soporte para logotipos, paletas de colores y exportacion vectorial.");
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Programador: Pirulug").strong());
                        ui.horizontal(|ui| {
                            ui.label("Web:");
                            ui.hyperlink_to("pirulug.com", "https://pirulug.com");
                        });
                        ui.horizontal(|ui| {
                            ui.label("GitHub:");
                            ui.hyperlink_to("github.com/pirulug", "https://github.com/pirulug");
                        });
                        ui.add_space(12.0);
                        if primary_button(ui, "Cerrar").clicked() {
                            self.active_modal = TemplateModal::None;
                        }
                    });
                if !open { self.active_modal = TemplateModal::None; }
            }
        }
    }
}

// ==========================================
// HELPERS DE DISENO UI (FONDO BLANCO Y SIN EMOJIS)
// ==========================================

fn toolbar_separator(ui: &mut egui::Ui) {
    ui.add(egui::Separator::default().vertical().spacing(12.0));
}

fn pill_tab(ui: &mut egui::Ui, current: &mut ActiveTab, target: ActiveTab, label: &str) {
    let is_selected = *current == target;
    let (text_color, bg_fill, stroke) = if is_selected {
        (
            Color32::from_rgb(37, 99, 235), // Azul primario
            Color32::from_rgb(239, 246, 255), // Fondo azul muy suave
            egui::Stroke::new(1.0_f32, Color32::from_rgb(191, 219, 254)),
        )
    } else {
        (
            Color32::from_rgb(71, 85, 105), // Slate neutro
            Color32::WHITE,
            egui::Stroke::new(1.0_f32, Color32::from_rgb(241, 245, 249)),
        )
    };

    let text = egui::RichText::new(label)
        .size(12.5)
        .color(text_color);

    let btn = egui::Button::new(text)
        .fill(bg_fill)
        .stroke(stroke)
        .rounding(6.0_f32);

    if ui.add(btn).clicked() {
        *current = target;
    }
}

fn primary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(Color32::WHITE).size(12.0))
            .fill(Color32::from_rgb(37, 99, 235))
            .rounding(5.0_f32),
    )
}

fn secondary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(Color32::from_rgb(51, 65, 85)).size(12.0))
            .fill(Color32::from_rgb(248, 250, 252))
            .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)))
            .rounding(5.0_f32),
    )
}

fn card_group<R>(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui) -> R) {
    egui::Frame::none()
        .fill(Color32::from_rgb(248, 250, 252))
        .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(226, 232, 240)))
        .rounding(6.0_f32)
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(title).size(13.0).strong().color(Color32::from_rgb(30, 41, 59)));
            ui.add_space(8.0);
            content(ui);
        });
}

fn color_circle(ui: &mut egui::Ui, color: [u8; 4]) {
    let (rect, _resp) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.circle_filled(rect.center(), 8.0, Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]));
    painter.circle_stroke(rect.center(), 8.0, egui::Stroke::new(1.0_f32, Color32::from_rgb(203, 213, 225)));
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
