use gltest::win32::{
    self, 
    window,
};

#[allow(non_snake_case)]
fn main() {
    let hInstance = unsafe { win32::core::GetModuleHandleW(std::ptr::null()) };
    let sample_window_class_wn = win32::utils::wide_null("Sample Window Class");

    let mut win: window::WNDCLASSW = window::WNDCLASSW::default();
    win.lpfnWndProc = Some(window::DefWindowProcW);
    win.hInstance = hInstance;
    win.lpszClassName = sample_window_class_wn.as_ptr();
    win.hCursor = unsafe { window::LoadCursorW(hInstance, window::IDC_ARROW)};

    let atom = unsafe { window::RegisterClassW(&win) };
    if atom == 0 {
        let last_error = unsafe { win32::core::GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }

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
            std::ptr::null_mut()
        )
    };
    if hwnd.is_null() {
        panic!("Failed to create window");
    }

    let _previously_visible = unsafe { window::ShowWindow(hwnd, window::SW_SHOW) };

    let mut msg = window::MSG::default();
    loop {
        let mut lpMsg = msg.clone() as window::LPMSG;
        let message_return = unsafe { window::GetMessageW(lpMsg, std::ptr::null_mut(), 0, 0) };
        // let message_return = unsafe { window::GetMessageW(&mut lpMsg, std::ptr::null_mut(), 0, 0) };

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
