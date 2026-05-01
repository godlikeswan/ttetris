pub struct ControlsState {
    pub left: bool,
    pub left_counter: i64,
    pub right: bool,
    pub right_counter: i64,
    pub hold: bool,
}

impl ControlsState {
    pub fn new() -> ControlsState {
        Self {
            left: false,
            left_counter: 0,
            right: false,
            right_counter: 0,
            hold: false,
        }
    }
}
