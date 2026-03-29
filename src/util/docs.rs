use core::ffi::{
    CStr,
    c_char,
};

mod force_zero {
    #[repr(transparent)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Zero(usize);

    impl Zero {
        pub const ZERO: Self = Self(0);
    }
}

use force_zero::Zero;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct LongDoc<const PARAGRAPHS: usize> {
    paragraphs: [*const c_char; PARAGRAPHS],
    null_end: Zero,
}

impl<const PARAGRAPHS: usize> LongDoc<PARAGRAPHS> {
    #[must_use]
    #[inline(always)]
    pub const fn new(paragraphs: [*const c_char; PARAGRAPHS]) -> Self {
        Self {
            paragraphs,
            null_end: Zero::ZERO,
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const *const c_char {
        (self as *const Self).cast()
    }
}