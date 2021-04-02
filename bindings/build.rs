fn main() {
    windows::build!(
        Windows::Win32::WindowsAndMessaging::*,
        Windows::Win32::DisplayDevices::RECT,
        Windows::Win32::SystemServices::{
            PSTR, BOOL,
        },
        Windows::Win32::Debug::GetLastError,
    );
}
