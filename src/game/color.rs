#[derive(Debug, Copy, Clone)]
pub enum Color {
    RED,
    YELLOW,
    ORANGE,
    GREEN,
    CYAN,
    BLUE,
    PURPLE,
    BLACK,
    GRAY,
    WHITE,
}

impl Into<*const f32> for Color {
    fn into(self) -> *const f32 {
        match self {
            Color::RED => [1.0, 0.0, 0.0].as_ptr(),
            Color::YELLOW => [1.0, 1.0, 0.0].as_ptr(),
            Color::ORANGE => [1.0, 0.5, 0.0].as_ptr(),
            Color::GREEN => [0.0, 1.0, 0.0].as_ptr(),
            Color::CYAN => [0.0, 1.0, 1.0].as_ptr(),
            Color::BLUE => [0.0, 0.0, 1.0].as_ptr(),
            Color::PURPLE => [1.0, 0.0, 1.0].as_ptr(),
            Color::BLACK => [0.0, 0.0, 0.0].as_ptr(),
            Color::GRAY => [0.2, 0.2, 0.2].as_ptr(),
            Color::WHITE => [1.0, 1.0, 1.0].as_ptr(),
        }
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        match self {
            Color::RED => [1.0, 0.0, 0.0],
            Color::YELLOW => [1.0, 1.0, 0.0],
            Color::ORANGE => [1.0, 0.5, 0.0],
            Color::GREEN => [0.0, 1.0, 0.0],
            Color::CYAN => [0.0, 1.0, 1.0],
            Color::BLUE => [0.0, 0.0, 1.0],
            Color::PURPLE => [1.0, 0.0, 1.0],
            Color::BLACK => [0.0, 0.0, 0.0],
            Color::GRAY => [0.5, 0.5, 0.5],
            Color::WHITE => [1.0, 1.0, 1.0],
        }
    }
}
