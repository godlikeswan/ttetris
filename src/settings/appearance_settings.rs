use std::mem;

use windows::Win32::{
    Foundation::{POINT, RECT},
    UI::WindowsAndMessaging::{WINDOWPLACEMENT, WINDOWPLACEMENT_FLAGS},
};

mod window_placement;
use window_placement::WINDOWPLACEMENTdef;

// # Appearance
pub const INIT_WIDTH: i32 = 850;
pub const INIT_HEIGHT: i32 = 600;

pub const DRAW_SHADOW: bool = true;
pub const SHADOW_ALPHA: f32 = 0.5;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AppearanceSettings {
    pub help: String,
    pub draw_shadow: bool,
    pub shadow_alpha: f32,
    pub save_last_window_placement: bool,
    #[serde(with = "WINDOWPLACEMENTdef")]
    pub window_placement: WINDOWPLACEMENT,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            help: "\
To always open window at sertain position first disable save_last_window_placement (so the following values are not overwritten)
and refer to https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement
to change window_placement as desired
"
            .to_string(),
            draw_shadow: DRAW_SHADOW,
            shadow_alpha: SHADOW_ALPHA,
            save_last_window_placement: true,
            window_placement: WINDOWPLACEMENT {
                length: mem::size_of::<WINDOWPLACEMENT>() as _ ,
                flags: WINDOWPLACEMENT_FLAGS(0),
                showCmd: 1,
                ptMinPosition: POINT {
                    x: -1,
                    y: -1,
                },
                ptMaxPosition: POINT {
                    x: -1,
                    y: -1,
                },
                rcNormalPosition: RECT {
                    left: 0,
                    top: 0,
                    right: INIT_WIDTH,
                    bottom: INIT_HEIGHT,
                }
            }
        }
    }
}
