use core::{
    ffi::{
        c_char,
    },
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Alias {
    pub name: *const c_char,
    pub value: *const c_char,
    pub flags: c_char,
}