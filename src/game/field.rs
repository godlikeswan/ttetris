use windows::Win32::Graphics::OpenGL::{
    GL_LINES, glBegin, glColor3fv, glEnd, glPopMatrix, glPushMatrix, glRecti, glScalef,
    glTranslatef, glVertex2i,
};

use crate::game::color::Color;

pub struct Field {
    pub buffer: [[Color; 40]; 10],
}

impl Field {
    pub fn new() -> Self {
        Self {
            buffer: [[Color::BLACK; 40]; 10],
        }
    }

    pub fn clear_full_lines(&mut self) {
        let mut y = 0;
        while y < 40 {
            while self.is_line_full(y) {
                self.clear_line(y);
            }
            y += 1;
        }
    }

    pub fn is_line_full(&self, y: i32) -> bool {
        for i in 0..10 {
            if matches!(self.buffer[i][y as usize], Color::BLACK) {
                return false;
            }
        }
        true
    }

    pub fn clear_line(&mut self, y: i32) {
        for i in y..39 {
            for j in 0..10 {
                self.buffer[j][i as usize] = self.buffer[j][(i + 1) as usize];
            }
        }
        for j in 0..10 {
            self.buffer[j][39] = Color::BLACK;
        }
    }

    pub fn render(&self) {
        unsafe {
            glPushMatrix();
            glTranslatef(0.0, 22.0, 0.0);
            glScalef(1.0, -1.0, 1.0);

            glColor3fv(Color::BLACK.into());
            glRecti(0, 0, 10, 22);

            glColor3fv(Color::GRAY.into());
            glBegin(GL_LINES);
            for i in 0..=22 {
                glVertex2i(0, i);
                glVertex2i(10, i);
            }
            for i in 0..=10 {
                glVertex2i(i, 0);
                glVertex2i(i, 22);
            }
            glEnd();

            for i in 0..10 {
                for j in 0..22 {
                    if matches!(self.buffer[i][j], Color::BLACK) {
                        continue;
                    }
                    glColor3fv(self.buffer[i][j].into());
                    glRecti(i as _, j as _, i as i32 + 1, j as i32 + 1);
                }
            }

            glColor3fv(Color::RED.into());
            glBegin(GL_LINES);
            glVertex2i(0, 20);
            glVertex2i(10, 20);
            glEnd();

            glPopMatrix();
        }
    }
}
