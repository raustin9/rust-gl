// use core::panic;

// use std::ptr::{
//     null,
//     null_mut,
// };
use crate::win32::types::*;
use crate::win32::core::*;


macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}

// WINDOW TYPES //
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[repr(C)]
pub struct POINT {
    x: LONG,
    y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

pub type WNDPROC = Option<
    unsafe extern "system" fn(
        hwnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT,
>;

#[allow(non_snake_case)]
#[repr(C)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpsxMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}
pub type LPMSG = *mut MSG;
unsafe_impl_default_zeroed!(MSG);

// Painting information for the window
#[allow(non_snake_case)]
#[repr(C)]
pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

#[repr(C)]
pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);



/// CONSTANTS ///

// WS values to specify parameters for Window's window
// TODO: There are more, but this is enough for now
pub const WS_OVERLAPPED: u32 = 0x0000_0000;
pub const WS_CAPTION: u32 = 0x00C0_0000;
pub const WS_SYSMENU: u32 = 0x0008_0000;
pub const WS_THICKFRAME: u32 = 0x0004_0000;
pub const WS_MINIMIZEBOX: u32 = 0x0002_0000;
pub const WS_MAXIMIZEBOX: u32 = 0x0001_0000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED 
    | WS_CAPTION
    | WS_SYSMENU
    | WS_THICKFRAME
    | WS_MINIMIZEBOX
    | WS_MAXIMIZEBOX;
pub const CW_USEDEFAULT: c_int = 0x8000_0000_u32 as c_int;
pub const SW_SHOW: c_int = 5;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;

pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);

pub const WM_PAINT: u32 = 0x000F;

pub const COLOR_WINDOW: u32 = 5;

// WINDOWS API //

// Register the window using win32 C API
#[link(name = "User32")]
extern "system" {
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
}

// Register the window using win32 C API
#[link(name = "User32")]
extern "system" {
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID
    ) -> HWND;
}

// Show the window
#[link(name = "User32")]
extern "system" {
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
}

// Dummy window proc 
#[allow(non_snake_case)]
#[allow(unused_variables)] // remove after implementing
pub unsafe extern "system" fn dummy_window_procedure(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    return 0;
}

#[allow(non_snake_case)]
pub unsafe extern "system" fn window_procedure(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    let zero: i32 = 0;
    match Msg {
        WM_CLOSE => drop(DestroyWindow(hWnd)),
        WM_DESTROY => PostQuitMessage(zero),
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hWnd, &mut ps);
            let _success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW+4) as HBRUSH);
            EndPaint(hWnd, &ps);
        }
        _ => {
            return DefWindowProcW(
                hWnd, 
                Msg, 
                wParam, 
                lParam
            );
        }
    }

    return 0;
}

// Show the window
#[link(name = "User32")]
extern "system" {
    pub fn DefWindowProcW(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
}

// Get message from input
#[link(name = "User32")]
extern "system" {
    pub fn GetMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilternMin: UINT,
        wMsgFilterMax: UINT,
    ) -> BOOL;

    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;

    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn PostQuitMessage(nExitCode: c_int);

    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursor: LPCWSTR) -> HCURSOR;

    // Painting the window
    pub fn BeginPaint(hWnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
}