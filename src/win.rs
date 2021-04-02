use bindings::Windows::{
    Win32::WindowsAndMessaging::*,
    Win32::DisplayDevices::RECT,
    Win32::SystemServices::{
        PWSTR,
    },
};

use std::char::{decode_utf16, REPLACEMENT_CHARACTER};

fn decode(source: &[u16]) -> String {
    decode_utf16(source.iter().cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn scren_size() -> Point {
        unsafe {
            Point {
                x: GetSystemMetrics(GetSystemMetrics_nIndexFlags::SM_CXSCREEN),
                y: GetSystemMetrics(GetSystemMetrics_nIndexFlags::SM_CYSCREEN),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub pos: Point,
    pub size: Point,
}

impl Rect {
    pub fn from_win_rect(rect: RECT) -> Rect {
        Rect {
            pos: Point {
                x: rect.left,
                y: rect.top,
            },
            size: Point {
                x: rect.right - rect.left,
                y: rect.bottom - rect.top,
            }
        }
    }
}


#[derive(Debug)]
pub struct Window {
    pub hwnd: HWND,
}

impl Window {
    pub fn new(hwnd: HWND) -> Self {
        Window{
            hwnd,
        }
    }

    pub fn get_rect(&self) -> Option<Rect> {
        let mut rect = RECT::default();
        unsafe {
            if !GetWindowRect(self.hwnd, &mut rect).as_bool() {
                return None;
            }
        }

        Some(Rect::from_win_rect(rect))
    }

    pub fn get_client_rect(&self) -> Option<Rect> {
        let mut rect = RECT::default();
        unsafe {
            if !GetClientRect(self.hwnd, &mut rect).as_bool() {
                return None;
            }
        }

        Some(Rect::from_win_rect(rect))
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            let len = GetWindowTextLengthW(self.hwnd);
        
            if len != 0 {
                let mut buf = vec![0u16; (len + 1) as usize];
                let buf_ptr = buf.as_mut_ptr();
    
                GetWindowTextW(self.hwnd, PWSTR{0: buf_ptr}, len + 1);
    
                return Some(decode(&buf[..len as usize]));
            }
        }
    
        None
    }

    pub fn has_title_bar(&self) -> bool {
        unsafe {
            GetWindowLongA(self.hwnd, WINDOW_LONG_PTR_INDEX::GWL_STYLE) & 0xce0000 != 0
        }
    }

    pub fn hide_title_bar(&self) {
        unsafe {
            let style = GetWindowLongA(self.hwnd, WINDOW_LONG_PTR_INDEX::GWL_STYLE) & !0xce0000;
            println!("{:x}", style);
            
            println!("{:?}", SetWindowLongA(self.hwnd, WINDOW_LONG_PTR_INDEX::GWL_STYLE, style));

            // SetWindowPos(self.hwnd, HWND_TOP, 0, 0, 0, 0, SetWindowPos_uFlags::SWP_NOMOVE|SetWindowPos_uFlags::SWP_NOSIZE|SetWindowPos_uFlags::SWP_NOZORDER | SetWindowPos_uFlags::SWP_NOACTIVATE|SetWindowPos_uFlags::SWP_FRAMECHANGED);
        }
    }

    pub fn resize(&self, rect: Rect) {
        unsafe {
            let hdwp = BeginDeferWindowPos(1);
        
            DeferWindowPos(
                hdwp,
                self.hwnd,
                HWND_TOP,
                rect.pos.x,
                rect.pos.y,
                rect.size.x,
                rect.size.y,
                 SetWindowPos_uFlags::SWP_SHOWWINDOW,
            );

            EndDeferWindowPos(hdwp);
        }
    }
}

