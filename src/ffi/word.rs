use std::{ffi::{CStr, c_char, c_int}, marker::PhantomData, mem::transmute, ptr::NonNull};
use crate::{
    macros::{
        cenum,
    },
    ffi::external::{self, ffi::{
        dispose_word, dispose_words, make_bare_word, make_word, make_word_flags, make_word_list
    }}, util::{self, ffi::to_cstr}
};

cenum!(
    pub enum WordFlags {
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
    }
);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FFIWord<'a> {
    pub word: *const c_char,
    pub flags: WordFlags,
    _phantom: PhantomData<&'a CStr>,
}

impl<'a> FFIWord<'a> {
    pub const EMTPY: FFIWord<'static> = FFIWord::new(c"".as_ptr(), WordFlags::NONE);

    #[must_use]
    #[inline(always)]
    pub const fn new(word: *const c_char, flags: WordFlags) -> Self {
        Self {
            word,
            flags,
            _phantom: PhantomData,
        }
    }
}

// #[repr(transparent)]
// #[derive(Clone, Copy)]
// pub struct WordRef(Option<NonNull<FFIWord>>);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Word<'a> {
    word: Option<&'a FFIWord<'a>>,
}

impl<'a> Word<'a> {
    #[must_use]
    #[inline(always)]
    pub const fn new(word: Option<&'a FFIWord<'a>>) -> Self {
        Self {
            word,
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn get(&self) -> Option<&'a FFIWord<'a>> {
        self.word
    }

    #[must_use]
    #[inline]
    pub fn to_str(&self) -> Option<&'a str> {
        if let Some(word) = self.get() {
            if word.word.is_null() {
                return None;
            }
            let cstr = unsafe { CStr::from_ptr(word.word) };
            let len = cstr.count_bytes();
            Some(unsafe { transmute(std::slice::from_raw_parts(word.word, len)) })
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub fn to_pair(&self) -> Option<(&'a str, WordFlags)> {
        if let Some(word) = self.get() {
            if word.word.is_null() {
                return None;
            }
            let cstr = unsafe { CStr::from_ptr(word.word) };
            let len = cstr.count_bytes();
            let s: &'a str = unsafe { transmute(core::slice::from_raw_parts(word.word.cast::<u8>(), len)) };
            Some((s, word.flags))
        } else {
            None
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn copy<'b>(self) -> Word<'b> {
        unsafe { external::ffi::copy_word(self) }
    }

    #[inline(always)]
    pub fn dispose(self) {
        unsafe { external::ffi::dispose_word(self); }
    }
}

impl<'a> std::fmt::Display for Word<'a> {
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
pub struct FFIWordList<'a> {
    pub next: WordList<'a>,
    pub word: Word<'a>,
}

impl<'a> FFIWordList<'a> {
    pub const NULL: FFIWordList<'static> = FFIWordList { next: WordList(None), word: Word::new(None) };

    #[must_use]
    #[inline]
    pub const fn next_raw(&self) -> Option<NonNull<FFIWordList<'a>>> {
        self.next.0
    }

    #[must_use]
    #[inline]
    pub const fn next(&self) -> Option<&'a FFIWordList<'a>> {
        self.next.get()
    }

    #[must_use]
    #[inline]
    pub const fn next_mut(&mut self) -> Option<&'a mut FFIWordList<'a>> {
        self.next.get_mut()
    }

    #[must_use]
    #[inline]
    pub const fn word(&self) -> Option<&'a FFIWord<'a>> {
        self.word.get()
    }

    #[must_use]
    #[inline]
    pub fn word_str(&self) -> Option<&'a str> {
        self.word.to_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WordKind<'a> {
    Bare(&'a str),
    Bash(&'a str),
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct WordList<'a>(Option<NonNull<FFIWordList<'a>>>);

impl<'a> WordList<'a> {
    pub const EMPTY: Self = Self(None);
    #[must_use]
    #[inline(always)]
    pub const fn get(&self) -> Option<&'a FFIWordList<'a>> {
        unsafe { transmute(self.0) }
    }

    #[must_use]
    #[inline(always)]
    pub const fn get_mut(&mut self) -> Option<&'a mut FFIWordList<'a>> {
        unsafe { transmute(self.0) }
    }

    #[must_use]
    pub fn new(words: &[WordKind]) -> Self {
        let mut tail = WordList(None);
        for &word in words.into_iter().rev() {
            match word {
                WordKind::Bare(word) => tail.prepend_bare_word(word),
                WordKind::Bash(word) => tail.prepend_word(word),
            }
        }
        tail
    }

    #[must_use]
    pub fn new_bash(words: &[&str]) -> Self {
        let mut tail = WordList(None);
        for &word in words.into_iter().rev() {
            tail.prepend_word(word);
        }
        tail
    }

    #[must_use]
    pub fn new_bare(bare_words: &[&str]) -> Self {
        let mut tail = WordList(None);
        for &word in bare_words.into_iter().rev() {
            tail.prepend_bare_word(word);
        }
        tail
    }

    #[inline(always)]
    fn internal_prepend(&mut self, word_kind: WordKind<'_>) {
        let ffi_word = match word_kind {
            WordKind::Bare(word) => {
                let word_cstr = to_cstr(word);
                unsafe { external::ffi::make_bare_word(word_cstr.as_ptr()) }
            },
            WordKind::Bash(word) => {
                let word_cstr = to_cstr(word);
                unsafe { external::ffi::make_word(word_cstr.as_ptr()) }
            },
        };
        *self = unsafe { external::ffi::make_word_list(ffi_word, *self) };
    }

    pub fn prepend(&mut self, word_kind: WordKind<'_>) {
        self.internal_prepend(word_kind);
    }

    #[inline(always)]
    pub fn prepend_bare_word(&mut self, word: &str) {
        self.internal_prepend(WordKind::Bare(word))
    }

    #[inline(always)]
    pub fn prepend_word(&mut self, word: &str) {
        self.internal_prepend(WordKind::Bash(word))
    }

    fn internal_append(&mut self, word_kind: WordKind<'_>) {
        let ffi_word = match word_kind {
            WordKind::Bare(word) => {
                let word_cstr = to_cstr(word);
                unsafe { external::ffi::make_bare_word(word_cstr.as_ptr()) }
            },
            WordKind::Bash(word) => {
                let word_cstr = to_cstr(word);
                unsafe { external::ffi::make_word(word_cstr.as_ptr()) }
            },
        };
        let next = unsafe { external::ffi::make_word_list(ffi_word, WordList::EMPTY) };
        if let Some(mut tail) = self.0 {
            loop {
                let tail_ref = unsafe { tail.as_ref() };
                if let Some(next_node) = tail_ref.next_raw() {
                    tail = next_node;
                } else {
                    break;
                }
            }
            let tail_mut = unsafe { tail.as_mut() };
            tail_mut.next = next;
        } else {
            *self = next;
        }
    }

    pub fn append(&mut self, word_kind: WordKind<'_>) {
        self.internal_append(word_kind);
    }

    #[inline(always)]
    pub fn append_bare_word(&mut self, word: &str) {
        self.internal_append(WordKind::Bare(word));
    }

    #[inline(always)]
    pub fn append_word(&mut self, word: &str) {
        self.internal_append(WordKind::Bash(word));
    }

    pub fn remember(self, destructive: bool) {
        let destructive = util::ffi::CBool::from_bool(destructive);
        unsafe { external::ffi::remember_args(self, destructive); }
    }

    #[must_use]
    #[inline(always)]
    pub fn copy<'b>(self) -> WordList<'b> {
        unsafe { external::ffi::copy_word_list(self) }
    }

    #[inline(always)]
    pub fn dispose(self) {
        unsafe { external::ffi::dispose_words(self); }
    }
}

impl<'a> Iterator for WordList<'a> {
    type Item = (&'a str, WordFlags);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.get()?;
        *self = WordList(unsafe { transmute(current.next()) });
        current.word.to_pair()
    }
}