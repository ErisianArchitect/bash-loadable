use std::{ffi::{c_int, c_uint}, ptr::NonNull};

use crate::{cenum, ffi::{bash_str::BashStrRef, fn_ptr::FreeFn}};

cenum! {
    pub enum TableFlags {
        NO_SEARCH = 0x01,
        CREATE    = 0x02,
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct HashDataFreeFn(FreeFn);

impl HashDataFreeFn {
    #[must_use]
    #[inline(always)]
    pub const fn new<T: Sized>(f: unsafe extern "C" fn(Option<NonNull<T>>)) -> Self {
        Self(FreeFn::new(f))
    }
}

#[repr(C)]
pub struct FFIBucket<'a> {
    pub next: Option<BucketRef<'a>>,
    pub key: BashStrRef<'a>,
    pub data: Option<NonNull<()>>,
    pub khash: c_uint,
    pub times_founds: c_int,
}

#[repr(C)]
pub struct FFITable<'a> {
    pub bucket_array: *const *const BucketRef<'a>,
    pub nbuckets: c_int,
    pub nentries: c_int,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BucketRef<'a>(NonNull<FFIBucket<'a>>);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TableRef<'a>(NonNull<FFITable<'a>>);



impl<'a> BucketRef<'a> {
    #[must_use]
    #[inline(always)]
    pub const fn forget_lifetime(self) -> BucketRef<'static> {
        BucketRef(self.0.cast())
    }
}

impl<'a> TableRef<'a> {
    #[must_use]
    #[inline(always)]
    pub const fn forget_lifetime(self) -> TableRef<'static> {
        TableRef(self.0.cast())
    }
}