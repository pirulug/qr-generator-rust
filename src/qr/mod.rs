#![allow(dead_code, unused_imports)]

pub mod engine;
pub mod export;
pub mod logo;
pub mod text;
pub mod types;

#[cfg(test)]
mod tests;

pub use engine::QrEngine;
pub use export::{base64_encode, generate_eps, generate_svg};
pub use types::{ErrorCorrection, LogoTemplate, QrSettings, TextCasing};
