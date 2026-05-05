use std::collections::VecDeque;

use rand::{rngs::SmallRng, seq::SliceRandom};
use windows::Win32::Graphics::OpenGL::{
    glColor3fv, glPopMatrix, glPushMatrix, glRecti, glTranslatef,
};

use crate::game::piece::Piece;

pub struct Queue {
    pub buffer: VecDeque<Piece>,
    pub bag: Vec<Piece>,
}

impl Queue {
    pub fn new() -> Self {
        let mut queue = Queue {
            buffer: VecDeque::new(),
            bag: Vec::new(),
        };
        queue.refill_bag();
        for _ in 0..5 {
            queue.buffer.push_back(queue.bag.pop().unwrap());
        }
        return queue;
    }

    pub fn shift(&mut self) -> Piece {
        let piece = self.buffer.pop_front().unwrap();
        self.buffer.push_back(self.bag.pop().unwrap());
        if self.bag.is_empty() {
            self.refill_bag()
        };
        return piece;
    }

    pub fn render(&self) {
        unsafe {
            glPushMatrix();
            for piece in &self.buffer {
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

    fn refill_bag(&mut self) {
        for piece_type in Piece::ALL_TYPES {
            self.bag.push(Piece::new(piece_type));
        }
        let mut rng: SmallRng = rand::make_rng();
        // let mut rng = rngs::SmallRng::seed_from_u64(0);
        self.bag.shuffle(&mut rng);
    }
}
