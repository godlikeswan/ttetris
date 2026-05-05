use crate::game::piece::rotation::{ROTATION_TESTS, ROTATION_TESTS_I};
use crate::game::piece::{color::COLORS, shape::SHAPES};

mod color;
mod rotation;
mod shape;

use crate::game::color::Color;

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    I = 0,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub struct Piece {
    pub r#type: PieceType,
}

impl Piece {
    pub const ALL_TYPES: [PieceType; 7] = [
        PieceType::I,
        PieceType::J,
        PieceType::L,
        PieceType::O,
        PieceType::S,
        PieceType::T,
        PieceType::Z,
    ];

    pub fn new(r#type: PieceType) -> Piece {
        Piece { r#type }
    }

    pub fn get_shape(&self, rotation: i32) -> &[[bool; 4]; 4] {
        &SHAPES[self.r#type as usize][((rotation + 4) % 4) as usize]
    }

    pub fn get_color(&self) -> &Color {
        &COLORS[self.r#type as usize]
    }

    pub fn get_rotation_tests(
        &self,
        current_rotation: i32,
        target_rotation: i32,
    ) -> &[[i32; 2]; 5] {
        let rotation_tests = if matches!(self.r#type, PieceType::I) {
            &ROTATION_TESTS_I
        } else {
            &ROTATION_TESTS
        };
        match (current_rotation, target_rotation) {
            (0, 1) => &rotation_tests[0],
            (1, 0) => &rotation_tests[1],
            (1, 2) => &rotation_tests[2],
            (2, 1) => &rotation_tests[3],
            (2, -1) => &rotation_tests[4],
            (-1, 2) => &rotation_tests[5],
            (-1, 0) => &rotation_tests[6],
            (0, -1) => &rotation_tests[7],
            _ => panic!("Wrong rotation"),
        }
    }
}
