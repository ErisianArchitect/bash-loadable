use std::{mem::transmute, ptr::NonNull};

use crate::ffi::external;



#[repr(transparent)]
pub struct BashOwned<T: Sized> {
    pub ptr: NonNull<T>,
}

impl<T: Sized> BashOwned<T> {
    #[must_use]
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    #[must_use]
    #[inline(always)]
    pub fn take_ptr(self) -> *mut T {
        let ptr = self.ptr.as_ptr();
        self.forget();
        ptr
    }

    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> &T {
        unsafe {
            self.ptr.as_ref()
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe {
            self.ptr.as_mut()
        }
    }

    #[inline(always)]
    pub fn dispose(self) {}

    #[inline(always)]
    pub fn forget(self) {
        core::mem::forget(self);
    }
}

impl<T: Sized> Drop for BashOwned<T> {
    fn drop(&mut self) {
        unsafe {
            external::ffi::xfree(self.ptr.as_ptr().cast());
        }
    }
}