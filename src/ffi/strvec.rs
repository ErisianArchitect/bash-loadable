use core::{
    ffi::{
        c_char,
    },
    ptr::{
        NonNull,
        null, null_mut,
    },
};
use std::marker::PhantomData;

use crate::{ffi::external, util::{check::assert_pointer_niche, ffi::CBool}};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct StrVecRef<'a> {
    ptr: NonNull<Option<NonNull<c_char>>>,
    _phantom: PhantomData<&'a c_char>,
}
const _: () = assert_pointer_niche::<StrVecRef<'_>>();
const _: () = assert_pointer_niche::<NonNull<c_char>>();

#[repr(transparent)]
pub struct StrVec {
    vec: StrVecRef<'static>,
}
const _: () = assert_pointer_niche::<StrVec>();

impl<'a> StrVecRef<'a> {
    /// Creates a copy of this array with the new size.
    #[must_use]
    #[inline(always)]
    pub fn resize(self, new_size: usize) -> StrVec {
        unsafe {
            external::ffi::strvec_resize(self, new_size)
        }
    }

    #[inline(always)]
    pub fn sort(self, posix: bool) {
        unsafe {
            external::ffi::strvec_sort(self, CBool::from_bool(posix))
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn deep_copy(self) -> StrVec {
        unsafe {
            external::ffi::strvec_copy(self)
        }
    }
}

impl StrVec {
    #[must_use]
    #[inline(always)]
    pub fn new(size: usize) -> Self {
        unsafe {
            external::ffi::strvec_create(size)
        }
    }

    #[inline(always)]
    pub fn dispose(self) {}

    #[must_use]
    #[inline]
    pub fn inner(&self) -> StrVecRef<'_> {
        self.vec
    }
}

impl Clone for StrVec {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.vec.deep_copy()
    }
}

impl Drop for StrVec {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            external::ffi::strvec_dispose(self.vec);
        }
    }
}