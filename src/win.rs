const CLASS_NAME: PCWSTR = w!("ttetris_class_name");

use std::mem;

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Graphics::{Gdi::HDC, OpenGL::glViewport},
    System::Performance::QueryPerformanceCounter,
    UI::{
        Input::{GetRawInputData, RAWINPUT, RAWINPUTHEADER, RID_INPUT, RIM_TYPEKEYBOARD},
        WindowsAndMessaging::{
            CW_USEDEFAULT, DefWindowProcW, DestroyWindow, GetWindowPlacement, KF_REPEAT, LoadIconW, PostQuitMessage, RI_KEY_BREAK, WINDOW_EX_STYLE, WINDOWPLACEMENT, WM_DESTROY, WM_INPUT, WM_KEYDOWN, WM_KEYUP, WM_SIZE
        },
    },
};
use windows::{
    Win32::{
        Foundation::HINSTANCE,
        Graphics::{
            Gdi::{GetDC, HBRUSH},
            OpenGL::{
                ChoosePixelFormat, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_SUPPORT_OPENGL,
                PFD_SWAP_LAYER_BUFFERS, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR, SetPixelFormat,
            },
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CreateWindowExW, HICON, IDC_ARROW, LoadCursorW,
            RegisterClassExW, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
        },
    },
    core::{PCWSTR, w},
};

use crate::APP_PTR;

unsafe extern "system" fn wnd_proc(wnd: HWND, msg: u32, p1: WPARAM, p2: LPARAM) -> LRESULT {
    unsafe {
        let app = &mut *APP_PTR;

        if msg == WM_DESTROY {
            // glDeleteContext
            if app.game.settings.appearance.save_last_window_placement {
                let mut wp = WINDOWPLACEMENT {
                    length: mem::size_of::<WINDOWPLACEMENT>() as _,
                    ..Default::default()
                };
                let _ = GetWindowPlacement(wnd, &mut wp as _);
                app.game.settings.appearance.window_placement = wp;
            }
            app.game.settings.save_to_file();
            PostQuitMessage(0);
            return LRESULT(0);
        }

        if msg == WM_KEYDOWN && (p2.0 as u32 / 0x10000) & KF_REPEAT != KF_REPEAT {
            if p1.0 == app.game.settings.controls.hard_drop as usize {
                app.game.hard_drop();
            }
            if p1.0 == app.game.settings.controls.soft_drop as usize {
                app.game.soft_drop();
            }
            if p1.0 == app.game.settings.controls.left as usize {
                app.game.current_piece.try_go_left(&app.game.field);
                app.game.controls_state.left = true;
                let _ = QueryPerformanceCounter(&mut app.game.controls_state.left_counter);
            }
            if p1.0 == app.game.settings.controls.right as usize {
                app.game.current_piece.try_go_right(&app.game.field);
                app.game.controls_state.right = true;
                let _ = QueryPerformanceCounter(&mut app.game.controls_state.right_counter);
            }
            if p1.0 == app.game.settings.controls.turn_180 as usize {
                app.game.current_piece.try_turn(&app.game.field, 2);
            }
            if p1.0 == app.game.settings.controls.turn_ccw as usize {
                app.game.current_piece.try_turn(&app.game.field, -1);
            }
            if p1.0 == app.game.settings.controls.turn_cw as usize {
                app.game.current_piece.try_turn(&app.game.field, 1);
            }
            if p1.0 == app.game.settings.controls.hold as usize {
                app.game.try_hold();
                app.game.controls_state.hold = true;
            }
            if p1.0 == app.game.settings.controls.restart as usize {
                app.game.restart();
            }
            if p1.0 == app.game.settings.controls.undo as usize {
                if app.history.len() > 0 {
                    app.game.set_state(app.history.pop_back().unwrap());
                }
            }
            if p1.0 == app.game.settings.controls.exit as usize {
                let _ = DestroyWindow(wnd);
            }
        }
        if msg == WM_KEYUP {
            if p1.0 == app.game.settings.controls.left as usize {
                app.game.controls_state.left = false;
                app.game.controls_state.left_counter = 0;
            }
            if p1.0 == app.game.settings.controls.right as usize {
                app.game.controls_state.right = false;
                app.game.controls_state.right_counter = 0;
            }
            if p1.0 == app.game.settings.controls.hold as usize {
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
                    if virtual_key == app.game.settings.controls.left {
                        app.game.controls_state.left = false;
                        app.game.controls_state.left_counter = 0;
                    }
                    if virtual_key == app.game.settings.controls.right {
                        app.game.controls_state.right = false;
                        app.game.controls_state.right_counter = 0;
                    }
                    if virtual_key == app.game.settings.controls.hold {
                        app.game.controls_state.hold = false;
                    }
                } else {
                    if virtual_key == app.game.settings.controls.hard_drop {
                        app.history.push_back(app.game.get_state());
                        app.game.hard_drop();
                    }
                    if virtual_key == app.game.settings.controls.soft_drop {
                        app.game.soft_drop();
                    }
                    if virtual_key == app.game.settings.controls.left
                        && !app.game.controls_state.left
                    {
                        app.game.current_piece.try_go_left(&app.game.field);
                        app.game.controls_state.left = true;
                        let _ = QueryPerformanceCounter(&mut app.game.controls_state.left_counter);
                    }
                    if virtual_key == app.game.settings.controls.right
                        && !app.game.controls_state.right
                    {
                        app.game.current_piece.try_go_right(&app.game.field);
                        app.game.controls_state.right = true;
                        let _ = QueryPerformanceCounter(&mut app.game.controls_state.right_counter);
                    }
                    if virtual_key == app.game.settings.controls.turn_180 {
                        app.game.current_piece.try_turn(&app.game.field, 2);
                    }
                    if virtual_key == app.game.settings.controls.turn_ccw {
                        app.game.current_piece.try_turn(&app.game.field, -1);
                    }
                    if virtual_key == app.game.settings.controls.turn_cw {
                        app.game.current_piece.try_turn(&app.game.field, 1);
                    }
                    if virtual_key == app.game.settings.controls.hold
                        && !app.game.controls_state.hold
                    {
                        app.game.try_hold();
                        app.game.controls_state.hold = true;
                    }
                    if virtual_key == app.game.settings.controls.restart {
                        app.history.push_back(app.game.get_state());
                        app.game.restart();
                    }
                    if virtual_key == app.game.settings.controls.undo {
                        if app.history.len() > 0 {
                            app.game.set_state(app.history.pop_back().unwrap());
                        }
                    }
                    if virtual_key == app.game.settings.controls.exit {
                        let _ = DestroyWindow(wnd);
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

pub fn create_window() -> (HDC, HWND) {
    unsafe {
        let instance = HINSTANCE::from(GetModuleHandleW(None).unwrap());
        let window_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as _,
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: LoadIconW(Some(instance), PCWSTR::from_raw(1 as _)).unwrap(),
            hCursor: LoadCursorW(None as _, IDC_ARROW).unwrap(),
            hbrBackground: HBRUSH::default(),
            lpszMenuName: CLASS_NAME,
            lpszClassName: CLASS_NAME,
            hIconSm: HICON::default(),
        };
        RegisterClassExW(&window_class);
        let wnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            CLASS_NAME,
            w!("ttetris"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
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

        return (device_context, wnd);
    }
}
