use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY, VK_A, VK_C, VK_DOWN, VK_LEFT, VK_RIGHT, VK_SHIFT, VK_UP, VK_V, VK_X, VK_Z
};

// # Controls
pub const KEY_LEFT: VIRTUAL_KEY = VK_LEFT;
pub const KEY_RIGHT: VIRTUAL_KEY = VK_RIGHT;
pub const KEY_HARD_DROP: VIRTUAL_KEY = VK_UP;
pub const KEY_SOFT_DROP: VIRTUAL_KEY = VK_DOWN;

pub const KEY_TURN_180: VIRTUAL_KEY = VK_SHIFT;
pub const KEY_TURN_CCW: VIRTUAL_KEY = VK_Z;
pub const KEY_TURN_CW: VIRTUAL_KEY = VK_X;
pub const KEY_HOLD: VIRTUAL_KEY = VK_C;

pub const KEY_RESTART: VIRTUAL_KEY = VK_V;
pub const KEY_UNDO: VIRTUAL_KEY = VK_A;

// # Handling
// in ms, Delay Auto Shift
pub const DAS: i64 = 150;

// # Appearance
pub const INIT_WIDTH: i32 = 850;
pub const INIT_HEIGHT: i32 = 600;

pub const DRAW_SHADOW: bool = true;
pub const SHADOW_ALPHA: f32 = 0.5;

// # Rules
pub const NO_GRAVITY: bool = true;
// in ms, the greater the slower pieces fall
pub const FALL_DELAY: i64 = 500;
