#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gltest::win32::{
    self, 
    window,
};

#[allow(non_snake_case)]
fn main() {
    let hInstance = unsafe { win32::core::GetModuleHandleW(std::ptr::null()) };
    let sample_window_class_wn = win32::utils::wide_null("Test Window");

    let mut win: window::WNDCLASSW = window::WNDCLASSW::default();
    win.lpfnWndProc = Some(window::window_procedure);
    win.hInstance = hInstance;
    win.lpszClassName = sample_window_class_wn.as_ptr();
    win.hCursor = unsafe { window::LoadCursorW(std::ptr::null_mut(), window::IDC_ARROW)};

    let atom = unsafe { window::RegisterClassW(&win) };
    if atom == 0 {
        let last_error = unsafe { win32::core::GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }

    let lparam: *mut i32 = Box::leak(Box::new(5_i32)); // make this leak so that we can clean it up on window destruction rather than scope
    let hwnd = unsafe {
        window::CreateWindowExW(
            0, 
            sample_window_class_wn.as_ptr(), 
            sample_window_class_wn.as_ptr(), 
            window::WS_OVERLAPPEDWINDOW, 
            window::CW_USEDEFAULT, 
            window::CW_USEDEFAULT, 
            window::CW_USEDEFAULT, 
            window::CW_USEDEFAULT, 
            std::ptr::null_mut(), 
            std::ptr::null_mut(), 
            hInstance, 
            lparam.cast()
        )
    };
    if hwnd.is_null() {
        panic!("Failed to create window");
    }

    let _previously_visible = unsafe { window::ShowWindow(hwnd, window::SW_SHOW) };

    let mut msg = window::MSG::default();
    loop {
        let message_return = unsafe { window::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) };

        if message_return == 0 {
            break;
        } else if message_return == -1 {
            let last_error = unsafe { win32::core::GetLastError() };
            panic!("Error with win32::core::GetMessageW. Error code: {}", last_error);
        } else {
            unsafe {
                window::TranslateMessage(&msg);
                window::DispatchMessageW(&msg);
            }
        }
    }


    println!("RUNS");

    return;
}
