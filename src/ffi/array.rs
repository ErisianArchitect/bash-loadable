use core::{
    ffi::{
        c_long,
        c_char,
    },
};
use std::{marker::PhantomData, mem::transmute, ptr::NonNull};
use crate::{
    macros::{
        cenum,
    },
};

cenum!{
    pub enum ShiftElementFlags {
        /// The element should be disposed.
        DISPOSE dispose = 0x01,
    }
}

cenum!{
    // TODO: Better documentation.
    pub enum PFlags {
        /// Do not perform command substitution.
        NO_COMMAND_SUBSTITUTION no_command_substitution = 0x01,
        /// Ignore unbound vars even if -u set.
        IGNORE_UNBOUND_VARS     ignore_unbound_vars     = 0x02,
        /// Same as W_NOSPLTI2
        NO_SPLIT2               no_split2               = 0x04,
        /// Same as W_ASSIGNRHS.
        ASSIGN_RHS              assign_rhs              = 0x08,
        /// Same as W_COMPLETE, sets SX_COMPLETE
        COMPLETE                complete                = 0x10,
        /// Same as W_EXPANDRHS
        EXPAND_RHS              expand_rhs              = 0x20,
        /// Array, act as if [@] was supplied.
        ALL_INDICES             all_indices             = 0x40,
        /// Differentiate `` from $() for command_substitution
        BACKQUOTE               backquote               = 0x80,
    }
}

#[repr(C)]
pub struct FFIArrayElement<'a> {
    pub index: c_long,
    pub value: *const c_char,
    pub next: ArrayElement<'a>,
    pub prev: ArrayElement<'a>,
    _phantom: PhantomData<&'a ()>,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ArrayElement<'a>(Option<NonNull<FFIArrayElement<'a>>>);

#[repr(C)]
pub struct FFIArray<'a> {
    pub max_index: c_long,
    pub num_elements: c_long,
    pub first_index: c_long,
    pub alloc_size: c_long,
    pub elements: *const ArrayElement<'a>,
}

#[repr(transparent)]
pub struct Array<'a> {
    pub inner: Option<NonNull<FFIArray<'a>>>,
}

impl<'a> Array<'a> {
    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> Option<&'a FFIArray<'a>> {
        unsafe {
            transmute(self.inner)
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn get_mut(&mut self) -> Option<&'a FFIArray<'a>> {
        unsafe {
            transmute(self.inner)
        }
    }
}