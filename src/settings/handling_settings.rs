// # Handling
// in ms, Delay Auto Shift
pub const DAS: i64 = 150;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HandlingSettings {
    pub help: String,
    pub das: i64,
}

impl Default for HandlingSettings {
    fn default() -> Self {
        Self {
            help: "\
das (in ms), Delay Auto Shift - time needed to hold right or left key for a piece
to start moving quickly in the said direction on it's own
"
            .to_string(),
            das: DAS,
        }
    }
}
