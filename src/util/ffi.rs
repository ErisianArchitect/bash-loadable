use core::ffi::{
    CStr,
    c_int,
    c_char,
};
use std::{
    borrow::Cow, ffi::CString, mem::transmute, str::FromStr
};

#[must_use]
#[inline(always)]
pub const fn cbool(value: bool) -> c_int {
    value as c_int
}

#[must_use]
#[inline(always)]
pub const fn from_cbool(value: c_int) -> bool {
    value != 0
}

#[must_use]
pub fn to_cstr<'a>(s: &'a str) -> Cow<'a, CStr> {
    if s.is_empty() {
        return Cow::Borrowed(c"");
    }
    match CStr::from_bytes_until_nul(s.as_bytes()) {
        Ok(cstr) => Cow::Borrowed(cstr),
        Err(_) => Cow::Owned(unsafe { CString::new(s.as_bytes()).unwrap_unchecked() })
    }
}

#[must_use]
pub fn from_cstr<'a>(cstr: *const c_char) -> &'a str {
    if cstr.is_null() {
        return "";
    }
    let str_cstr = unsafe { CStr::from_ptr(cstr) };
    let len = str_cstr.count_bytes();
    unsafe {
        transmute(core::slice::from_raw_parts(cstr.cast::<u8>(), len))
    }
}