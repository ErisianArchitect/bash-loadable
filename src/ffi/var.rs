use core::ffi::{
    c_char,
    c_long,
    c_int,
};
use std::{marker::PhantomData, mem::transmute, ptr::NonNull};

use crate::{ffi::external, util::ffi::{from_cstr, to_cstr}};

pub type ShellVarValueFn = extern "C" fn(*const FFIShellVar);
pub type ShellVarAssignFn = extern "C" fn(*mut FFIShellVar, value: *const c_char, index: c_long, key: *const c_char) -> *const FFIShellVar;

#[repr(C)]
pub struct FFIShellVar<'a> {
    pub name: *const c_char,
    pub value: *const c_char,
    pub export_str: *const c_char,
    pub dynamic_value: ShellVarValueFn,
    pub assign_func: ShellVarAssignFn,
    pub attributes: c_int,
    pub context: c_int,
    _phantom: PhantomData<&'a ()>,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ShellVar<'a>(Option<NonNull<FFIShellVar<'a>>>);

impl<'a> ShellVar<'a> {
    pub const NULL: Self = Self(None);
    #[must_use]
    #[inline(always)]
    pub const fn null() -> Self {
        Self::NULL
    }

    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> Option<&'a FFIShellVar<'a>> {
        unsafe { transmute(self.0) }
    }

    #[must_use]
    #[inline(always)]
    pub fn get_mut(&mut self) -> Option<&'a mut FFIShellVar<'a>> {
        unsafe { transmute(self.0) }
    }

    #[must_use]
    #[inline(always)]
    pub fn find(name: &str) -> Self {
        unsafe {
            external::ffi::find_variable(to_cstr(name).as_ptr())
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn find_global(name: &str) -> Self {
        unsafe {
            external::ffi::find_global_variable(to_cstr(name).as_ptr())
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn find_shell_var(name: &str) -> Self {
        unsafe {
            external::ffi::find_shell_variable(to_cstr(name).as_ptr())
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn find_tempenv_var(name: &str) -> Self {
        unsafe {
            external::ffi::find_tempenv_variable(to_cstr(name).as_ptr())
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn copy<'b>(self) -> ShellVar<'b> {
        unsafe {
            external::ffi::copy_variable(self)
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn value(&self) -> &str {
        from_cstr(unsafe {
            external::ffi::get_variable_value(*self)
        }).unwrap_or("")
    }

    #[inline(always)]
    pub fn dispose(self) {
        unsafe {
            external::ffi::dispose_variable(self);
        }
    }
}