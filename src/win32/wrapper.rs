use crate::win32::*;

use self::{window::{WNDCLASSW, CreateWindowExW, CW_USEDEFAULT}, core::GetLastError, types, utils::wide_null};

/// Abstraction to represent an error
#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Error(pub types::DWORD);

#[allow(nonstandard_style)]
impl std::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // If the 29th bit is set in the code, then it is an application
        // error, and it cannot be formatted so we return early here
        if self.0 & (1 << 29) > 0 {
            return write!(f, "Win32ApplicationError({})", self.0);
        }

        pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: types::DWORD = 0x0000_0100;
        pub const FORMAT_MESSAGE_FROM_SYSTEM: types::DWORD = 0x0000_1000;
        pub const FORMAT_MESSAGE_IGNORE_INSERTS: types::DWORD = 0x0000_0200;

        let dwFlags = 
            FORMAT_MESSAGE_ALLOCATE_BUFFER
            | FORMAT_MESSAGE_FROM_SYSTEM
            | FORMAT_MESSAGE_IGNORE_INSERTS;
        let lpSource = std::ptr::null_mut();
        let dwMessageId = self.0;
        let dwLanguageId = 0;
        let mut buffer: *mut u16 = std::ptr::null_mut();
        let lpBuffer = &mut buffer as *mut *mut u16 as *mut u16;
        let nSize = 0;
        let Arguments = std::ptr::null_mut();
        let tchar_count_excluding_null = unsafe {
            core::FormatMessageW(dwFlags, lpSource, dwMessageId, dwLanguageId, lpBuffer, nSize, Arguments)
        };

        if tchar_count_excluding_null == 0 || buffer.is_null() {
            // some problem happened. We cannot usefully get_last_error 
            // since display formatting doesn't let you give an error value
            return Err(std::fmt::Error);
        } else {
            // For freeing the memory allocated 
            // since we used FORMAT_MESSAGE_ALLOCATE_BUFFER
            struct OnDropLocalFree(types::HLOCAL);
            impl Drop for OnDropLocalFree {
                fn drop(&mut self) {
                    unsafe {
                        core::LocalFree(self.0)
                    };
                }
            }
            let _on_drop = OnDropLocalFree(buffer as types::HLOCAL); // cannot bind this to the "_" special variable because it would drop immediately 
                                                                                      // before we ever read the buffer. We bind to a local var because it will call 
                                                                                      // our drop implementation to free the buffer at the end of scope
            let buffer_slice: &[u16] = unsafe {
                std::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize)
            };

            for decode_result in 
                std::char::decode_utf16(buffer_slice.iter().copied())
            {
                match decode_result {
                    Ok('\r') | Ok('\n') => write!(f, " ")?, // eat the newlines
                    Ok(ch) => write!(f, "{}", ch)?,   // print the code
                    Err(_) => write!(f, "�")?,             // if unknown char print this default 
                }
                
            }
            todo!();
        }
    }
}

impl std::error::Error for Win32Error {}


/// Returns a handle to the file used to create the calling process (.exe file)
///
/// See [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
pub fn get_process_handle() -> types::HINSTANCE {
    // Safety: as per the MSDN documentation
    unsafe { core::GetModuleHandleW(std::ptr::null()) }
}

/// Load one of our predefined cursors
/// 
/// See [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
pub fn load_predefined_cursor(cursor: window::IDCursor) -> Result<types::HCURSOR, Win32Error> {
    // Safety: The enum only allows values from valid list of cursors from MSDN
    let hcursor = unsafe {
        window::LoadCursorW(std::ptr::null_mut(), core::MAKEINTRESOURCEW(cursor as types::WORD))
    };

    if hcursor.is_null() {
        return Err(get_last_error());
    } else {
        return Ok(hcursor);
    }
}

/// Registers a window class struct
/// 
/// ## Safety
/// 
/// All pointer fields of the struct must be valid
/// 
/// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
pub unsafe fn register_class(window_class: &WNDCLASSW) -> Result<types::ATOM, Win32Error> {
    let atom = window::RegisterClassW(window_class);
    if atom == 0 {
        return Err(get_last_error());
    } else {
        return Ok(atom);
    }
}

/// Gets the thread-local last error code value
/// 
/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> Win32Error {
    return Win32Error(unsafe { GetLastError() });
}

/// Creates a window
/// 
/// See [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
pub unsafe fn create_window_ex_w(
    ex_style: types::DWORD,
    class_name: types::LPCWSTR,
    window_name: types::LPCWSTR,
    style: types::DWORD,
    x: types::c_int,
    y: types::c_int,
    width: types::c_int,
    height: types::c_int,
    parent: types::HWND,
    menu: types::HMENU,
    instance: types::HINSTANCE,
    param: types::LPVOID,
) -> Result<types::HWND, Win32Error> {
    let hwnd = CreateWindowExW(
        ex_style, 
        class_name, 
        window_name,
        style, 
        x, 
        y, 
        width,
        height, 
        parent,
        menu,
        instance,
        param
        );

        if hwnd.is_null() {
            return Err(get_last_error());
        } else {
            return Ok(hwnd);
        }
}

/// Create an app window with default styling.
/// If you want to customize this, you can call
/// the function win32::wrapper::create_window_ex_w(...)
/// using the parameters you prefer.
pub unsafe fn create_app_window(
    class_name: &str,
    window_name: &str,
    position: Option<[i32; 2]>,
    [width, height]: [i32; 2],
    create_param: types::LPVOID,
) -> Result<types::HWND, Win32Error> {
    let class_name_null = wide_null(class_name);
    let window_name_null = wide_null(window_name);

    let (x, y) = match position {
        Some([x, y]) => (x, y),
        None => (CW_USEDEFAULT, CW_USEDEFAULT),
    };

    let hwnd = CreateWindowExW(
        window::WS_EX_APPWINDOW | window::WS_EX_OVERLAPPEDWINDOW,
        class_name_null.as_ptr(),
        window_name_null.as_ptr(),
        window::WS_OVERLAPPEDWINDOW | window::WS_CLIPCHILDREN | window::WS_CLIPSIBLINGS,
        x,
        y,
        width,
        height,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        get_process_handle(),
        create_param
    );

    if hwnd.is_null() {
        return Err(get_last_error());
    } else {
        return Ok(hwnd);
    }
}

/// Gets a message from the thread's message queue.
/// 
/// The message can be for any window from this thread,
/// or it can be a non-window message.
/// 
/// See [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
#[inline(always)]
pub fn get_any_message() -> Result<window::MSG, Win32Error> {
    let mut msg = window::MSG::default();
    let output = unsafe { window::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) };
    if output == -1 {
        return Err(get_last_error());
    } else {
        return Ok(msg);
    }
}

