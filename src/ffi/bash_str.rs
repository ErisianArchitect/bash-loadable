use core::{
    ffi::{
        CStr,
        c_char,
    },
    mem::{
        transmute,
    },
    ptr::{
        NonNull,
    },
};
use std::marker::PhantomData;

use crate::{
    ffi::external,
    util::ffi::{
        from_cstr_nonnull, strlen_nonnull,
    },
};


#[repr(transparent)]
pub struct BashStr {
    ptr: NonNull<c_char>,
}

impl BashStr {
    #[must_use]
    #[inline(always)]
    pub fn new(s: *const c_char) -> Option<Self> {
        // SAFETY: `BashStr` has the same size and layout as `*const c_char`.
        // `Option<BashStr>` has the same size and layout as `BashStr`.
        // `BashStr` is a NonNull internally, which has a niche optimization
        // so that `null` becomes `Option::None`.
        unsafe {
            transmute(s)
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ptr(&self) -> *const c_char {
        self.ptr.as_ptr().cast_const()
    }

    /// This is O(n), which is why it's named `get_len` instead of `len`.
    #[must_use]
    #[inline]
    pub fn get_len(&self) -> usize {
        strlen_nonnull(self.ptr)
    }

    #[must_use]
    #[inline]
    pub fn to_str(&self) -> &str {
        from_cstr_nonnull(self.ptr)
    }
}

impl Drop for BashStr {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            external::ffi::xfree(self.ptr.as_ptr().cast());
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BashStrRef<'a> {
    ptr: NonNull<c_char>,
    _phantom: PhantomData<&'a c_char>,
}

impl<'a> BashStrRef<'a> {
    #[must_use]
    pub fn get_len(self) -> usize {
        unsafe {
            CStr::from_ptr(self.ptr.as_ptr())
        }.count_bytes()
    }

    #[must_use]
    #[inline]
    pub fn to_str(self) -> &'a str {
        from_cstr_nonnull(self.ptr)
    }
}