// use core::panic;

// use std::ptr::{
//     null,
//     null_mut,
// };
use crate::win32::types::*;
use crate::win32::core::*;

/// Define a function to zero out a struct's fields
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
#[allow(non_snake_case)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD,
}
pub type LPMSG = *mut MSG;
unsafe_impl_default_zeroed!(MSG);

// Painting information for the window
#[allow(non_snake_case)]
#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);

#[allow(non_snake_case)]
#[repr(C)]
pub struct CREATESTRUCTW {
    pub lpCreateParams: LPVOID,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hWndParent: HWND,
    pub cy: c_int,
    pub cx: c_int,
    pub y: c_int,
    pub x: c_int,
    pub style: LONG,
    pub lpszName: LPCWSTR,
    pub lpszClass: LPCWSTR,
    pub dwExStyle: DWORD,
}
unsafe_impl_default_zeroed!(CREATESTRUCTW);

/// Enumeration of predefined cursors styles
pub enum IDCursor {
    /// Standard arrow and small hourglass
    AppStarting = 32650,
    
    /// Standard arrow
    Arrow = 32512,
    
    /// Crosshair
    Cross = 32515,
    
    /// Hand
    Hand = 32649,
    
    /// Arrow and question mark
    Help = 32651,
    
    /// I-beam
    IBeam = 32513,
    
    /// Slashed circle
    No = 32648,
    
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll = 32646,
    
    /// Double-pointed arrow pointing northeast and southwest
    SizeNeSw = 32643,
    
    /// Double-pointed arrow pointing north and south
    SizeNS = 32645,
    
    /// Double-pointed arrow pointing northwest and southeast
    SizeNwSe = 32642,
    
    /// Double-pointed arrow pointing west and east
    SizeWE = 32644,
    
    /// Vertical arrow
    UpArrow = 32516,
    
    /// Hourglass
    Wait = 32514,
}

/// See [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub enum SysColor {
  _3dDarkShadow = 21,
  _3dLight = 22,
  ActiveBorder = 10,
  ActiveCaption = 2,
  AppWorkspace = 12,
  /// Button face, also "3D face" color.
  ButtonFace = 15,
  /// Button highlight, also "3D highlight" color.
  ButtonHighlight = 20,
  /// Button shadow, also "3D shadow" color.
  ButtonShadow = 16,
  ButtonText = 18,
  CaptionText = 9,
  /// Desktop background color
  Desktop = 1,
  GradientActiveCaption = 27,
  GradientInactiveCaption = 28,
  GrayText = 17,
  Highlight = 13,
  HighlightText = 14,
  HotLight = 26,
  InactiveBorder = 11,
  InactiveCaption = 3,
  InactiveCaptionText = 19,
  InfoBackground = 24,
  InfoText = 23,
  Menu = 4,
  MenuHighlight = 29,
  MenuBar = 30,
  MenuText = 7,
  ScrollBar = 0,
  Window = 5,
  WindowFrame = 6,
  WindowText = 8,
}

// CONSTANTS //

// WS values to specify parameters for Window's window
// TODO: There are more, but this is enough for now
pub const WS_OVERLAPPED: u32 = 0x0000_0000;
pub const WS_CAPTION: u32 = 0x00C0_0000;
pub const WS_SYSMENU: u32 = 0x0008_0000;
pub const WS_THICKFRAME: u32 = 0x0004_0000;
pub const WS_MINIMIZEBOX: u32 = 0x0002_0000;
pub const WS_MAXIMIZEBOX: u32 = 0x0001_0000;
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;
pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED 
    | WS_CAPTION
    | WS_SYSMENU
    | WS_THICKFRAME
    | WS_MINIMIZEBOX
    | WS_MAXIMIZEBOX;
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
pub const CW_USEDEFAULT: c_int = 0x8000_0000_u32 as c_int;
pub const SW_SHOW: c_int = 5;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_QUIT: u32 = 0x0012;
pub const WM_DESTROY: u32 = 0x0002;

pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);

pub const WM_PAINT: u32 = 0x000F;

pub const COLOR_WINDOW: u32 = 5;
pub const MB_OKCANCEL: u32 = 1;
pub const IDOK: c_int = 1;

pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_SETCURSOR: u32 = 0x0020;

pub const GWLP_USERDATA: c_int = -21;

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
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;

    // Painting the window
    pub fn BeginPaint(hWnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;

    // Closing the window
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT);

    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
}