#![windows_subsystem = "windows"]

mod game;
mod settings;

use std::ptr;
use windows::{
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::{
            Gdi::{GetDC, HBRUSH, HDC, UpdateWindow, WGL_SWAP_MAIN_PLANE},
            OpenGL::{
                ChoosePixelFormat, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_SUPPORT_OPENGL,
                PFD_SWAP_LAYER_BUFFERS, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR, SetPixelFormat,
                glViewport, wglCreateContext, wglMakeCurrent, wglSwapLayerBuffers,
            },
        },
        System::{
            LibraryLoader::GetModuleHandleW,
            Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
        },
        UI::{
            Input::{
                GetRawInputData, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RID_INPUT,
                RIDEV_NOLEGACY, RIM_TYPEKEYBOARD, RegisterRawInputDevices,
            },
            WindowsAndMessaging::{
                CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CreateWindowExW, DefWindowProcW,
                DispatchMessageW, GetMessageW, HICON, IDC_ARROW, KF_REPEAT, LoadCursorW, MSG,
                PM_NOREMOVE, PeekMessageW, PostQuitMessage, RI_KEY_BREAK, RegisterClassExW,
                SW_NORMAL, ShowWindow, WM_DESTROY, WM_INPUT, WM_KEYDOWN, WM_KEYUP, WM_SIZE,
                WNDCLASSEXW, WS_EX_OVERLAPPEDWINDOW, WS_OVERLAPPEDWINDOW,
            },
        },
    },
    core::{PCWSTR, w},
};

use crate::game::Game;

const CLASS_NAME: PCWSTR = w!("ttetris_class_name");

const INIT_WIDTH: i32 = 850;
const INIT_HEIGHT: i32 = 600;

struct App {
    game: Game,
    device_context: HDC,
}

static mut APP_PTR: *mut App = ptr::null_mut();

