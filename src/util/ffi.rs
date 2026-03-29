use core::ffi::{
    CStr,
    c_int,
    c_char,
};
#[cfg(any(target_arch = "avr", target_arch = "msp430"))]
use std::num::NonZeroI16;
#[cfg(not(any(target_arch = "avr", target_arch = "msp430")))]
use std::num::NonZeroI32;
use std::{
    borrow::Cow, ffi::CString, marker::PhantomData, mem::transmute, ptr::NonNull, str::FromStr
};

// RE-EXPORTS
pub use libc::{
    strlen,
};

#[cfg_attr(any(target_arch = "avr", target_arch = "msp430"), repr(i16))]
#[cfg_attr(not(any(target_arch = "avr", target_arch = "msp430")), repr(i32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CBool {
    False = 0,
    True = 1,
}

const _: () = {
    ["CBool incorrect size"]
        [size_of::<CBool>() - size_of::<c_int>()];
    ["CBool incorrect align"]
        [align_of::<CBool>() - align_of::<c_int>()];
};

pub use CBool::{False, True};

impl CBool {
    pub const BOTH: [Self; 2] = [Self::False, Self::True];
    #[must_use]
    #[inline(always)]
    pub const fn from_bool(value: bool) -> Self {
        Self::BOTH[value as usize]
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_c_int(value: c_int) -> Self {
        Self::from_bool(value != 0)
    }

    #[must_use]
    #[inline(always)]
    pub const fn to_c_int(self) -> c_int {
        self as c_int
    }
}

#[cfg(any(target_arch = "avr", target_arch = "msp430"))]
pub type BashStatusFailure = NonZeroI16;
#[cfg(not(any(target_arch = "avr", target_arch = "msp430")))]
pub type BashStatusFailure = NonZeroI32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BashStatus {
    Success,
    Failure(BashStatusFailure),
}

const _: () = {
    ["BashStatus incorrect size"]
        [size_of::<BashStatus>() - size_of::<c_int>()];
    ["BashStatus incorrect align"]
        [align_of::<BashStatus>() - align_of::<c_int>()];
    ["BashStatusFailure incorrect size"]
        [size_of::<BashStatusFailure>() - size_of::<c_int>()];
    ["BashStatusFailure incorrect align"]
        [align_of::<BashStatusFailure>() - align_of::<c_int>()];
};

impl BashStatus {
    pub const BOOL_ORDER: [Self; 2] = [Self::from_c_int(1), Self::Success];
    #[must_use]
    #[inline(always)]
    pub const fn from_bool(success: bool) -> Self {
        Self::BOOL_ORDER[success as usize]
    }

    #[must_use]
    #[inline(always)]
    pub const fn from_c_int(value: c_int) -> Self {
        unsafe {
            transmute(value)
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Success)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Failure(_))
    }

    #[must_use]
    #[inline(always)]
    pub const fn to_c_int(self) -> c_int {
        unsafe {
            transmute(self)
        }
    }
}

#[must_use]
#[inline(always)]
pub fn strlen_nonnull(s: NonNull<c_char>) -> usize {
    unsafe { strlen(s.as_ptr()) }
}

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
pub fn from_cstr_nonnull<'a>(cstr: NonNull<c_char>) -> &'a str {
    let len = strlen_nonnull(cstr);
    unsafe {
        transmute(std::slice::from_raw_parts(cstr.as_ptr().cast::<u8>(), len))
    }
}

#[must_use]
pub fn from_cstr<'a>(cstr: *const c_char) -> Option<&'a str> {
    Some(from_cstr_nonnull(NonNull::new(cstr.cast_mut())?))
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct RealCStr<'a> {
    pub inner: Option<NonNull<c_char>>,
    _phantom: PhantomData<&'a str>,
}

impl<'a> RealCStr<'a> {
    #[inline(always)]
    pub fn new(s: *const c_char) -> Self {
        Self {
            inner: unsafe { transmute(s) },
            _phantom: PhantomData,
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ptr(&self) -> *const c_char {
        unsafe {
            transmute(self.inner)
        }
    }

    #[must_use]
    pub fn to_str(&self) -> &'a str {
        if let Some(inner) = self.inner {
            from_cstr_nonnull(inner)
        } else {
            ""
        }
    }
}