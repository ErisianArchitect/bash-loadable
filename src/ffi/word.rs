use std::{ffi::{CStr, c_char, c_int}, mem::transmute, ptr::NonNull};
//
use paste::paste;

// /* Possible values for the `flags' field of a WORD_DESC. */
// #define W_HASDOLLAR	(1 << 0)	/* Dollar sign present. */
// #define W_QUOTED	(1 << 1)	/* Some form of quote character is present. */
// #define W_ASSIGNMENT	(1 << 2)	/* This word is a variable assignment. */
// #define W_SPLITSPACE	(1 << 3)	/* Split this word on " " regardless of IFS */
// #define W_NOSPLIT	(1 << 4)	/* Do not perform word splitting on this word because ifs is empty string. */
// #define W_NOGLOB	(1 << 5)	/* Do not perform globbing on this word. */
// #define W_NOSPLIT2	(1 << 6)	/* Don't split word except for $@ expansion (using spaces) because context does not allow it. */
// #define W_TILDEEXP	(1 << 7)	/* Tilde expand this assignment word */
// #define W_DOLLARAT	(1 << 8)	/* UNUSED - $@ and its special handling */
// #define W_ARRAYREF	(1 << 9)	/* word is a valid array reference */
// #define W_NOCOMSUB	(1 << 10)	/* Don't perform command substitution on this word */
// #define W_ASSIGNRHS	(1 << 11)	/* Word is rhs of an assignment statement */
// #define W_NOTILDE	(1 << 12)	/* Don't perform tilde expansion on this word */
// #define W_NOASSNTILDE	(1 << 13)	/* don't do tilde expansion like an assignment statement */
// #define W_EXPANDRHS	(1 << 14)	/* Expanding word in ${paramOPword} */
// #define W_COMPASSIGN	(1 << 15)	/* Compound assignment */
// #define W_ASSNBLTIN	(1 << 16)	/* word is a builtin command that takes assignments */
// #define W_ASSIGNARG	(1 << 17)	/* word is assignment argument to command */
// #define W_HASQUOTEDNULL	(1 << 18)	/* word contains a quoted null character */
// #define W_DQUOTE	(1 << 19)	/* UNUSED - word should be treated as if double-quoted */
// #define W_NOPROCSUB	(1 << 20)	/* don't perform process substitution */
// #define W_SAWQUOTEDNULL	(1 << 21)	/* word contained a quoted null that was removed */
// #define W_ASSIGNASSOC	(1 << 22)	/* word looks like associative array assignment */
// #define W_ASSIGNARRAY	(1 << 23)	/* word looks like a compound indexed array assignment */
// #define W_ARRAYIND	(1 << 24)	/* word is an array index being expanded */
// #define W_ASSNGLOBAL	(1 << 25)	/* word is a global assignment to declare (declare/typeset -g) */
// #define W_NOBRACE	(1 << 26)	/* Don't perform brace expansion */
// #define W_COMPLETE	(1 << 27)	/* word is being expanded for completion */
// #define W_CHKLOCAL	(1 << 28)	/* check for local vars on assignment */
// #define W_FORCELOCAL	(1 << 29)	/* force assignments to be to local variables, non-fatal on assignment errors */

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FFIWordFlags(c_int);

impl FFIWordFlags {
    pub const NONE: Self = Self(0);

    #[must_use]
    #[inline(always)]
    pub const fn has_flags(self, flags: Self) -> bool {
        self.0 & flags.0 == flags.0
    }

    #[inline(always)]
    pub const fn add_flags(&mut self, flags: Self) {
        self.0 |= flags.0;
    }

    #[inline(always)]
    pub const fn remove_flags(&mut self, flags: Self) {
        self.0 &= !flags.0;
    }

    #[inline(always)]
    pub const fn toggle_flags(&mut self, flags: Self) {
        self.0 ^= flags.0;
    }

    #[must_use]
    #[inline(always)]
    pub const fn with_flags(mut self, flags: Self) -> Self {
        self.add_flags(flags);
        self
    }

    #[must_use]
    #[inline(always)]
    pub const fn without_flags(mut self, flags: Self) -> Self {
        self.remove_flags(flags);
        self
    }

