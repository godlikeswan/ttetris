use windows::Win32::Graphics::OpenGL::{
    glColor3fv, glPopMatrix, glPushMatrix, glRecti, glTranslatef,
};

use crate::game::piece::Piece;

pub struct Hold {
    pub piece: Option<Piece>,
    pub was_used: bool,
}

impl Hold {
    pub fn new() -> Hold {
        Hold {
            piece: None,
            was_used: false,
        }
    }

    pub fn render(&self) {
        if self.piece.is_none() {
            return;
        }
        let piece = self.piece.as_ref().unwrap();
        unsafe {
            glPushMatrix();
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
            glPopMatrix();
        }
    }
}
