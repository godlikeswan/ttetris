use std::{mem::swap, slice};

use windows::Win32::{
    Graphics::{
        Gdi::{GetStockObject, HDC, SYSTEM_FONT, SelectObject},
        OpenGL::{
            GL_BLEND, GL_COLOR_BUFFER_BIT, GL_MODELVIEW, GL_ONE_MINUS_SRC_ALPHA,
            GL_PROJECTION, GL_SRC_ALPHA, glBlendFunc, glClear, glClearColor,
            glEnable, glLoadIdentity, glMatrixMode, glOrtho, glPopMatrix, glPushMatrix, glScalef,
            glTranslatef, wglUseFontBitmapsW,
        },
    },
    System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
};

use crate::{
    game::{
        color::Color, controls::ControlsState, current_piece::CurrentPiece, field::Field,
        hold::Hold, piece::Piece, queue::Queue, state::GameState,
    },
    settings::Settings,
};

mod color;
mod controls;
mod current_piece;
mod field;
mod hold;
mod piece;
mod queue;
mod rng;
pub mod state;

pub struct Game {
    pub field: Field,
    hold: Hold,
    pub current_piece: CurrentPiece,
    pub controls_state: ControlsState,
    queue: Queue,
    pub w: i32,
    pub h: i32,
    pub settings: Settings,
}
impl Game {
    pub fn new() -> Self {
        let mut queue = Queue::new();
        let current_piece = CurrentPiece::new(queue.shift());
        Self {
            field: Field::new(),
            hold: Hold::new(),
            queue,
            current_piece,
            controls_state: ControlsState::new(),
            h: 0 as _,
            w: 0 as _,
            settings: Default::default(),
        }
    }

    pub fn init(&mut self, device_context: HDC) {
        unsafe {
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            glOrtho(0.0, 24.0, 24.0, 0.0, -1.0, 0.0);
            glMatrixMode(GL_MODELVIEW);
            glLoadIdentity();
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            SelectObject(device_context, GetStockObject(SYSTEM_FONT));
            let _ = wglUseFontBitmapsW(device_context, 0, 255, 1000);
        };
    }

    pub fn render(&self) {
        unsafe {
            let clear_color: *const f32 = Color::GRAY.into();
            let clear_color_slice = slice::from_raw_parts(clear_color, 3);
            glClearColor(
                clear_color_slice[0],
                clear_color_slice[1],
                clear_color_slice[2],
                1.0,
            );
            glClear(GL_COLOR_BUFFER_BIT);

            glPushMatrix();
            let w = self.w as f32;
            let h = self.h as f32;
            let screen_ratio = w / h;
            if screen_ratio > 1.0 {
                glTranslatef(24.0 / 2.0, 0.0, 0.0);
                glScalef(1.0 / screen_ratio, 1.0, 1.0);
                glTranslatef(-24.0 / 2.0, 0.0, 0.0);
            } else {
                glTranslatef(0.0, 24.0 / 2.0, 0.0);
                glScalef(1.0, screen_ratio, 1.0);
                glTranslatef(0.0, -24.0 / 2.0, 0.0);
            }

            glPushMatrix();
            glTranslatef(2.0, 1.0, 0.0);
            self.hold.render();
            glTranslatef(5.0, 0.0, 0.0);
            self.field.render();
            self.current_piece.render(&self.field, &self.settings);
            glTranslatef(11.0, 0.0, 0.0);
            self.queue.render();
            glPopMatrix();

            glPopMatrix();
        }
    }

    pub fn update(&mut self, diff: i64) -> bool {
        let das = self.settings.handling.das;
        if self.controls_state.left || self.controls_state.right {
            let mut freq = 0;
            let mut current = 0;
            unsafe {
                let _ = QueryPerformanceFrequency(&mut freq);
                let _ = QueryPerformanceCounter(&mut current);
            }
            if self.controls_state.left
                && (current - self.controls_state.left_counter) * 1000000 / freq > das * 1000
            {
                while self.current_piece.try_go_left(&self.field) {}
            }
            if self.controls_state.right
                && (current - self.controls_state.right_counter) * 1000000 / freq > das * 1000
            {
                while self.current_piece.try_go_right(&self.field) {}
            }
        }

        if !self.current_piece.update(diff, &self.field, &self.settings) {
            self.lock();
            return true;
        }
        return false;
    }

    pub fn try_hold(&mut self) {
        if self.hold.was_used {
            return;
        }
        if self.hold.piece.is_none() {
            self.hold.piece = Some(self.queue.shift());
        }
        let hold_piece = self.hold.piece.as_mut().unwrap();
        swap(hold_piece, &mut self.current_piece.piece);
        self.current_piece.x = 3;
        self.current_piece.y = 21;
        self.current_piece.r = 0;
        self.current_piece.since_last_move_down = 0;
        self.hold.was_used = true;
    }

    pub fn hard_drop(&mut self) {
        self.soft_drop();
        self.lock();
    }

    pub fn soft_drop(&mut self) {
        self.current_piece.soft_drop(&self.field);
    }

    pub fn lock(&mut self) {
        let color = self.current_piece.piece.get_color();
        let shape = self.current_piece.piece.get_shape(self.current_piece.r);
        for (i, row) in shape.iter().enumerate() {
            for (j, has_square) in row.iter().enumerate() {
                if *has_square {
                    let y_square = self.current_piece.y - i as i32;
                    let x_square = self.current_piece.x + j as i32;
                    self.field.buffer[x_square as usize][y_square as usize] = *color;
                }
            }
        }
        self.field.clear_full_lines();
        self.current_piece.set(self.queue.shift());
        self.hold.was_used = false;
        if self.controls_state.hold {
            self.try_hold();
        }
    }

    pub fn restart(&mut self) {
        self.hold.was_used = false;
        self.hold.piece = None;

        self.queue.reset();
        self.current_piece.set(self.queue.shift());

        for i in 0..10 {
            for j in 0..40 {
                self.field.buffer[i][j] = Color::BLACK;
            }
        }
    }

    pub fn get_state(&self) -> GameState {
        let mut field = [[Color::BLACK; 40]; 10];
        for i in 0..10 {
            for j in 0..40 {
                field[i][j] = self.field.buffer[i][j];
            }
        }
        let hold = if self.hold.piece.is_some() {
            Some(self.hold.piece.as_ref().unwrap().r#type)
        } else {
            None
        };

        let mut queue = Vec::new();

        for piece in &self.queue.buffer {
            queue.push(piece.r#type);
        }

        GameState {
            hold,
            field,
            current_piece: self.current_piece.piece.r#type,
            queue,
            rng_state: self.queue.rng.t,
        }
    }

    pub fn set_state(&mut self, state: GameState) {
        if state.hold.is_none() {
            self.hold.piece = None;
        } else {
            self.hold.piece = Some(Piece::new(state.hold.unwrap()));
        }

        self.current_piece.set(Piece::new(state.current_piece));

        for i in 0..10 {
            for j in 0..40 {
                self.field.buffer[i][j] = state.field[i][j];
            }
        }

        self.queue.buffer.clear();
        for piece_type in state.queue {
            self.queue.buffer.push_back(Piece::new(piece_type));
        }

        self.queue.rng.t = state.rng_state;
    }
}
