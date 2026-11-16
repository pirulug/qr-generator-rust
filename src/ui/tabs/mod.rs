pub mod colors;
pub mod input;
pub mod logo;
pub mod text;
pub mod verify;

pub use colors::build_tab_colors;
pub use input::build_tab_input;
pub use logo::build_tab_logo;
pub use text::build_tab_text;
pub use verify::build_tab_verify;

use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOW};

use crate::ui::state::AppState;

pub fn update_tab_visibility(state: &AppState) {
    unsafe {
        for (i, controls) in state.tab_controls.iter().enumerate() {
            let show_cmd = if i == state.active_tab { SW_SHOW } else { SW_HIDE };
            for &ctrl in controls {
                let _ = ShowWindow(ctrl, show_cmd);
            }
        }
    }
}
