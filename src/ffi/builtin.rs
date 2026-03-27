use std::{ffi::{
    c_char,
    c_int,
}};

use super::word::{
    WordListRef,
};

pub type BuiltinFunc = extern "C" fn(WordListRef) -> c_int;

#[repr(C)]
pub struct Builtin {
    pub name: *const c_char,
    pub function: BuiltinFunc,
    pub flags: c_int,
    pub long_doc: *const *const c_char,
    pub short_doc: *const c_char,
    pub handle: *const c_char,
}

unsafe impl Send for Builtin {}
unsafe impl Sync for Builtin {}