use windows::Win32::Graphics::OpenGL::{
    glColor3fv, glColor4f, glPopMatrix, glPushMatrix, glRecti, glScalef, glTranslatef,
};

use crate::{
    game::{color::Color, field::Field, piece::Piece},
    settings,
};

pub struct CurrentPiece {
    pub piece: Piece,
    pub x: i32,
    pub y: i32,
    pub r: i32,
    pub since_last_move_down: i64,
}

impl CurrentPiece {
    pub fn new(piece: Piece) -> CurrentPiece {
        Self {
            piece,
            x: 3,
            y: 21,
            r: 0,
            since_last_move_down: 0,
        }
    }

    pub fn set(&mut self, piece: Piece) {
        self.x = 3;
        self.y = 21;
        self.r = 0;
        self.since_last_move_down = 0;
        self.piece = piece;
    }

    pub fn update(&mut self, diff: i64, field: &Field) -> bool {
        self.since_last_move_down += diff;
        let g_period = settings::FALL_DELAY * 1000;
        if self.since_last_move_down > g_period {
            self.since_last_move_down -= g_period;
            if settings::NO_GRAVITY {
                return true;
            }
            if self.can_go_down(field) {
                self.y -= 1;
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    pub fn soft_drop(&mut self, field: &Field) {
        while self.can_go_down(field) {
            self.y -= 1;
        }
        self.since_last_move_down = 0;
    }

    fn can_go_down(&self, field: &Field) -> bool {
        self.can_move(field, 0, -1)
    }

    pub fn try_go_right(&mut self, field: &Field) -> bool {
        if self.can_move(field, 1, 0) {
            self.x += 1;
            self.since_last_move_down = 0;
            return true;
        }
        false
    }

    pub fn try_go_left(&mut self, field: &Field) -> bool {
        if self.can_move(field, -1, 0) {
            self.x -= 1;
            self.since_last_move_down = 0;
            return true;
        }
        false
    }

    pub fn try_turn(&mut self, field: &Field, dr: i32) {
        let new_r = ((self.r + 1 + 4 + dr) % 4) - 1;
        if dr == 2 {
            if Self::can_fit(field, &self.piece, self.x, self.y, new_r) {
                self.since_last_move_down = 0;
                self.r = new_r;
                return;
            }
        }
        let rotation_tests = self.piece.get_rotation_tests(self.r, new_r);
        for i in 0..5 {
            let dx = rotation_tests[i][0];
            let new_x = self.x + dx;
            let dy = rotation_tests[i][1];
            let new_y = self.y + dy;
            if Self::can_fit(field, &self.piece, self.x + dx, self.y + dy, new_r) {
                self.since_last_move_down = 0;
                self.x = new_x;
                self.y = new_y;
                self.r = new_r;
                return;
            }
        }
    }

    fn can_move(&self, field: &Field, dx: i32, dy: i32) -> bool {
        Self::can_fit(field, &self.piece, self.x + dx, self.y + dy, self.r)
    }

    pub fn render(&self, field: &Field) {
        let piece = &self.piece;
        unsafe {
            glPushMatrix();
            let shape = piece.get_shape(self.r);

            glTranslatef(0.0, 21.0, 0.0);
            glScalef(1.0, -1.0, 1.0);
            glTranslatef(self.x as f32, self.y as f32, 0.0);

            // glScalef(1.0, -1.0, 1.0);

            if settings::DRAW_SHADOW {
                let mut dy_shadow = 0;
                while self.can_move(field, 0, dy_shadow - 1) {
                    dy_shadow -= 1;
                }

                glPushMatrix();
                glTranslatef(0.0, dy_shadow as _, 0.0);
                glScalef(1.0, -1.0, 1.0);

                let color: [f32; 3] = (*piece.get_color()).into();
                glColor4f(color[0], color[1], color[2], settings::SHADOW_ALPHA);

                for i in 0..4 {
                    for j in 0..4 {
                        if shape[i][j] {
                            let x = j as i32;
                            let y = i as i32;
                            glRecti(x, y, x + 1, y + 1);
                        }
                    }
                }
                glPopMatrix();
            }

            glColor3fv((*piece.get_color()).into());
            glScalef(1.0, -1.0, 1.0);

            for i in 0..4 {
                for j in 0..4 {
                    if shape[i][j] {
                        let x = j as i32;
                        let y = i as i32;
                        glRecti(x, y, x + 1, y + 1);
                    }
                }
            }
            glPopMatrix();
        }
    }
    pub fn can_fit(field: &Field, piece: &Piece, x: i32, y: i32, r: i32) -> bool {
        let shape = piece.get_shape(r);
        for i in 0..4 {
            for j in 0..4 {
                if shape[i][j] {
                    let y_square = y - i as i32;
                    let x_square = x + j as i32;
                    if y_square < 0
                        || x_square < 0
                        || y_square >= field.buffer[0].len() as _
                        || x_square >= field.buffer.len() as _
                    {
                        return false;
                    }
                    if !matches!(
                        field.buffer[x_square as usize][y_square as usize],
                        Color::BLACK
                    ) {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}
