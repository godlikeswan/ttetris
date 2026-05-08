use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY, VK_A, VK_C, VK_DOWN, VK_ESCAPE, VK_LEFT, VK_RIGHT, VK_SHIFT, VK_UP, VK_V, VK_X,
    VK_Z,
};

// # Default controls
const KEY_LEFT: VIRTUAL_KEY = VK_LEFT;
const KEY_RIGHT: VIRTUAL_KEY = VK_RIGHT;
const KEY_HARD_DROP: VIRTUAL_KEY = VK_UP;
const KEY_SOFT_DROP: VIRTUAL_KEY = VK_DOWN;

const KEY_TURN_180: VIRTUAL_KEY = VK_SHIFT;
const KEY_TURN_CCW: VIRTUAL_KEY = VK_Z;
const KEY_TURN_CW: VIRTUAL_KEY = VK_X;
const KEY_HOLD: VIRTUAL_KEY = VK_C;

const KEY_RESTART: VIRTUAL_KEY = VK_V;
const KEY_UNDO: VIRTUAL_KEY = VK_A;
const KEY_EXIT: VIRTUAL_KEY = VK_ESCAPE;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ControlsSettings {
    pub help: String,
    pub left: u16,
    pub right: u16,
    pub hard_drop: u16,
    pub soft_drop: u16,
    pub turn_180: u16,
    pub turn_ccw: u16,
    pub turn_cw: u16,
    pub hold: u16,
    pub restart: u16,
    pub undo: u16,
    pub exit: u16,
}

impl Default for ControlsSettings {
    fn default() -> Self {
        Self {
            help: "\
Key bindings.
Refer to https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes to find the mappings.
(You can directly insert values in the hex format 0x... - they will be translated automatically)
"
            .to_string(),
            left: KEY_LEFT.0,
            right: KEY_RIGHT.0,
            hard_drop: KEY_HARD_DROP.0,
            soft_drop: KEY_SOFT_DROP.0,
            turn_180: KEY_TURN_180.0,
            turn_ccw: KEY_TURN_CCW.0,
            turn_cw: KEY_TURN_CW.0,
            hold: KEY_HOLD.0,
            restart: KEY_RESTART.0,
            undo: KEY_UNDO.0,
            exit: KEY_EXIT.0,
        }
    }
}
