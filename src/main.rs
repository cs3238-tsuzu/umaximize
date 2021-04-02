mod win;

use bindings::Windows::{
    Win32::WindowsAndMessaging::*,
    Win32::SystemServices::{
        BOOL,
    }
};

use win::{
    Point,
    Window,
    Rect,
};

use std::cmp;

const EXPECTED_POS: Point = Point{x: 0, y: 0};

fn handler_impl(hwnd: HWND, _lparam: LPARAM) {
    let wnd = Window::new(hwnd);

    if wnd.get_title().unwrap_or_default() != "umamusume" {
        return
    }

    let client_rect = match wnd.get_client_rect() {
        Some(rect) => rect,
        None => return
    };
    let rect = match wnd.get_rect() {
        Some(rect) => rect,
        None => return
    };

    let screen_size = Point::scren_size();

    if wnd.has_title_bar() {
        println!("hiding");
        wnd.hide_title_bar()
    }

    println!("{:?} {:?}", rect, screen_size);

    let ratio = if client_rect.size.x < client_rect.size.y {
        Point {x: 9, y: 16}
    } else {
        Point {x: 16, y: 9}
    };

    let expected_size = Point {
        x: cmp::min(screen_size.x, screen_size.y * ratio.x / ratio.y),
        y: cmp::min(screen_size.y, screen_size.x * ratio.y / ratio.x),
    };

    println!("{:?}", expected_size);

    if rect.size != expected_size || rect.pos != EXPECTED_POS {
        wnd.resize(Rect {
            pos: EXPECTED_POS,
            size: expected_size,
        })
    }

    

    // if wnd.has_title_bar() {
    //     wnd.hide_title_bar()
    // }
}

extern "system"
fn handler(hwnd: HWND, lparam: LPARAM) -> BOOL {
    handler_impl(hwnd, lparam);

    BOOL::from(true)
}

fn main() {
    // let mut rect = RECT::default();

    unsafe {
        EnumWindows(Some(handler), LPARAM::default());

        // let hwnd = FindWindowA(PSTR::default(), "umamusume");

        // assert!(GetWindowRect(hwnd, &mut rect).as_bool());
    }

    // println!("{:?}", rect);
    println!("Hello, world!");  
}
