// IDs de comandos de Menu
pub const ID_FILE_NEW: u16 = 101;
pub const ID_FILE_OPEN: u16 = 102;
pub const ID_FILE_SAVE: u16 = 103;
pub const ID_FILE_PRINT: u16 = 104;
pub const ID_FILE_REFRESH: u16 = 105;
pub const ID_FILE_EXIT: u16 = 106;

pub const ID_EXPORT_PNG: u16 = 110;
pub const ID_EXPORT_JPEG: u16 = 111;
pub const ID_EXPORT_SVG: u16 = 112;
pub const ID_EXPORT_EPS: u16 = 113;
pub const ID_EXPORT_BASE64: u16 = 114;
pub const ID_EXPORT_MARKDOWN: u16 = 115;

pub const ID_EDIT_COPY: u16 = 201;
pub const ID_EDIT_CLEAR: u16 = 202;

pub const ID_TEMPLATE_TEXT: u16 = 301;
pub const ID_TEMPLATE_URL: u16 = 302;
pub const ID_TEMPLATE_TWITTER: u16 = 303;
pub const ID_TEMPLATE_VCARD: u16 = 304;
pub const ID_TEMPLATE_WIFI: u16 = 305;
pub const ID_TEMPLATE_DATE: u16 = 306;
pub const ID_TEMPLATE_PAYPAL: u16 = 307;
pub const ID_TEMPLATE_IBAN: u16 = 308;
pub const ID_TEMPLATE_TOTP: u16 = 309;
pub const ID_TEMPLATE_PROGRAMMING: u16 = 310;

pub const ID_VIEW_RESET: u16 = 401;
pub const ID_CONFIG_LANG: u16 = 501;
pub const ID_HELP_ABOUT: u16 = 601;

// IDs de controles de Barra de Herramientas
pub const ID_BTN_REFRESH: u16 = 1001;
pub const ID_BTN_TEMPLATES_MENU: u16 = 1002;
pub const ID_BTN_COPY: u16 = 1003;
pub const ID_BTN_VERIFY_TAB: u16 = 1004;
pub const ID_BTN_EXPORT_MENU: u16 = 1005;
pub const ID_BTN_TOOLS_MENU: u16 = 1006;
pub const ID_BTN_CANVAS_SCALE: u16 = 1007;

// IDs de Controles de Pestanas
pub const ID_TAB_CONTROL: u16 = 1100;
pub const ID_TAB_BTN_0: u16 = 1101;
pub const ID_TAB_BTN_1: u16 = 1102;
pub const ID_TAB_BTN_2: u16 = 1103;
pub const ID_TAB_BTN_3: u16 = 1104;
pub const ID_TAB_BTN_4: u16 = 1105;

// Pestana 0: Entrada
pub const ID_EDIT_INPUT: u16 = 1201;
pub const ID_CHK_USE_TEXT_AS_INPUT: u16 = 1202;
pub const ID_BTN_IMPORT_FILE: u16 = 1203;
pub const ID_BTN_CLEAR_INPUT: u16 = 1204;
pub const ID_LBL_CHAR_COUNT: u16 = 1205;
pub const ID_BTN_PASTE_INPUT: u16 = 1206;
pub const ID_BTN_COPY_INPUT: u16 = 1207;

// Pestana 1: Colores y tamano
pub const ID_COMBO_EC: u16 = 1301;
pub const ID_COMBO_BORDER: u16 = 1302;
pub const ID_COMBO_SCALE: u16 = 1303;
pub const ID_LBL_FINAL_SIZE: u16 = 1304;
pub const ID_CHK_SAVE_SETTINGS: u16 = 1305;
pub const ID_BTN_COLOR_BG: u16 = 1306;
pub const ID_BTN_COLOR_FG: u16 = 1307;
pub const ID_BTN_RANDOM_COLORS: u16 = 1308;
pub const ID_BTN_SWAP_COLORS: u16 = 1309;
pub const ID_BTN_PALETTE_CLASSIC: u16 = 1310;
pub const ID_BTN_PALETTE_TECH_BLUE: u16 = 1311;
pub const ID_BTN_PALETTE_BIO_GREEN: u16 = 1312;
pub const ID_BTN_PALETTE_DARK_MODE: u16 = 1313;

// Pestana 2: Logo en QR
pub const ID_CHK_LOGO_ENABLED: u16 = 1401;
pub const ID_BTN_SELECT_LOGO: u16 = 1402;
pub const ID_BTN_REMOVE_LOGO: u16 = 1403;
pub const ID_LBL_LOGO_PATH: u16 = 1404;
pub const ID_COMBO_LOGO_TEMPLATE: u16 = 1405;
pub const ID_LBL_LOGO_SIZE: u16 = 1406;
pub const ID_TRACK_LOGO_SIZE: u16 = 1407;
pub const ID_CHK_KEEP_CENTERED: u16 = 1408;
pub const ID_CHK_CHECK_VALIDITY: u16 = 1409;
pub const ID_BTN_AUTO_ADJUST: u16 = 1410;
pub const ID_BTN_LOGO_UP: u16 = 1411;
pub const ID_BTN_LOGO_DOWN: u16 = 1412;
pub const ID_BTN_LOGO_LEFT: u16 = 1413;
pub const ID_BTN_LOGO_RIGHT: u16 = 1414;
pub const ID_BTN_LOGO_CENTER: u16 = 1415;
pub const ID_COMBO_LOGO_STEP: u16 = 1416;

// Pestana 3: Texto en QR
pub const ID_CHK_TEXT_ENABLED: u16 = 1501;
pub const ID_EDIT_TEXT_CONTENT: u16 = 1502;
pub const ID_COMBO_TEXT_FONT: u16 = 1503;
pub const ID_RADIO_CASING_ORIG: u16 = 1504;
pub const ID_RADIO_CASING_LOWER: u16 = 1505;
pub const ID_RADIO_CASING_UPPER: u16 = 1506;
pub const ID_CHK_TEXT_BOLD: u16 = 1507;
pub const ID_CHK_TEXT_ITALIC: u16 = 1508;
pub const ID_CHK_TEXT_UNDERLINE: u16 = 1509;
pub const ID_BTN_TEXT_COLOR: u16 = 1510;
pub const ID_LBL_TEXT_COLOR_HEX: u16 = 1511;
pub const ID_LBL_TEXT_FONT_SIZE: u16 = 1512;
pub const ID_TRACK_TEXT_FONT_SIZE: u16 = 1513;
pub const ID_BTN_TEXT_UP: u16 = 1514;
pub const ID_BTN_TEXT_DOWN: u16 = 1515;
pub const ID_BTN_TEXT_LEFT: u16 = 1516;
pub const ID_BTN_TEXT_RIGHT: u16 = 1517;
pub const ID_BTN_TEXT_CENTER: u16 = 1518;
pub const ID_COMBO_TEXT_STEP: u16 = 1519;
pub const ID_COMBO_TEXT_CASING: u16 = 1520;

// Pestana 4: Verificacion
pub const ID_EDIT_VERIFY_READ: u16 = 1601;
pub const ID_BTN_VERIFY_MANUAL: u16 = 1602;
pub const ID_BTN_VERIFY_EXTENDED: u16 = 1603;

// Constantes Win32 auxiliares
pub const CF_UNICODETEXT: u32 = 13;
pub const BST_UNCHECKED: usize = 0;
pub const BST_CHECKED: usize = 1;
pub const TBM_GETPOS: u32 = 1024;
