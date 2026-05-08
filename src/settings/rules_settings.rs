// # Rules
pub const NO_GRAVITY: bool = true;
// in ms, the greater the slower pieces fall
pub const FALL_DELAY: i64 = 500;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RulesSettings {
    pub help: String,
    pub fall_delay: i64,
    pub no_gravity: bool,
}

impl Default for RulesSettings {
    fn default() -> Self {
        Self {
            help: "\
fall_delay (in ms) is the period between each movement of a falling piece, the more the slower
no_gravity disables falling entirely
"
            .to_string(),
            fall_delay: FALL_DELAY,
            no_gravity: NO_GRAVITY,
        }
    }
}