impl App {
    pub fn new() -> Self {
        Self {
            game: Game::new(INIT_WIDTH, INIT_HEIGHT),
            device_context: HDC::default(),
        }
    }
    unsafe extern "system" fn wnd_proc(wnd: HWND, msg: u32, p1: WPARAM, p2: LPARAM) -> LRESULT {
        unsafe {
            if msg == WM_DESTROY {
                // glDeleteContext
                PostQuitMessage(0);
                return LRESULT(0);
            }
            let app = &mut *APP_PTR;

            if msg == WM_KEYDOWN && (p2.0 as u32 / 0x10000) & KF_REPEAT != KF_REPEAT {
                if p1.0 == settings::KEY_HARD_DROP.0 as usize {
                    app.game.hard_drop();
                }
                if p1.0 == settings::KEY_SOFT_DROP.0 as usize {
                    app.game.soft_drop();
                }
                if p1.0 == settings::KEY_LEFT.0 as usize {
                    app.game.current_piece.try_go_left(&app.game.field);
                    app.game.controls_state.left = true;
                    let _ = QueryPerformanceCounter(&mut app.game.controls_state.left_counter);
                }
                if p1.0 == settings::KEY_RIGHT.0 as usize {
                    app.game.current_piece.try_go_right(&app.game.field);
                    app.game.controls_state.right = true;
                    let _ = QueryPerformanceCounter(&mut app.game.controls_state.right_counter);
                }
                if p1.0 == settings::KEY_TURN_180.0 as usize {
                    app.game.current_piece.try_turn(&app.game.field, 2);
                }
                if p1.0 == settings::KEY_TURN_CCW.0 as usize {
                    app.game.current_piece.try_turn(&app.game.field, -1);
                }
                if p1.0 == settings::KEY_TURN_CW.0 as usize {
                    app.game.current_piece.try_turn(&app.game.field, 1);
                }
                if p1.0 == settings::KEY_HOLD.0 as usize {
                    app.game.try_hold();
                    app.game.controls_state.hold = true;
                }
                if p1.0 == settings::KEY_RESTART.0 as usize {
                    app.game.restart();
                }
            }
            if msg == WM_KEYUP {
                if p1.0 == settings::KEY_LEFT.0 as usize {
                    app.game.controls_state.left = false;
                    app.game.controls_state.left_counter = 0;
                }
                if p1.0 == settings::KEY_RIGHT.0 as usize {
                    app.game.controls_state.right = false;
                    app.game.controls_state.right_counter = 0;
                }
                if p1.0 == settings::KEY_HOLD.0 as usize {
                    app.game.controls_state.hold = false;
                }
            }
            if msg == WM_INPUT {
                let mut buffer: [RAWINPUT; 64] = [Default::default(); 64];
                let mut buffer_size = (std::mem::size_of::<RAWINPUT>() * 64) as u32;
                GetRawInputData(
                    std::mem::transmute(p2),
                    RID_INPUT,
                    Some(buffer.as_mut_ptr() as _),
                    &mut buffer_size,
                    std::mem::size_of::<RAWINPUTHEADER>() as _,
                );
                if buffer[0].header.dwType == RIM_TYPEKEYBOARD.0 as _ {
                    let kb = buffer[0].data.keyboard;
                    let virtual_key = kb.VKey;
                    if kb.Flags as u32 & RI_KEY_BREAK == RI_KEY_BREAK {
                        if virtual_key == settings::KEY_LEFT.0 {
                            app.game.controls_state.left = false;
                            app.game.controls_state.left_counter = 0;
                        }
                        if virtual_key == settings::KEY_RIGHT.0 {
                            app.game.controls_state.right = false;
                            app.game.controls_state.right_counter = 0;
                        }
                        if virtual_key == settings::KEY_HOLD.0 {
                            app.game.controls_state.hold = false;
                        }
                    } else {
                        if virtual_key == settings::KEY_HARD_DROP.0 {
                            app.game.hard_drop();
                        }
                        if virtual_key == settings::KEY_SOFT_DROP.0 {
                            app.game.soft_drop();
                        }
                        if virtual_key == settings::KEY_LEFT.0 {
                            app.game.current_piece.try_go_left(&app.game.field);
                            app.game.controls_state.left = true;
                            let _ =
                                QueryPerformanceCounter(&mut app.game.controls_state.left_counter);
                        }
                        if virtual_key == settings::KEY_RIGHT.0 {
                            app.game.current_piece.try_go_right(&app.game.field);
                            app.game.controls_state.right = true;
                            let _ =
                                QueryPerformanceCounter(&mut app.game.controls_state.right_counter);
                        }
                        if virtual_key == settings::KEY_TURN_180.0 {
                            app.game.current_piece.try_turn(&app.game.field, 2);
                        }
                        if virtual_key == settings::KEY_TURN_CCW.0 {
                            app.game.current_piece.try_turn(&app.game.field, -1);
                        }
                        if virtual_key == settings::KEY_TURN_CW.0 {
                            app.game.current_piece.try_turn(&app.game.field, 1);
                        }
                        if virtual_key == settings::KEY_HOLD.0 {
                            app.game.try_hold();
                            app.game.controls_state.hold = true;
                        }
                        if virtual_key == settings::KEY_RESTART.0 {
                            app.game.restart();
                        }
                    }
                }
            }
            if msg == WM_SIZE {
                let w = (p2.0 & 0xffff) as _;
                let h = ((p2.0 >> 16) & 0xffff) as _;
                app.game.w = w as _;
                app.game.h = h as _;
                glViewport(0, 0, w, h);
            }
            return DefWindowProcW(wnd, msg, p1, p2);
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let instance = HINSTANCE::from(GetModuleHandleW(None).unwrap());
            let window_class = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as _,
                style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(App::wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: instance,
                hIcon: HICON::default(),
                hCursor: LoadCursorW(None as _, IDC_ARROW).unwrap(),
                hbrBackground: HBRUSH::default(),
                lpszMenuName: CLASS_NAME,
                lpszClassName: CLASS_NAME,
                hIconSm: HICON::default(),
            };
            RegisterClassExW(&window_class);
            let wnd = CreateWindowExW(
                WS_EX_OVERLAPPEDWINDOW,
                CLASS_NAME,
                w!("ttetris"),
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                INIT_WIDTH,
                INIT_HEIGHT,
                None,
                None,
                Some(instance),
                None,
            )
            .unwrap();

            let device_context = GetDC(Some(wnd));
            let pixel_format = PIXELFORMATDESCRIPTOR {
                nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW
                    | PFD_SUPPORT_OPENGL
                    | PFD_DOUBLEBUFFER
                    | PFD_SWAP_LAYER_BUFFERS,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cStencilBits: 24,
                cDepthBits: 8,
                iLayerType: 0,
                ..Default::default()
            };
            let chosen_format = ChoosePixelFormat(device_context, &pixel_format);
            let _ = SetPixelFormat(device_context, chosen_format, &pixel_format);

            let gl_context = wglCreateContext(device_context).unwrap();
            let _ = wglMakeCurrent(device_context, gl_context);

            let _ = ShowWindow(wnd, SW_NORMAL);
            let _ = UpdateWindow(wnd);

            self.device_context = device_context;
            self.game.init(self.device_context);

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
                self.game.update(diff);
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
