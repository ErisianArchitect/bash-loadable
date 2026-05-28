use std::{cell::UnsafeCell, ffi::{
    CStr, c_char, c_int
}, mem::transmute};

use crate::{
    ffi::word::WordList, macros::cenum, util::ffi::{BashStatus, from_cstr}
};

// found in builtins.h, search for `#define BUILTIN_ENABLED`
cenum!{
    pub enum BuiltinFlags {
        /// This builtin is enabled.
        ENABLED       = 0x01,
        /// This builtin has been deleted with enable -d
        DELETED       = 0x02,
        /// This builtin is not dynamically loaded.
        STATIC_BUILTIN= 0x04,
        /// This is a Posix `special` builtin.
        SPECIAL       = 0x08,
        /// This builtin takes assignment statements.
        ASSIGNMENT    = 0x10,
        /// This builtin is special in the Posix command search order.
        POSIX         = 0x20,
        /// This builtin creates local variables.
        LOCALVAR      = 0x40,
        /// This builtin takes array references as arguments.
        ARRAYREF      = 0x80,
    }
}

pub type BuiltinFn = extern "C" fn(WordList) -> BashStatus;

pub struct BuiltinInfo {
    pub name: *const c_char,
    pub function: BuiltinFn,
    pub short_doc: *const c_char,
    pub long_doc: *const *const c_char,
}

impl BuiltinInfo {
    /// You most likely shouldn't be using this function. It's better to use `build`.
    #[must_use]
    #[inline]
    pub const fn build_with_flags(self, flags: BuiltinFlags) -> Builtin {
        Builtin(UnsafeCell::new(BuiltinInner {
            name: self.name,
            function: self.function,
            flags: flags.with_enabled(),
            long_doc: self.long_doc,
            short_doc: self.short_doc,
            handle: UnsafeCell::new(core::ptr::null()),
        }))
    }

    #[must_use]
    #[inline]
    pub const fn build(self) -> Builtin {
        self.build_with_flags(BuiltinFlags::ENABLED)
    }
}

#[repr(C)]
pub struct BuiltinInner {
    name: *const c_char,
    function: BuiltinFn,
    flags: BuiltinFlags,
    long_doc: *const *const c_char,
    short_doc: *const c_char,
    // Handle is not used by the builtin, and is instead used by bash for some reason.
    // Set it to null.
    handle: UnsafeCell<*const c_char>,
}

impl BuiltinInner {
    #[must_use]
    pub fn get_name(&self) -> &str {
        from_cstr(self.name).unwrap_or("")
    }

    #[must_use]
    #[inline(always)]
    pub fn get_function(&self) -> BuiltinFn {
        self.function
    }

    #[must_use]
    #[inline(always)]
    pub fn get_flags(&self) -> BuiltinFlags {
        self.flags
    }

    #[must_use]
    #[inline(always)]
    pub fn get_short(&self) -> &str {
        from_cstr(self.short_doc).unwrap_or("")
    }
}

#[repr(transparent)]
pub struct Builtin(UnsafeCell<BuiltinInner>);

unsafe impl Sync for Builtin {}
