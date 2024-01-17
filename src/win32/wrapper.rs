use crate::win32::*;

use self::window::WNDCLASSW;

/// Abstraction to represent an error
#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Error(pub types::DWORD);

impl std::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dwFlags = todo!();
        let lpSource = todo!();
        let dwMessageId = todo!();
        let dwLanguageId = todo!();
        let lpBuffer = todo!();
        let nSize = todo!();
        let Arguments = todo!();
        let dword = unsafe {
            core::FormatMessageW(dwFlags, lpSource, dwMessageId, dwLanguageId, lpBuffer, nSize, Arguments)
        };
        todo!("Call FormatMessageW()");
    }
}


/// Returns a handle to the file used to create the calling process (.exe file)
///
/// See [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
pub fn get_process_handle() -> types::HMODULE {
    // Safety: as per the MSDN documentation
    unsafe { core::GetModuleHandleW(std::ptr::null()) }
}

/// Load one of our predefined cursors
/// 
/// See [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
pub fn load_predefined_cursor(cursor: window::IDCursor) -> Result<types::HCURSOR, ()> {
    // Safety: The enum only allows values from valid list of cursors from MSDN
    let hcursor = unsafe {
        window::LoadCursorW(std::ptr::null_mut(), core::MAKEINTRESOURCEW(cursor as types::WORD))
    };

    if hcursor.is_null() {
        return Err(());
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
pub unsafe fn register_class(window_class: &WNDCLASSW) -> Result<types::ATOM, ()> {
    let atom = window::RegisterClassW(window_class);
    if atom == 0 {
        return Err(());
    } else {
        return Ok(atom);
    }
}

/// Gets the thread-local last error code value
/// 
/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> types::DWORD {
    return unsafe {
        core::GetLastError()
    };
}