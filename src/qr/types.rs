use qrcode::EcLevel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCorrection {
    Low,
    Medium,
    Quartile,
    High,
}

impl ErrorCorrection {
    pub fn to_ec_level(self) -> EcLevel {
        match self {
            Self::Low => EcLevel::L,
            Self::Medium => EcLevel::M,
            Self::Quartile => EcLevel::Q,
            Self::High => EcLevel::H,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Low => "Bajo (L)",
            Self::Medium => "Medio (M)",
            Self::Quartile => "Cuartil (Q)",
            Self::High => "Alto (H)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogoTemplate {
    None,
    AppIcon,
    HeartPulse,
    Star,
    Shield,
    Wifi,
    TwitterX,
    UserContact,
}

impl LogoTemplate {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "---",
            Self::AppIcon => "Icono de la aplicacion (icon.ico)",
            Self::HeartPulse => "Corazon / Pulso",
            Self::Star => "Estrella",
            Self::Shield => "Escudo",
            Self::Wifi => "Simbolo Wi-Fi",
            Self::TwitterX => "X (Twitter)",
            Self::UserContact => "Avatar de contacto",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextCasing {
    TextField,
    Small,
    Great,
}

#[derive(Clone, Debug)]
pub struct QrSettings {
    // Matrix settings
    pub ec_level: ErrorCorrection,
    pub border: usize,
    pub scale_factor: usize,
    pub fg_color: [u8; 4],
    pub bg_color: [u8; 4],

    // Logo settings
    pub logo_enabled: bool,
    pub logo_template: LogoTemplate,
    pub logo_size_px: u32,
    pub keep_centered_enlarge: bool,
    pub check_validity: bool,
    pub logo_offset_x: i32,
    pub logo_offset_y: i32,
    pub logo_pixel_per_click: i32,

    // Text settings
    pub text_enabled: bool,
    pub text_content: String,
    pub text_font: String,
    pub _text_icon: String,
    pub text_casing: TextCasing,
    pub text_bold: bool,
    pub text_italic: bool,
    pub text_underline: bool,
    pub text_color: [u8; 4],
    pub text_font_size: f32,
    pub text_offset_x: i32,
    pub text_offset_y: i32,
    pub text_pixel_per_click: i32,
}

impl Default for QrSettings {
    fn default() -> Self {
        Self {
            ec_level: ErrorCorrection::Low,
            border: 2,
            scale_factor: 7,
            fg_color: [0, 0, 0, 255],
            bg_color: [255, 255, 255, 255],

            logo_enabled: false,
            logo_template: LogoTemplate::HeartPulse,
            logo_size_px: 100,
            keep_centered_enlarge: true,
            check_validity: true,
            logo_offset_x: 0,
            logo_offset_y: 0,
            logo_pixel_per_click: 5,

            text_enabled: false,
            text_content: "Texto de ejemplo".to_string(),
            text_font: "Predeterminada".to_string(),
            _text_icon: "Icono de mano a la derecha".to_string(),
            text_casing: TextCasing::TextField,
            text_bold: true,
            text_italic: false,
            text_underline: false,
            text_color: [255, 0, 128, 255],
            text_font_size: 16.0,
            text_offset_x: 0,
            text_offset_y: 0,
            text_pixel_per_click: 5,
        }
    }
}
