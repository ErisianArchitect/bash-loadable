use core::{
    ffi::{
        c_long,
        c_int,
        c_char,
    },
    marker::PhantomData,
    ptr::NonNull,
};
use std::{mem::transmute, ops::{ControlFlow, Deref}};
use crate::{
    ffi::external,
    macros::cenum,
    util::{check::assert_pointer_niche, ffi::to_cstr},
};

cenum!{
    pub enum ShiftElementFlags {
        /// The element should be disposed.
        DISPOSE = 0x01,
    }
}

cenum!{
    // TODO: Better documentation.
    pub enum PFlags {
        /// Do not perform command substitution.
        NO_COMMAND_SUBSTITUTION   = 0x01,
        /// Ignore unbound vars even if -u set.
        IGNORE_UNBOUND_VARS       = 0x02,
        /// Same as W_NOSPLTI2
        NO_SPLIT2                 = 0x04,
        /// Same as W_ASSIGNRHS.
        ASSIGN_RHS                = 0x08,
        /// Same as W_COMPLETE, sets SX_COMPLETE
        COMPLETE                  = 0x10,
        /// Same as W_EXPANDRHS
        EXPAND_RHS                = 0x20,
        /// Array, act as if [@] was supplied.
        ALL_INDICES               = 0x40,
        /// Differentiate `` from $() for command_substitution
        BACKQUOTE                 = 0x80,
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ArrayElementMapFn<T> {
    func: extern "C" fn(ArrayElementRef<'_>, &mut T) -> c_int,
}

impl<T> ArrayElementMapFn<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(func: extern "C" fn(ArrayElementRef<'_>, &mut T) -> c_int) -> Self {
        Self { func }
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn erased(self) -> ArrayElementMapFn<()> {
        unsafe {
            transmute(self)
        }
    }
}

#[repr(C)]
pub struct FFIArrayElement {
    pub index: c_long,
    pub value: *const c_char,
    pub next: Option<ArrayElementRef<'static>>,
    pub prev: Option<ArrayElementRef<'static>>,
}

#[repr(C)]
pub struct FFIArray {
    pub max_index: c_long,
    pub num_elements: c_long,
    pub first_index: c_long,
    pub alloc_size: c_long,
    pub elements: *const Option<ArrayElementRef<'static>>,
}

/// Borrowed variant of ArrayElement.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ArrayElementRef<'a> {
    ptr: NonNull<FFIArrayElement>,
    _phantom: PhantomData<&'a FFIArrayElement>,
}
const _: () = assert_pointer_niche::<ArrayElementRef>();

/// Owned variant of ArrayElement
#[repr(transparent)]
pub struct ArrayElement {
    elem: ArrayElementRef<'static>,
}
const _: () = assert_pointer_niche::<ArrayElement>();

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ArrayRef<'a> {
    ptr: NonNull<FFIArray>,
    _phantom: PhantomData<&'a FFIArray>,
}
const _: () = assert_pointer_niche::<ArrayRef>();

#[repr(transparent)]
pub struct Array {
    arr: ArrayRef<'static>,
}
const _: () = assert_pointer_niche::<Array>();

impl<'a> ArrayElementRef<'a> {
    #[must_use]
    #[inline(always)]
    pub fn from_ptr(ptr: NonNull<FFIArrayElement>) -> Self {
        Self { ptr, _phantom: PhantomData }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ptr(self) -> *const FFIArrayElement {
        self.ptr.as_ptr()
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ref(&self) -> &FFIArrayElement {
        unsafe { self.ptr.as_ref() }
    }

    // #[must_use]
    // #[inline(always)]
    // pub fn copy(self) -> Option<ArrayElement> {
    //     unsafe {
    //         external::ffi::array_copy_element(self.ptr)
    //     }
    // }
    
    #[must_use]
    #[inline(always)]
    pub unsafe fn make_static(self) -> ArrayElementRef<'static> {
        ArrayElementRef { ptr: self.ptr, _phantom: PhantomData }
    }

    #[must_use]
    #[inline(always)]
    pub fn shorten_lifetime<'b>(self) -> ArrayElementRef<'b>
    where 'a: 'b {
        ArrayElementRef { ptr: self.ptr, _phantom: PhantomData }
    }
}

impl ArrayElement {
    #[must_use]
    #[inline(always)]
    pub fn new(index: c_long, value: &str) -> Option<Self> {
        let value = to_cstr(value).as_ptr();
        unsafe {
            external::ffi::array_create_element(index, value)
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ref(&self) -> &FFIArrayElement {
        self.elem.as_ref()
    }

    #[must_use]
    #[inline(always)]
    pub fn elem_ref<'a>(&'a self) -> ArrayElementRef<'a> {
        self.elem.shorten_lifetime()
    }
    
    // #[must_use]
    // #[inline(always)]
    // pub fn copy(&self) -> Option<ArrayElement> {
    //     self.elem.copy()
    // }

    #[inline(always)]
    pub fn dispose(self) {}

    #[inline(always)]
    pub fn forget(self) {
        core::mem::forget(self);
    }
}

impl Drop for ArrayElement {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            external::ffi::array_dispose_element(self.elem.ptr);
        }
    }
}

impl<'a> ArrayRef<'a> {
    #[must_use]
    #[inline(always)]
    pub fn from_ptr(ptr: NonNull<FFIArray>) -> Self {
        Self { ptr, _phantom: PhantomData }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ptr(self) -> NonNull<FFIArray> {
        self.ptr
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ref(self) -> &'a FFIArray {
        unsafe {
            self.ptr.as_ref()
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn copy(self) -> Array {
        unsafe {
            external::ffi::array_copy(self.ptr)
        }
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn make_static(self) -> ArrayRef<'static> {
        ArrayRef { ptr: self.ptr, _phantom: PhantomData }
    }

    #[must_use]
    #[inline(always)]
    pub fn shorten_lifetime<'b>(self) -> ArrayRef<'b>
    where 'a: 'b {
        ArrayRef { ptr: self.ptr, _phantom: PhantomData }
    }

    // #[inline(always)]
    // pub fn resize(self, n: c_long) {
    //     unsafe {
    //         external::ffi::array_resize(self.ptr, n);
    //     }
    // }

    // #[inline(always)]
    // pub fn expand(self, n: c_long) {
    //     unsafe {
    //         external::ffi::array_expand(self.ptr, n);
    //     }
    // }

    #[must_use]
    #[inline(always)]
    pub fn slice(self, start: c_long, end: c_long) -> Array {
        unsafe {
            external::ffi::array_slice(
                self,
                start,
                end,
            )
        }
    }

    pub fn walk<F: FnMut(ArrayElementRef<'a>) -> ControlFlow<()>>(self, f: F) {
        extern "C" fn walker(
            elem: ArrayElementRef<'_>,
            data: &mut Box<dyn FnMut(ArrayElementRef<'_>) -> ControlFlow<()>>,
        ) -> c_int {
            match (data)(elem) {
                ControlFlow::Continue(_) => 0,
                ControlFlow::Break(_) => -1,
            }
        }
        let mut callback = Box::new(f);
        let callback_ptr = (&mut callback as *mut Box<F>).cast();
        unsafe {
            external::ffi::array_walk(
                self,
                transmute(ArrayElementMapFn::new(walker)),
                callback_ptr,
            )
        }
    }
}

impl Array {
    #[must_use]
    pub fn new(element_count: c_long) -> Self {
        unsafe {
            let array = external::ffi::array_create();
            // external::ffi::array_resize(array.arr.ptr, element_count);
            array
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ref(&self) -> &FFIArray {
        self.arr.as_ref()
    }

    #[must_use]
    #[inline(always)]
    pub fn array_ref<'a>(&'a self) -> ArrayRef<'a> {
        self.arr.shorten_lifetime()
    }

    /// Clone calls this function, this is a deep copy.
    /// The reason this exists is because it's called
    /// `copy` in the bash API.
    #[must_use]
    #[inline(always)]
    pub fn copy(&self) -> Self {
        self.arr.copy()
    }

    #[inline(always)]
    pub fn dispose(self) {}

    #[inline(always)]
    pub fn forget(self) {
        core::mem::forget(self);
    }

    #[must_use]
    #[inline(always)]
    pub fn walk<'a, F: FnMut(ArrayElementRef<'a>) -> ControlFlow<()>>(&'a self, f: F) {
        self.array_ref().walk(f);
    }
}

impl Clone for Array {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.copy()
    }
}

impl Drop for Array {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            external::ffi::array_dispose(self.arr.ptr);
        }
    }
}