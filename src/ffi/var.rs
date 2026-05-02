use core::ffi::{
    c_char,
    c_long,
    c_int,
};
use std::{marker::PhantomData, mem::{ManuallyDrop, transmute}, ptr::NonNull};

use crate::{cenum, ffi::{array::ArrayRef, bash_str::BashStrRef, external}, util::ffi::{from_cstr, to_cstr}};

pub type ShellVarValueFn = extern "C" fn(*const FFIShellVar);
pub type ShellVarAssignFn = extern "C" fn(*mut FFIShellVar, value: *const c_char, index: c_long, key: *const c_char) -> *const FFIShellVar;

cenum! {
    pub enum VarAttrs {
        EXPORTED      = 0x0000001,
        READONLY      = 0x0000002,
        ARRAY         = 0x0000004,
        FUNCTION      = 0x0000008,
        INTEGER       = 0x0000010,
        LOCAL         = 0x0000020,
        ASSOC         = 0x0000040,
        TRACE         = 0x0000080,
        UPPERCASE     = 0x0000100,
        LOWERCASE     = 0x0000200,
        CAPCASE       = 0x0000400,
        NAMEREF       = 0x0000800,

        MASK_USER     = 0x0000fff,

        INVISIBLE     = 0x0001000,
        NO_UNSET      = 0x0002000,
        NO_ASSIGN     = 0x0004000,
        IMPORTED      = 0x0008000,
        SPECIAL       = 0x0010000,
        NO_FREE       = 0x0020000,
        REGENERATE    = 0x0040000,

        MASK_INT      = 0x00ff000,
        
        TEMP_VAR      = 0x0100000,
        PROPAGATE     = 0x0200000,

        MASK_SCOPE    = 0x0f00000,
    }
}

// search for: `#define VC_HASLOCAL`
cenum! {
    pub enum VCFlags {
        HAS_LOCAL     = 0x01,
        HAS_TMP_VAR   = 0x02,
        FUNC_ENV      = 0x04,
        BUILTIN_ENV   = 0x08,
        TEMP_ENV      = 0x10,
        SPEC_TEMP_ENV = 0x20,

        TEMP_FLAGS    = Self::join(&[
                            Self::FUNC_ENV,
                            Self::BUILTIN_ENV,
                            Self::TEMP_ENV,
                        ]),
    }
}

#[repr(C)]
pub struct FFIShellVar<'a> {
    pub name: *const c_char,
    pub value: *const c_char,
    pub export_str: *const c_char,
    pub dynamic_value: ShellVarValueFn,
    pub assign_func: ShellVarAssignFn,
    pub attributes: VarAttrs,
    pub context: c_int,
    _phantom: PhantomData<&'a ()>,
}

#[repr(C)]
pub struct FFIVarContext<'a> {
    pub name: Option<BashStrRef<'static>>,
    pub scope: c_int,
    pub flags: VCFlags,
    pub up: Option<NonNull<FFIVarContext<'a>>>,
    pub down: Option<NonNull<FFIVarContext<'a>>>,
    // TODO: Implement HASH_TABLE
    pub table: *const (),
    _phantom: PhantomData<&'a ()>,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ShellVar<'a>(NonNull<FFIShellVar<'a>>);

#[derive(Clone, Copy)]
pub enum ShellVarValue<'a> {
    Null,
    Str(&'a str),
    Array(ArrayRef<'a>),
}

impl<'a> ShellVar<'a> {
    pub const NULL: Option<Self> = None;
    #[must_use]
    #[inline(always)]
    pub const fn null() -> Option<Self> {
        Self::NULL
    }

    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> &'a FFIShellVar<'a> {
        unsafe { self.0.as_ref() }
    }

    #[must_use]
    #[inline(always)]
    pub fn get_mut(&mut self) -> &'a mut FFIShellVar<'a> {
        unsafe { self.0.as_mut() }
    }

    #[must_use]
    #[inline(always)]
    pub fn find(name: &str) -> Option<Self> {
        unsafe {
            external::ffi::find_variable(to_cstr(name).as_ptr())
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn find_global(name: &str) -> Option<Self> {
        unsafe {
            external::ffi::find_global_variable(to_cstr(name).as_ptr())
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn find_shell_var(name: &str) -> Option<Self> {
        unsafe {
            external::ffi::find_shell_variable(to_cstr(name).as_ptr())
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn find_tempenv_var(name: &str) -> Option<Self> {
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
    pub fn value<'b>(&self) -> ShellVarValue<'b>
    where 'a: 'b {
        if self.get().attributes.get_array() {
            let array_ptr = self.get().value;
            let Some(ptr) = NonNull::new(array_ptr.cast_mut()) else {
                return ShellVarValue::Null;
            };
            ShellVarValue::Array(ArrayRef::from_ptr(ptr.cast()))
        } else if self.get().attributes.get_assoc() {
            // TODO
            ShellVarValue::Null
        } else {
            ShellVarValue::Str(from_cstr(unsafe {
                external::ffi::get_variable_value(*self)
            }).unwrap_or(""))
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn forget_lifetime(self) -> ShellVar<'static> {
        ShellVar(ManuallyDrop::new(self).0.cast())
    }

    #[inline(always)]
    pub fn dispose(self) {
        unsafe {
            external::ffi::dispose_variable(self);
        }
    }
}