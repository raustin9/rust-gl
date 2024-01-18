#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gltest::win32::{
    self, 
    window,
};

#[allow(non_snake_case)]
fn main() {
    let hInstance = win32::wrapper::get_process_handle();
    let sample_window_class = "Sample Window Class";
    let sample_window_class_wn = win32::utils::wide_null(sample_window_class);

    let mut win: window::WNDCLASSW = window::WNDCLASSW::default();
    win.lpfnWndProc = Some(window_procedure);
    win.hInstance = hInstance;
    win.lpszClassName = sample_window_class_wn.as_ptr();
    win.hCursor = win32::wrapper::load_predefined_cursor(window::IDCursor::Arrow).unwrap();

    let _atom = unsafe { win32::wrapper::register_class(&win)}.unwrap();

    let lparam: *mut i32 = Box::leak(Box::new(5_i32)); // make this leak so that we can clean it up on window destruction rather than scope
    let hwnd = unsafe { win32::wrapper::create_app_window(
        sample_window_class, 
        "Test Window", 
        None,
        [800, 600], 
        lparam.cast(),
    )}.unwrap();

    let _previously_visible = unsafe { window::ShowWindow(hwnd, window::SW_SHOW) };

    // let mut msg = window::MSG::default();
    loop {
        match win32::wrapper::get_any_message() {
            Ok(msg) => {
                if msg.message == window::WM_QUIT {
                    std::process::exit(msg.wParam as i32);
                }
                win32::wrapper::translate_message(&msg);
                unsafe {
                    // window::TranslateMessage(&msg);
                    window::DispatchMessageW(&msg);
                }
            }
            Err(e) => panic!("Error when getting msg from message queue: {}", e),
        }
        // let message_return = unsafe { window::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) };

        // if message_return == 0 {
        //     break;
        // } else if message_return == -1 {
        //     let last_error = unsafe { win32::core::GetLastError() };
        //     panic!("Error with win32::core::GetMessageW. Error code: {}", last_error);
        // } else {
        //     unsafe {
        //         window::TranslateMessage(&msg);
        //         window::DispatchMessageW(&msg);
        //     }
        // }
    }
}

#[allow(non_snake_case)]
pub unsafe extern "system" fn window_procedure(
    hWnd: win32::types::HWND,
    Msg: win32::types::UINT,
    wParam: win32::types::WPARAM,
    lParam: win32::types::LPARAM,
) -> win32::types::LRESULT {
    match Msg {
        // TODO: Set the title of the window in one of the creation events
        window::WM_NCCREATE => {
            println!("NC Create");
            let createstruct: *mut window::CREATESTRUCTW = lParam as *mut _;
            if createstruct.is_null() {
                return 0;
            }
            let ptr = (*createstruct).lpCreateParams as *mut i32;
            return win32::wrapper::set_window_userdata(hWnd, ptr).is_ok() as win32::types::LRESULT;
        }
        window::WM_CREATE => println!("CREATE"),
        window::WM_CLOSE => drop(window::DestroyWindow(hWnd)),
        window::WM_DESTROY => {
            match win32::wrapper::get_window_userdata::<i32>(hWnd) {
                Ok(ptr) if !ptr.is_null() => {
                    let _dropped = Box::from_raw(ptr);
                    println!("Cleaned up the box");
                }
                Ok(_) => {
                    println!("userdata ptr is null, no clenaup");
                }
                Err(e) => {
                    println!("Error while getting the userdata ptr to clean up: {}", e);
                }
            }
            window::PostQuitMessage(0);
            // let ptr = window::GetWindowLongPtrW(hWnd, window::GWLP_USERDATA) as *mut i32;
            // let _dropped = Box::from_raw(ptr);
            // println!("Cleanup Window");
            // window::PostQuitMessage(0_i32);
        }
        window::WM_PAINT => {
            match win32::wrapper::get_window_userdata::<i32>(hWnd) {
                Ok(ptr) if !ptr.is_null()  => {
                    println!("Current ptr: {}", *ptr);
                    *ptr += 1;
                }
                Ok(_) => {
                    println!("userdata pointer is null");
                }
                Err(e) => {
                    println!("Error while getting userdata pointer: {}", e);
                }
            }
            win32::wrapper::paint_window(hWnd, |hdc, _erase_bg, target_rect| {
                let _ = win32::wrapper::fill_rect_with_sys_color(hdc, &target_rect, window::SysColor::Window);
                return Ok(());
            }).unwrap_or_else(|e| println!("error during painting {}", e));
            // match win32::wrapper::begin_paint(hWnd) {
            //     Ok((hdc, ps)) => {
            //         let _ = win32::wrapper::fill_rect_with_sys_color(hdc, &ps.rcPaint, window::SysColor::Window);
            //         win32::wrapper::end_paint(hWnd, &ps);
            //     }
            //     Err(e) => {
            //         println!("Couldn't begin painting: {}", e);
            //     }
            // }
        }
        // WM_SETCURSOR => {
        //     let hInstance = GetModuleHandleW(std::ptr::null());
        //     let cursor = LoadCursorW(hInstance, IDC_ARROW);
        //     let _old_cursor = SetCursor(cursor);
        //     return 1;
        // }
        _ => {
            return window::DefWindowProcW(
                hWnd, 
                Msg, 
                wParam, 
                lParam
            );
        }
    }

    return 0;
}