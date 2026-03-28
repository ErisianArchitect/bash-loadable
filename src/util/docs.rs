use core::ffi::{
    CStr,
    c_char,
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct LongDoc<const PARAGRAPHS: usize> {
    paragraphs: [*const c_char; PARAGRAPHS],
    null_end: usize,
}

impl<const PARAGRAPHS: usize> LongDoc<PARAGRAPHS> {
    #[must_use]
    #[inline(always)]
    pub const fn new(paragraphs: [*const c_char; PARAGRAPHS]) -> Self {
        Self {
            paragraphs,
            null_end: 0,
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const *const c_char {
        (self as *const Self).cast()
    }
}