pub mod builtin;
pub mod command;
pub mod common;
pub mod word;

use core::ffi::{
    c_int,
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