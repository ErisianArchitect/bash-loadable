use std::{ffi::{
    CStr, c_char, c_int
}, mem::transmute};

use crate::{
    ffi::word::WordList, macros::cenum, util::ffi::from_cstr
};

cenum!{
    pub enum BuiltinFlags {
        /// This builtin is enabled.
        ENABLED         enabled         = 0x01,
        /// This builtin has been deleted with enable -d
        DELETED         deleted         = 0x02,
        /// This built8in is not dynamically loaded.
        STATIC_BUILTIN  static_builtin  = 0x04,
        /// This is a Posix `special` builtin.
        SPECIAL         special         = 0x08,
        /// This bultin takes assignment statements.
        ASSIGNMENT      assignment      = 0x10,
        /// This builtin is special in the Posix command search order.
        POSIX           posix           = 0x20,
        /// This builtin creates local variables.
        LOCALVAR        localvar        = 0x40,
        /// This builtin takes array references as arguments.
        ARRAYREF        arrayref        = 0x80,
        
    }
}

pub type BuiltinFn = extern "C" fn(WordList) -> c_int;

pub struct BuiltinInfo {
    pub name: *const c_char,
    pub function: BuiltinFn,
    pub short_doc: *const c_char,
    pub long_doc: *const *const c_char,
}

impl BuiltinInfo {
    #[must_use]
    #[inline]
    pub const fn build(self) -> Builtin {
        self.build_with_flags(BuiltinFlags::ENABLED)
    }

    /// You most likely shouldn't be using this function. It's better to use `build`.
    #[must_use]
    #[inline]
    pub const fn build_with_flags(self, flags: BuiltinFlags) -> Builtin {
        Builtin {
            name: self.name,
            function: self.function,
            flags,
            long_doc: self.long_doc,
            short_doc: self.short_doc,
            handle: core::ptr::null(),
        }
    }
}

#[repr(C)]
pub struct Builtin {
    name: *const c_char,
    function: BuiltinFn,
    flags: BuiltinFlags,
    long_doc: *const *const c_char,
    short_doc: *const c_char,
    // Handle is not used by the builtin, and is instead used by bash for some reason.
    // Set it to null.
    handle: *const c_char,
}

impl Builtin {
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

unsafe impl Send for Builtin {}
unsafe impl Sync for Builtin {}