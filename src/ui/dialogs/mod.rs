pub mod about;
pub mod modal;
pub mod programming;
pub mod templates;

pub use about::show_about_dialog;
pub use modal::show_single_input_dialog;
pub use programming::show_programming_dialog;
pub use templates::*;
