#[allow(non_camel_case_types)]
pub type UINT = c_uint;

#[allow(non_camel_case_types)]
pub type UINT_PTR = usize;

#[allow(non_camel_case_types)]
pub type PVOID = *mut core::ffi::c_void;

#[allow(non_camel_case_types)]
pub type c_uint = u32;

#[allow(non_camel_case_types)]
pub type c_int = i32;

#[allow(non_camel_case_types)]
pub type LONG_PTR = isize;

#[allow(non_camel_case_types)]
pub type LPARAM = LONG_PTR;

#[allow(non_camel_case_types)]
pub type LRESULT = LONG_PTR;

#[allow(non_camel_case_types)]
pub type HINSTANCE = HANDLE;

#[allow(non_camel_case_types)]
pub type HBRUSH = HANDLE;

#[allow(non_camel_case_types)]
pub type HANDLE = PVOID;

#[allow(non_camel_case_types)]
pub type HMODULE = HINSTANCE;

#[allow(non_camel_case_types)]
pub type HICON = HANDLE;

#[allow(non_camel_case_types)]
pub type HCURSOR = HICON;

#[allow(non_camel_case_types)]
pub type LPCWSTR = *const WCHAR;

#[allow(non_camel_case_types)]
pub type WCHAR = wchar_t;

#[allow(non_camel_case_types)]
pub type wchar_t = u16;

#[allow(non_camel_case_types)]
pub type HWND = HANDLE;

#[allow(non_camel_case_types)]
pub type WPARAM = UINT_PTR;

#[allow(non_camel_case_types)]
pub type ATOM = WORD;

#[allow(non_camel_case_types)]
pub type WORD = c_ushort;

#[allow(non_camel_case_types)]
pub type c_ushort = u16;

#[allow(non_camel_case_types)]
pub type DWORD = c_ulong;

#[allow(non_camel_case_types)]
pub type c_ulong = u32;

#[allow(non_camel_case_types)]
pub type HMENU = HANDLE;

#[allow(non_camel_case_types)]
pub type LPVOID = *mut std::ffi::c_void;

#[allow(non_camel_case_types)]
pub type BOOL = c_int;

#[allow(non_camel_case_types)]
pub type LONG = c_long;

#[allow(non_camel_case_types)]
pub type c_long = i32;

#[allow(non_camel_case_types)]
pub type LPWSTR = *mut WCHAR;

#[allow(non_camel_case_types)]
pub type ULONG_PTR = usize;

#[allow(non_camel_case_types)]
pub type HDC = HANDLE;

#[allow(non_camel_case_types)]
pub type BYTE = u8;

#[allow(non_camel_case_types)]
pub type LPCVOID = std::ffi::c_void;

#[allow(non_camel_case_types)]
pub type va_list = *mut c_char;

#[allow(non_camel_case_types)]
pub type c_char = i8;