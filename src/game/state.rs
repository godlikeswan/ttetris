use crate::game::{color::Color, piece::PieceType};

pub struct GameState {
    pub hold: Option<PieceType>,
    pub current_piece: PieceType,
    pub queue: Vec<PieceType>,
    pub rng_state: u64,
    pub field: [[Color; 40]; 10],
}