    #[must_use]
    #[inline]
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    #[inline]
    pub const fn pop_bottom_index(&mut self) -> Option<u32> {
        if self.0 == 0 {
            return None;
        }
        let mut mask = self.0.cast_unsigned();
        let next_bit = mask.trailing_zeros();
        mask ^= 1 << next_bit;
        self.0 = mask.cast_signed();
        Some(next_bit)
    }

    #[must_use]
    #[inline]
    pub fn get_flags(self) -> Box<[(Self, &'static str)]> {
        self.collect()
    }
}

impl Iterator for FFIWordFlags {
    type Item = (FFIWordFlags, &'static str);
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count_ones() as usize, Some(self.count_ones() as usize))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.pop_bottom_index()?;
        let index = index as usize;
        Some((Self::ALL_FLAGS[index], Self::FLAG_NAMES[index]))
    }
}

macro_rules! define_word_flags {
    (
        $(
            #[$doc_meta:meta]
            $const_name:ident $func_name:ident = $value:expr
        ),*
        $(,)
    ?) => {
        paste!{
            impl FFIWordFlags {
                pub const ALL: Self = {
                    let mut builder = Self::NONE;
                    $(
                        builder.add_flags(Self::$const_name);
                    )*
                    builder
                };
                pub const ALL_FLAGS: &'static [Self] = &[
                    $(
                        Self::$const_name,
                    )*
                ];
                pub const FLAG_NAMES: &'static [&'static str] = &[
                    $(
                        stringify!($func_name),
                    )*
                ];
                $(
                    #[$doc_meta]
                    pub const $const_name: Self = Self($value);
                    
                    #[must_use]
                    #[inline(always)]
                    pub const fn [<get_ $func_name>](self) -> bool {
                        self.has_flags(Self::$const_name)
                    }

                    #[inline(always)]
                    pub const fn [<add_ $func_name>](&mut self) {
                        self.add_flags(Self::$const_name);
                    }

                    #[inline(always)]
                    pub const fn [<remove_ $func_name>](&mut self) {
                        self.remove_flags(Self::$const_name);
                    }

                    #[inline]
                    pub const fn [<set_ $func_name>](&mut self, on: bool) {
                        if on {
                            self.add_flags(Self::$const_name);
                        } else {
                            self.remove_flags(Self::$const_name);
                        }
                    }

                    #[inline(always)]
                    pub const fn [<toggle_ $func_name>](&mut self) {
                        self.0 ^= Self::$const_name.0;
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn [<with_ $func_name>](self) -> Self {
                        self.with_flags(Self::$const_name)
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn [<without_ $func_name>](self) -> Self {
                        self.without_flags(Self::$const_name)
                    }
                )*
            }
        }
    };
}

