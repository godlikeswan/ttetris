// To show test report (see https://github.com/rust-lang/rust/issues/67159)
#![cfg_attr(not(test), windows_subsystem = "windows")]

mod game;
mod settings;
mod win;

use std::{collections::VecDeque, mem::transmute, ptr};
use windows::core::s;
use windows::Win32::{
    Foundation::HWND,
    Graphics::{
        Gdi::{HDC, WGL_SWAP_MAIN_PLANE},
        OpenGL::{wglCreateContext, wglGetProcAddress, wglMakeCurrent, wglSwapLayerBuffers},
    },
    System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
    UI::{
        Input::{RAWINPUTDEVICE, RIDEV_NOLEGACY, RegisterRawInputDevices},
        WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, MSG, PM_NOREMOVE, PeekMessageW, SetWindowPlacement,
        },
    },
};

use crate::{
    game::{Game, state::GameState},
    settings::Settings,
    win::create_window,
};

struct App {
    game: Game,
    device_context: HDC,
    history: VecDeque<GameState>,
}

static mut APP_PTR: *mut App = ptr::null_mut();

impl App {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            device_context: HDC::default(),
            history: Default::default(),
        }
    }

    pub fn init(&mut self) {
        let settings_from_file = Settings::load_from_file();
        if settings_from_file.is_some() {
            self.game.settings = settings_from_file.unwrap();
        }
        let (device_context, wnd) = create_window();

        unsafe {
            let gl_context = wglCreateContext(device_context).unwrap();
            let _ = wglMakeCurrent(device_context, gl_context);

            if self.game.settings.appearance.vsync {
                let proc = wglGetProcAddress(s!("wglSwapIntervalEXT")).unwrap();
                let wgl_swap_interval_ext: extern "system" fn(i32) -> i32 =
                    transmute(proc);
                wgl_swap_interval_ext(1);
            }

            self.device_context = device_context;
            self.game.init(self.device_context);
            // self.history.push_back(self.game.get_state());

            let _ = SetWindowPlacement(
                wnd,
                &mut self.game.settings.appearance.window_placement as _,
            );

            let input_devices = [RAWINPUTDEVICE {
                usUsagePage: 0x01,
                usUsage: 0x06,
                dwFlags: RIDEV_NOLEGACY,
                hwndTarget: HWND(ptr::null_mut()),
            }];
            let _ =
                RegisterRawInputDevices(&input_devices, std::mem::size_of::<RAWINPUTDEVICE>() as _);
        }
    }
    pub fn run(&mut self) {
        unsafe {
            let mut freq = 0;
            let _ = QueryPerformanceFrequency(&mut freq);
            let mut start = 0;
            let _ = QueryPerformanceCounter(&mut start);
            let mut stop = 0;
            loop {
                self.game.render();
                let _ = wglSwapLayerBuffers(self.device_context, WGL_SWAP_MAIN_PLANE);

                let mut msg = MSG::default();
                while PeekMessageW(&mut msg, None, 0, 0, PM_NOREMOVE) == true {
                    if GetMessageW(&mut msg, None, 0, 0) == true {
                        DispatchMessageW(&msg);
                    } else {
                        return;
                    }
                }
                let _ = QueryPerformanceCounter(&mut stop);
                let diff = (stop - start) * 1000000 / freq;
                let game_state = self.game.get_state();
                let locked = self.game.update(diff);
                if locked {
                    self.history.push_back(game_state);
                }
                start = stop;
            }
        }
    }
}
fn main() {
    let mut app = App::new();
    unsafe {
        APP_PTR = &mut app;
    }
    app.init();
    app.run();
}
