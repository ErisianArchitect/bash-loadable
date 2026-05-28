use std::{ffi::c_int, ptr::NonNull};

use crate::ffi::hash_table::{BucketRef, FFIBucket};

pub trait CPtrEquivalent<T: Sized> {}

impl<T: Sized> CPtrEquivalent<T> for *mut T {}
impl<T: Sized> CPtrEquivalent<T> for *const T {}
impl<T: Sized> CPtrEquivalent<T> for NonNull<T> {}
impl<T: Sized> CPtrEquivalent<T> for Option<NonNull<T>> {}
const _: () = {
    use lolevel::checks::assert_same_size_align;
    assert_same_size_align::<NonNull<()>, *const ()>();
    assert_same_size_align::<Option<NonNull<()>>, *const ()>();
};


#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct FreeFn(unsafe extern "C" fn(*mut ()));
const _: () = lolevel::checks::assert_same_align::<FreeFn, *const ()>();

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CopyFn(unsafe extern "C" fn (*const ()) -> *mut ());
const _: () = lolevel::checks::assert_same_align::<CopyFn, *const ()>();

#[cfg_attr(any(target_arch = "avr", target_arch = "msp430"), repr(i16))]
#[cfg_attr(not(any(target_arch = "avr", target_arch = "msp430")), repr(i32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WalkFlow {
    Continue = 1,
    Break = -1,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct HashWalkFn<'a>(pub unsafe extern "C" fn (BucketRef<'a>) -> WalkFlow);

impl FreeFn {
    #[must_use]
    #[inline(always)]
    pub const fn new<
        T: Sized,
        P: CPtrEquivalent<T>,
    >(f: unsafe extern "C" fn(P)) -> Self {
        Self(unsafe { std::mem::transmute(f) })
    }
}

impl CopyFn {
    #[must_use]
    #[inline(always)]
    pub const fn new<
        P: CPtrEquivalent<T>,
        T: Sized,
        R: CPtrEquivalent<T>,
    >(f: unsafe extern "C" fn(P) -> R) -> Self {
        Self(unsafe { std::mem::transmute(f) })
    }
}

impl<'a> HashWalkFn<'a> {
    #[must_use]
    #[inline(always)]
    pub const fn new(f: unsafe extern "C" fn (BucketRef<'a>) -> WalkFlow) -> Self {
        Self(f)
    }
}