define_word_flags!(
    /// Dollar sign present.
    HAS_DOLLAR
        has_dollar              = 1 << 0,
    /// Some form of quoted character is present.
    QUOTED
        quoted                  = 1 << 1,
    /// This word is a variable assignment.
    ASSIGNMENT
        assignment              = 1 << 2,
    /// Split this word on " " regardless of IFS.
    SPLIT_SPACE
        split_space             = 1 << 3,
    /// Do not perform word splitting on this word because IFS is empty string.
    NO_SPLIT
        no_split                = 1 << 4,
    /// Do not perform globbing on this word.
    NO_GLOB
        no_glob                 = 1 << 5,
    /// Don't split word except for $@ expansion (using spaces) because context does not allow it.
    NO_SPLIT2
        no_split2               = 1 << 6,
    /// Tilde expand this assignment word.
    TILDE_EXP
        tilde_exp               = 1 << 7,
    /// $@ and its special handling. (Unused)
    DOLLAR_AT
        dollar_at               = 1 << 8,
    /// Word is a valid array reference.
    ARRAY_REF
        array_ref               = 1 << 9,
    /// Don't perform command substitution on this word.
    NO_COMMAND_SUBSTITUTION
        no_command_substitution = 1 << 10,
    /// Word is RHS of an assignment statement.
    ASSIGN_RHS
        assign_rhs              = 1 << 11,
    /// Don't perform tilde expansion on this word.
    NO_TILDE
        no_tilde                = 1 << 12,
    /// Don't do tilde expansion like an assignment statement.
    NO_ASSIGN_TILDE
        no_assign_tilde         = 1 << 13,
    /// Expanding word in ${paramOPword}
    EXPAND_RHS
        expand_rhs              = 1 << 14,
    /// Compound assignment. (no idea what that means, better look it up.) // TODO
    COMPOUND_ASSIGNMENT
        compound_assignment     = 1 << 15,
    /// Word is a builtin command that takes assignments
    ASSIGN_BUILTIN
        assign_builtin          = 1 << 16,
    /// Word is assignment argument to command.
    ASSIGN_ARG
        assign_arg              = 1 << 17,
    /// Word contains a quoted null character.
    HAS_QUOTED_NULL
        has_quoted_null         = 1 << 18,
    /// Word should be treated as if double-quoted. (Unused)
    DOUBLE_QUOTE
        double_quote            = 1 << 19,
    /// Don't perform process substitution.
    NO_PROCESS_SUBSTITUTION
        no_process_substitution = 1 << 20,
    /// Word contained a quoted null that was removed.
    SAW_QUOTED_NULL
        saw_quoted_null         = 1 << 21,
    /// Word looks like associative array assignment.
    ASSIGN_ASSOC
        assign_assoc            = 1 << 22,
    /// Word looks like a compound indexed array assignment.
    ASSIGN_ARRAY
        assign_array            = 1 << 23,
    /// Word is an array index being expanded.
    ARRAY_INDEX
        array_index             = 1 << 24,
    /// Word is a global assignment to declare
    ASSIGN_GLOBAL
        assign_global           = 1 << 25,
    /// Don't perform brace expansion
    NO_BRACE
        no_brace                = 1 << 26,
    /// Word is being expanded for completion.
    COMPLETION
        completion              = 1 << 27,
    /// Check for local vars on assignment.
    CHECK_LOCAL
        check_local             = 1 << 28,
    /// Force assignment to be local variables, non-fatal on assignment errors.]    
    FORCE_LOCAL
        force_local             = 1 << 29
);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FFIWord {
    pub word: *const c_char,
    pub flags: FFIWordFlags,
}

impl FFIWord {
    pub const EMTPY: Self = Self { word: c"".as_ptr(), flags: FFIWordFlags::NONE };
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct WordRef(Option<NonNull<FFIWord>>);

impl WordRef {
    #[must_use]
    #[inline(always)]
    pub const fn get(&self) -> Option<&FFIWord> {
        unsafe { transmute(self.0) }
    }

    #[must_use]
    #[inline]
    pub fn to_str(&self) -> Option<&str> {
        if let Some(word) = self.get() {
            if word.word.is_null() {
                return None;
            }
            let cstr = unsafe { CStr::from_ptr(word.word) };
            let len = cstr.count_bytes();
            Some(unsafe { transmute(std::slice::from_raw_parts(word.word.cast::<u8>(), len)) })
        } else {
            None
        }
    }
}

impl std::fmt::Display for WordRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = self.to_str() {
            write!(f, "{s}")
        } else {
            Ok(())
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FFIWordList {
    pub next: WordListRef,
    pub word: WordRef,
}

impl FFIWordList {
    const NULL: Self = Self { next: WordListRef(None), word: WordRef(None) };

    #[must_use]
    #[inline]
    pub const fn next(&self) -> Option<&FFIWordList> {
        self.next.get()
    }

    #[must_use]
    #[inline]
    pub const fn word(&self) -> Option<&FFIWord> {
        self.word.get()
    }

    #[must_use]
    #[inline]
    pub fn word_str(&self) -> Option<&str> {
        self.word.to_str()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct WordListRef(Option<NonNull<FFIWordList>>);

impl WordListRef {
    #[must_use]
    #[inline(always)]
    pub const fn get(&self) -> Option<&FFIWordList> {
        unsafe { transmute(self.0) }
    }

    #[must_use]
    #[inline]
    pub const fn as_ref(&self) -> &FFIWordList {
        if let Some(inner) = self.get() {
            inner
        } else {
            &FFIWordList::NULL
        }
    }
}