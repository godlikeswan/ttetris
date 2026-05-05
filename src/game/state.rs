use crate::game::{color::Color, piece::PieceType};

pub struct GameState {
    pub hold: Option<PieceType>,
    pub current_piece: PieceType,
    pub queue: [PieceType; 5],
    pub rng_state: i64,
    pub field: [[Color; 40]; 10],
}