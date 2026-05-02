
pub mod ffi {
    use core::ffi::{
        // CStr,
        c_void,
        c_char,
        c_int,
        c_long,
    };
    use std::ptr::NonNull;
    use crate::{ffi::{
        alias::Alias, array::{Array, ArrayElement, ArrayElementMapFn, ArrayRef, FFIArray, FFIArrayElement, PFlags, ShiftElementFlags}, bash_owned::BashOwned, bash_str::BashStr, eval::EvalFlags, pattern::MatchFlags, strvec::{StrVec, StrVecRef}, var::ShellVar, word::{
            Word,
            WordList,
        }
    }, util::ffi::{BashStatus, CBool}};

    // pub type ArrayElementMapFn<T> = extern "C" fn(ArrayElementRef<'_>, data: &mut T);

    unsafe extern "C" {
        // xmalloc.h
        pub fn xmalloc(
            size: usize,
        ) -> *mut ();
        pub fn xrealloc(
            ptr: *mut (),
            new_size: usize,
        ) -> *mut ();
        pub fn xreallocarray(
            ptr: *mut (),
            elem_count: usize,
            elem_size: usize,
        ) -> *mut ();
        pub fn xfree(
            ptr: *mut (),
        );

        // common.h
        pub fn remember_args(
            words: WordList,
            destructive: CBool,
        );

        // command.h
        pub fn copy_word(
            word: Word,
        ) -> Word;
        pub fn copy_word_list(
            words: WordList,
        ) -> WordList;

        // make_cmd.h
        pub fn alloc_word_desc(
        ) -> Word;
        pub fn make_bare_word(
            word: *const c_char
        ) -> Word;
        pub fn make_word_flags(
            word: Word,
            string: *const c_char,
        ) -> Word;
        pub fn make_word(
            word: *const c_char,
        ) -> Word;
        pub fn make_word_list(
            word: Word,
            head: WordList,
        ) -> WordList;
    
        // dispose_cmd.h
        pub fn dispose_word(
            word: Word,
        );
        pub fn dispose_words(
            list: WordList,
        );
    
        // Current working directory
        pub fn get_working_directory(
            for_whom: *const c_char,
        ) -> *const c_char;
        pub fn set_working_directory(
            name: *const c_char,
        );
        // evalstring.c
        pub fn parse_and_execute(
            source: *const c_char,
            from_file: *const c_char,
            flags: EvalFlags,
        ) -> BashStatus;
        pub fn evalstring(
            eval: *const c_char,
            from_file: *const c_char,
            flags: EvalFlags,
        ) -> BashStatus;
        pub fn parse_and_execute_cleanup(
            old_running_trap: c_int,
        );
        /// Returns the number of characters read or whatever.
        pub fn parse_string(
            source: *const c_char,
            from_file: *const c_char,
            flags: EvalFlags,
            cmdp: *const *const c_void,
            endp: *const *const c_char,
        ) -> c_int;
    
        // evalfile.c
        pub fn maybe_execute_file(
            filename: *const c_char,
            force_noninteractive: c_int,
        ) -> BashStatus;
        pub fn force_execute_file(
            filename: *const c_char,
            force_noninteractive: c_int,
        ) -> BashStatus;
        pub fn source_file(
            filename: *const c_char,
            sflags: c_int,
        ) -> BashStatus;

        // variables.h
        pub fn find_variable<'a>(
            name: *const c_char,
        ) -> Option<ShellVar<'a>>;
        pub fn find_variable_noref<'a>(
            name: *const c_char,
        ) -> Option<ShellVar<'a>>;
        
        pub fn find_global_variable<'a>(
            name: *const c_char,
        ) -> Option<ShellVar<'a>>;
        pub fn find_global_variable_noref<'a>(
            name: *const c_char,
        ) -> Option<ShellVar<'a>>;
        pub fn find_shell_variable<'a>(
            name: *const c_char,
        ) -> Option<ShellVar<'a>>;
        pub fn find_tempenv_variable<'a>(
            name: *const c_char,
        ) -> Option<ShellVar<'a>>;

        pub fn get_variable_value(
            var: ShellVar<'_>,
        ) -> *const c_char;
        pub fn get_string_value(
            name: *const c_char,
        ) -> *const c_char;

        pub fn copy_variable<'a, 'b>(
            var: ShellVar<'a>,
        ) -> ShellVar<'b>;

        pub fn make_local_variable<'a>(
            name: *const c_char,
            flags: c_int,
        ) -> Option<ShellVar<'a>>;

        pub fn bind_variable<'a>(
            name: *const c_char,
            value: *const c_char,
            flags: c_int,
        ) -> Option<ShellVar<'a>>;
        pub fn bind_global_variable<'a>(
            name: *const c_char,
            value: *const c_char,
            flags: c_int,
        ) -> Option<ShellVar<'a>>;
        pub fn bind_variable_value<'a>(
            var: ShellVar<'a>,
            value: *const c_char,
            flags: c_int,
        ) -> Option<ShellVar<'a>>;
        pub fn bind_int_value<'a>(
            var: ShellVar<'a>,
            value: *const c_char,
            flags: c_int,
        ) -> Option<ShellVar<'a>>;
        pub fn bind_var_to_int<'a>(
            var: *const c_char,
            value: c_long,
            flags: c_int,
        ) -> Option<ShellVar<'a>>;

        pub fn unbind_variable(
            name: *const c_char,
        ) -> BashStatus;
        pub fn check_unbind_variable(
            name: *const c_char,
        ) -> BashStatus;
        pub fn unbind_nameref(
            name: *const c_char,
        ) -> BashStatus;
        pub fn unbind_variable_noref(
            name: *const c_char,
        ) -> BashStatus;

        pub fn dispose_variable(
            var: ShellVar<'_>,
        );

        

        // alias.h
        // TODO: Owned Alias(?) and AliasRef<'_>
        pub fn find_alias(
            name: *const c_char,
        ) -> *const Alias; // NOTE: Alias ref
        pub fn get_alias_value(
            name: *const c_char,
        ) -> *const c_char; // C-str ref
        pub fn add_alias(
            name: *const c_char,
            value: *const c_char,
        );
        pub fn remove_alias(
            name: *const c_char,
        ) -> c_int;
        /// This is probably a bad idea to use, but I've included it anyway.
        pub fn delete_all_aliases(
        ); // is sad
        // TODO: *const *const Alias? Really??
        pub fn all_aliases(
        ) -> *const *const Alias; // Must free yourself. Not sure if there's a way in bash source code, will try to find it.
        pub fn alias_expand_word(
            s: *const c_char,
        ) -> Option<BashOwned<c_char>>;
        pub fn alias_expand(
            s: *const c_char,
        ) -> Option<BashOwned<c_char>>;
        // TODO: Continue working here.
        pub fn clear_string_list_expander(
            alias: *const Alias,
        );

        // array.h
        // pub fn array_alloc(
        //     array: NonNull<FFIArray>,
        //     n: c_long,
        // );
        // pub fn array_resize(
        //     array: NonNull<FFIArray>,
        //     n: c_long,
        // );
        // pub fn array_expand(
        //     array: NonNull<FFIArray>,
        //     n: c_long,
        // );
        // TODO: array_expand_index, array_expand_once
        // TODO: Check if this needs to be BashOwned
        // pub fn array_dispose_elements(
        //     elements: *const Option<NonNull<FFIArrayElement>>,
        // );
        pub fn array_create(
        ) -> Array;
        pub fn array_flush(
            array: NonNull<FFIArray>,
        );
        pub fn array_dispose(
            array: NonNull<FFIArray>,
        );
        pub fn array_copy(
            array: NonNull<FFIArray>,
        ) -> Array;
        // TODO: Does this need owned?? Probably not.
        pub fn array_slice(
            array: ArrayRef<'_>,
            start: c_long,
            end: c_long,
        ) -> Array;
        // TODO: array_value(???), array_variable_name(???), array_variable_part(???)
        // TODO: Does this need owned??
        pub fn array_walk(
            array: ArrayRef<'_>,
            map: ArrayElementMapFn<()>,
            data: *mut (),
        );
        // TODO: Here is where I left off (going down)
        // TODO: Check if this needs to be BashOwned
        // TODO: Does this need owned??
        pub fn array_shift(
            array: Array,
            n: c_int,
            flags: ShiftElementFlags,
        ) -> *const ArrayElement;
        // TODO: Does this need owned??
        pub fn array_rshift(
            array: Array,
            n: c_int,
            value: *const c_char,
        ) -> c_int;
        // TODO: Does this need owned??
        pub fn array_unshift_element(
            array: Array,
        ) -> ArrayElement;
        // pub fn array_shift_element(
        //     array: Array,
        //     value: *const c_char,
        // ) -> c_int;
        // TODO: Does this need owned??
        pub fn array_quote(
            array: Array,
        ) -> Array;
        // TODO: Does this need owned??
        pub fn array_quote_escapes(
            array: Array,
        ) -> Array;
        pub fn array_dequote(
            array: Array,
        ) -> Array;
        pub fn array_dequote_escapes(
            array: Array,
        ) -> Array;
        // TODO: Does this need owned??
        pub fn array_remove_quoted_nulls(
            array: Array,
        ) -> Array;
        // TODO: Does this need owned??
        pub fn array_subrange(
            array: Array,
            start: c_long,
            nelem: c_long,
            starsub: c_int,
            quoted: c_int,
            flags: PFlags,
        ) -> Option<BashStr>;
        // TODO: Does this need owned??
        pub fn array_patsub(
            array: Array,
            pattern: *const c_char,
            rep: *const c_char,
            flags: MatchFlags,
        ) -> Option<BashStr>;
        // TODO: Does this need owned??
        pub fn array_modcase(
            array: Array,
            pattern: *const c_char,
            modop: c_int,
            flags: MatchFlags,
        ) -> Option<BashStr>;
        pub fn array_create_element(
            index: c_long,
            value: *const c_char,
        ) -> Option<ArrayElement>;
        // pub fn array_copy_element(
        //     element: NonNull<FFIArrayElement>,
        // ) -> Option<ArrayElement>;
        pub fn array_dispose_element(
            element: NonNull<FFIArrayElement>,
        );
        pub fn array_insert(
            array: ArrayRef<'_>,
            index: c_long,
            value: *const c_char,
        ) -> c_int;
        // TODO: Does this need owned??
        pub fn array_remove(
            array: Array,
        ) -> ArrayElement;
        // /// return value is owned by bash.
        // TODO: Does this need owned??
        pub fn array_reference(
            array: ArrayRef<'_>,
            index: c_long,
        ) -> *const c_char;
        // TODO: Does this need owned??
        pub fn array_to_word_list(
            array: Array,
        ) -> WordList;
        pub fn array_from_wordlist(
            words: WordList,
        ) -> Array;
        // TODO: Does this need owned??
        pub fn array_keys_to_word_list(
            array: Array,
        ) -> WordList;
        // TODO: Does this need owned??
        pub fn array_to_kvpair_list(
            array: Array,
        ) -> WordList;
        pub fn array_assign_list(
            array: Array,
            words: WordList,
        ) -> Array;
        // // TODO: Investigate memory management of return value.
        // TODO: Does this need owned??
        pub fn array_to_argv(
            array: ArrayRef<'_>,
            count: &mut c_int,
        ) -> BashOwned<*const c_char>;
        pub fn array_from_argv(
            array: ArrayRef<'_>,
            argv: *const *const c_char,
            count: c_int,
        ) -> Array;
        // TODO: Does this need owned?? Check inputs/outputs
        pub fn array_to_kvpair(
            array: Array,
            quoted: CBool,
        ) -> Option<BashStr>;
        // TODO: Does this need owned??
        pub fn array_to_assign(
            array: Array,
        ) -> Option<BashStr>;
        // TODO: Does this need owned??
        pub fn array_to_string(
            array: Array,
            sep: *const c_char,
            quoted: CBool,
        ) -> Option<BashStr>;
        // pub fn array_from_string(
        //     s: *const c_char,
        //     sep: *const c_char,
        // ) -> Array;

        // externs.h

        pub fn evalexp(
            expr: *const c_char,
            flags: c_int,
            validp: &mut c_int,
        ) -> c_long;
        
        // externs.h :: strvec

        pub fn strvec_create(
            size: usize,
        ) -> StrVec;
        pub fn strvec_resize(
            vec: StrVecRef<'_>,
            new_size: usize,
        ) -> StrVec;
        // This is used internally by bash
        // pub fn strvec_flush(
        //     vec: StrVecRef<'_>,
        // );
        pub fn strvec_dispose(
            // this is expected to invalidate the memory
            // of the StrVec, and you would think it shoud
            // be called on the StrVec itself, but `drop`
            // doesn't take an owned value, it takes a
            // mutable reference. Good thing this function
            // requires an unsafe block!
            vec: StrVecRef<'_>,
        );
        pub fn strvec_remove(
            vec: StrVecRef<'_>,
            // In bash, this parameter is `name`, but I believe that's an oddity.
            // It's definitely removing the value within the array. I think these
            // strvecs may be used to store names? I don't know.
            // TODO: This should be a bash string of some sort. It should not be an owned variant.
            value: *const c_char,
        ) -> BashStatus;
        pub fn strvec_len(
            vec: StrVecRef<'_>,
        ) -> usize;
        pub fn strvec_search(
            vec: StrVecRef<'_>,
            value: *const c_char,
        ) -> isize;
        pub fn strvec_copy(
            vec: StrVecRef<'_>,
        ) -> StrVec;
        // This is used internally by bash
        // pub fn strvec_posixcmp(
        //     lhs: StrVecRef<'_>,
        //     rhs: StrVecRef<'_>,
        // ) -> c_int;
        // This is used internally by bash
        // pub fn strvec_strcmp(
        //     lhs: StrVecRef<'_>,
        //     rhs: StrVecRef<'_>,
        // ) -> c_int;
        pub fn strvec_sort(
            vec: StrVecRef<'_>,
            posix: CBool,
        );
        // TODO: Continue after strvec_sort in externs.h when you redo word.rs
    }
}