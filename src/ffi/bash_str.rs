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
use std::{marker::PhantomData, mem::ManuallyDrop};

use crate::{
    ffi::external,
    util::ffi::{
        from_cstr_nonnull, strlen_nonnull, to_cstr,
    },
};


#[repr(transparent)]
pub struct BashStr {
    ptr: NonNull<c_char>,
}

unsafe impl Send for BashStr where Box<str>: Send {}
unsafe impl Sync for BashStr where Box<str>: Sync {}

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

    pub fn from_str(s: &str) -> Self {
        unsafe {
            let cstr = to_cstr(s);
            let len = cstr.count_bytes();
            let buf = external::ffi::xmalloc(len).cast::<c_char>();
            let Some(ptr) = NonNull::new(buf) else {
                panic!("Null pointer: {}:{}:{}", file!(), line!(), column!());
            };
            let cstr_nonnull = NonNull::new_unchecked(cstr.as_ptr().cast::<c_char>().cast_mut());
            ptr.copy_from_nonoverlapping(cstr_nonnull, len);
            ptr.byte_add(len).write(0);
            Self {
                ptr,
            }

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

    #[must_use]
    #[inline]
    pub fn take(self) -> *const c_char {
        ManuallyDrop::new(self).ptr.as_ptr()
    }

    #[must_use]
    #[inline]
    pub fn bash_ref(&self) -> BashStrRef<'_> {
        BashStrRef {
            ptr: self.ptr,
            _phantom: PhantomData,
        }
    }
}

impl std::fmt::Display for BashStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <str as std::fmt::Display>::fmt(self.to_str(), f)
    }
}

impl std::fmt::Debug for BashStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <str as std::fmt::Debug>::fmt(self.to_str(), f)
    }
}

impl Clone for BashStr {
    fn clone(&self) -> Self {
        Self::from_str(self.to_str())
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
    pub fn from_ptr(ptr: *const c_char) -> Option<Self> {
        Some(BashStrRef { ptr: NonNull::new(ptr.cast_mut())?, _phantom: PhantomData })
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ptr(&self) -> *const c_char {
        self.ptr.as_ptr().cast_const()
    }

    #[must_use]
    pub fn get_len(self) -> usize {
        strlen_nonnull(self.ptr)
    }

    #[must_use]
    #[inline]
    pub fn to_str(self) -> &'a str {
        from_cstr_nonnull(self.ptr)
    }
}