use crate::win32::types::*;

// Get a handle to an HINSTANCE using Windows's win32 API
#[link(name = "Kernel32")]
extern "system" {
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    // pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
}


#[link(name = "Kernel32")]
extern "system" {
    pub fn GetLastError() -> DWORD;

    pub fn FormatMessageW(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPWSTR,
        nSize: DWORD,
        Arguments: va_list,
    ) -> DWORD;

    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;

    pub fn SetLastError(dwErrCode: DWORD);
}

#[allow(non_snake_case)]
pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR {
    return i as ULONG_PTR as LPWSTR;
}