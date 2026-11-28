use std::{cell::UnsafeCell, ffi::{
    CStr, c_char, c_int, c_void,
}};

use std::{
    ptr::{
        NonNull,
    }
};

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
            function: Some(self.function),
            flags: flags.with_enabled(),
            long_doc: self.long_doc,
            short_doc: self.short_doc,
            handle: None,
        }))
    }

    #[must_use]
    #[inline]
    pub const fn build(self) -> Builtin {
        self.build_with_flags(BuiltinFlags::ENABLED)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BuiltinInner {
    name: *const c_char,
    function: Option<BuiltinFn>,
    flags: BuiltinFlags,
    long_doc: *const *const c_char,
    short_doc: *const c_char,
    // The handle to the dynamic library that the builtin
    // is present in.
    handle: Option<DlHandle>,
}

impl BuiltinInner {
    #[must_use]
    pub fn get_name(&self) -> &str {
        from_cstr(self.name).unwrap_or("")
    }

    #[must_use]
    #[inline(always)]
    pub fn get_function(&self) -> Option<BuiltinFn> {
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

    #[must_use]
    #[inline(always)]
    pub fn get_handle(&self) -> Option<DlHandle> {
        self.handle
    }
}

#[repr(transparent)]
pub struct Builtin(UnsafeCell<BuiltinInner>);

impl Builtin {
    fn get_inner(&self) -> &BuiltinInner {
        unsafe { self.0.get().as_ref_unchecked() }
    }

    pub fn name(&self) -> &str {
        if self.get_inner().name.is_null() {
            return "";
        }
        let cstr = unsafe { CStr::from_ptr(self.get_inner().name) };
        let len = cstr.count_bytes();
        return unsafe {
            str::from_utf8_unchecked(
                ::core::slice::from_raw_parts(cstr.as_ptr().cast::<u8>(), len)
            )
        };
    }
    
    pub fn get_handle(&self) -> Option<DlHandle> {
        self.get_inner().get_handle()
    }

    pub fn dupe(&self) -> Self {
        Self(UnsafeCell::new(*self.get_inner()))
    }
}

unsafe impl Sync for Builtin {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DlHandle(NonNull<c_void>);

impl DlHandle {
    // This probably isn't safe to have here.
    // fn close(self) {
    //     unsafe {
    //         let ptr: *mut c_void = self.0.as_ptr();
    //         let result = libc::dlclose(ptr); const ERR_LINE: u32 = line!();
    //         if result < 0 {
    //             eprintln!("dlclose error ({}). {}:{}", -result, file!(), ERR_LINE);
    //         }
    //     }
    // }

    #[must_use]
    #[inline(always)]
    pub fn as_ptr(self) -> *mut c_void {
        return self.0.as_ptr()
    }
}

fn dlsym_handle<T>(sym: &T) -> Option<DlHandle> {
    unsafe {
        let mut info: libc::Dl_info = std::mem::zeroed();
        let result = libc::dladdr((sym as *const T).cast(), &mut info);
        if result < 0 {
            return None;
        }
        if info.dli_fname.is_null() {
            return None;
        }
        let handle = libc::dlopen(info.dli_fname, libc::RTLD_NOLOAD);
        if handle.is_null() {
            return None;
        }
        Some(DlHandle(NonNull::new_unchecked(handle)))
    }
}

pub unsafe fn get_builtins() -> &'static [Builtin] {
    use crate::external::ffi::*;
    let ptr = unsafe { shell_builtins };
    let len = unsafe { num_shell_builtins } as usize;
    if ptr.is_null() {
        return &[];
    }
    unsafe {
        ::core::slice::from_raw_parts(ptr, len)
    }
}

pub unsafe fn get_builtins_mut() -> &'static mut [Builtin] {
    use crate::external::ffi::*;
    let ptr = unsafe { shell_builtins };
    let len = unsafe { num_shell_builtins } as usize;
    if ptr.is_null() {
        return &mut [];
    }
    unsafe {
        ::core::slice::from_raw_parts_mut(ptr, len)
    }
}

#[must_use]
#[inline(always)]
fn is_static_shell_builtins() -> bool {
    use crate::external::ffi;
    unsafe {
        let shell_builtins = ffi::shell_builtins;
        let static_shell_builtins = ffi::static_shell_builtins;
        shell_builtins.cast_const() == static_shell_builtins
    }
}

pub fn find_builtin<'a>(name: &str) -> Option<&'a Builtin> {
    let builtins = unsafe { get_builtins() };
    match builtins.binary_search_by(|b| b.name().cmp(name)) {
        Ok(index) => Some(&builtins[index]),
        Err(_) => None,
    }
}

fn sort_builtins() {
    let builtins = unsafe { get_builtins_mut() };
    builtins.sort_by(|lhs, rhs| lhs.name().cmp(rhs.name()));
}

pub fn add_builtins(builtins: &[&Builtin]) {
    use crate::external::ffi;
    if builtins.len() == 0
    || is_static_shell_builtins() {
        return;
    }
    unsafe {
        let num_shell_builtins = ffi::num_shell_builtins;
        let shell_builtins = ffi::shell_builtins;
        let new_count = num_shell_builtins as usize + builtins.len();
        let new_shell_builtins = ffi::xmalloc(
            (new_count + 1) * size_of::<Builtin>()
        ).cast::<Builtin>();
        new_shell_builtins
            .offset(new_count as isize)
            .write(std::mem::zeroed());
        std::ptr::copy_nonoverlapping(
            shell_builtins,
            new_shell_builtins,
            num_shell_builtins as usize,
        );
        let mut write_index = num_shell_builtins as isize;
        for builtin in builtins.iter().copied() {
            let mut inner = *builtin.get_inner();
            inner.handle = dlsym_handle(&builtin);
            new_shell_builtins
                .offset(write_index)
                .write(Builtin(UnsafeCell::new(inner)));
            write_index += 1;
        }
        ffi::shell_builtins = new_shell_builtins;
        ffi::num_shell_builtins = new_count as c_int;
        if shell_builtins.cast_const() != ffi::static_shell_builtins {
            ffi::xfree(shell_builtins.cast());
        }
        sort_builtins();
    }
}
