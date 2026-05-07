use std::collections::VecDeque;
use windows::Win32::Graphics::OpenGL::{
    glColor3fv, glPopMatrix, glPushMatrix, glRecti, glTranslatef,
};

use crate::game::piece::Piece;
use crate::game::rng::Rng;

pub struct Queue {
    pub buffer: VecDeque<Piece>,
    pub rng: Rng,
}

impl Queue {
    pub fn new() -> Self {
        let mut queue = Queue {
            buffer: VecDeque::new(),
            rng: Rng::make(),
        };
        queue.refill();
        return queue;
    }

    pub fn shift(&mut self) -> Piece {
        let piece = self.buffer.pop_front().unwrap();
        if self.buffer.len() < 5 {
            self.refill()
        };
        return piece;
    }

    pub fn render(&self) {
        unsafe {
            glPushMatrix();
            for i in 0..5 {
                let piece = &self.buffer[i];
                glColor3fv((*piece.get_color()).into());
                let shape = piece.get_shape(0);
                glPushMatrix();

                for i in 0..2 {
                    for (j, has_rect) in shape[i].iter().enumerate() {
                        if *has_rect {
                            glRecti(j as i32, i as i32, j as i32 + 1, i as i32 + 1);
                        }
                    }
                }
                glPopMatrix();
                glTranslatef(0.0, 3.0, 0.0);
            }
            glPopMatrix();
        }
    }

    fn refill(&mut self) {
        let mut bag = Piece::ALL_TYPES_RAINBOW;
        self.rng.shuffle(&mut bag);
        for piece_type in bag {
            self.buffer.push_back(Piece::new(piece_type));
        }

    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.refill();
    }
}
