// use crate::win32::types::*;

pub fn wide_null(s: &str) -> Vec<u16> {
    return s.encode_utf16().chain(Some(0)).collect();
}