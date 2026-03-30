
pub mod ffi {
    use core::ffi::{
        // CStr,
        c_void,
        c_char,
        c_int,
        c_long,
    };
    use crate::{ffi::{
        alias::Alias, array::{Array, ArrayElement, PFlags, ShiftElementFlags}, bash_owned::BashOwned, bash_str::BashStr, pattern::MatchFlags, var::ShellVar, word::{
            Word,
            WordList,
        }
    }, util::ffi::{BashStatus, CBool}};
    pub type ArrayElementMapFn = extern "C" fn(ArrayElement<'_>, data: *const ());

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
            flags: c_int,
        ) -> BashStatus;
        pub fn eval_string(
            eval: *const c_char,
            from_file: *const c_char,
            flags: c_int,
        ) -> BashStatus;
        pub fn parse_and_execute_cleanup(
            old_running_trap: c_int,
        );
        /// Returns the number of characters read or whatever.
        pub fn parse_string(
            source: *const c_char,
            from_file: *const c_char,
            flags: c_int,
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
        ) -> ShellVar<'a>;
        pub fn find_variable_noref<'a>(
            name: *const c_char,
        ) -> ShellVar<'a>;
        
        pub fn find_global_variable<'a>(
            name: *const c_char,
        ) -> ShellVar<'a>;
        pub fn find_global_variable_noref<'a>(
            name: *const c_char,
        ) -> ShellVar<'a>;
        pub fn find_shell_variable<'a>(
            name: *const c_char,
        ) -> ShellVar<'a>;
        pub fn find_tempenv_variable<'a>(
            name: *const c_char,
        ) -> ShellVar<'a>;

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
        ) -> ShellVar<'a>;

        pub fn bind_variable<'a>(
            name: *const c_char,
            value: *const c_char,
            flags: c_int,
        ) -> ShellVar<'a>;
        pub fn bind_global_variable<'a>(
            name: *const c_char,
            value: *const c_char,
            flags: c_int,
        ) -> ShellVar<'a>;
        pub fn bind_variable_value<'a>(
            var: ShellVar<'a>,
            value: *const c_char,
            flags: c_int,
        ) -> ShellVar<'a>;
        pub fn bind_int_value<'a>(
            var: ShellVar<'a>,
            value: *const c_char,
            flags: c_int,
        ) -> ShellVar<'a>;
        pub fn bind_var_to_int<'a>(
            var: *const c_char,
            value: c_long,
            flags: c_int,
        ) -> ShellVar<'a>;

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
        pub fn array_alloc(
            array: Array<'_>,
            n: c_long,
        );
        pub fn array_resize(
            array: Array<'_>,
            n: c_long,
        );
        pub fn array_expand(
            array: Array<'_>,
            n: c_long,
        );
        // TODO: Check if this needs to be BashOwned
        pub fn array_dispose_elements(
            elements: *const ArrayElement<'_>,
        );
        pub fn array_create<'a>(
        ) -> Array<'a>;
        pub fn array_flush(
            array: Array<'_>
        );
        pub fn array_dispose(
            array: Array<'_>,
        );
        pub fn array_copy<'a, 'b>(
            array: Array<'a>,
        ) -> Array<'b>;
        pub fn array_slice<'a, 'b>(
            array: Array<'a>,
            start: c_long,
            end: c_long,
        ) -> Array<'b>;
        pub fn array_walk(
            array: Array<'_>,
            map: ArrayElementMapFn,
            data: *const (),
        );
        // TODO: Check if this needs to be BashOwned
        pub fn array_shift<'a>(
            array: Array<'a>,
            n: c_int,
            flags: ShiftElementFlags,
        ) -> *const ArrayElement<'a>;
        pub fn array_rshift(
            array: Array<'_>,
            n: c_int,
            value: *const c_char,
        ) -> c_int;
        pub fn array_unshift_element<'a>(
            array: Array<'a>,
        ) -> ArrayElement<'a>;
        pub fn array_shift_element(
            array: Array<'_>,
            value: *const c_char,
        ) -> c_int;
        pub fn array_quote(
            array: Array<'_>,
        ) -> Array<'_>;
        pub fn array_quote_escapes(
            array: Array<'_>,
        ) -> Array<'_>;
        pub fn array_dequote(
            array: Array<'_>,
        ) -> Array<'_>;
        pub fn array_dequote_escapes(
            array: Array<'_>,
        ) -> Array<'_>;
        pub fn array_remove_quoted_nulls(
            array: Array<'_>,
        ) -> Array<'_>;
        pub fn array_subrange(
            array: Array<'_>,
            start: c_long,
            nelem: c_long,
            starsub: c_int,
            quoted: c_int,
            flags: PFlags,
        ) -> Option<BashStr>;
        pub fn array_patsub(
            array: Array<'_>,
            pattern: *const c_char,
            rep: *const c_char,
            flags: MatchFlags,
        ) -> Option<BashStr>;
        pub fn array_modcase(
            array: Array<'_>,
            pattern: *const c_char,
            modop: c_int,
            flags: MatchFlags,
        ) -> Option<BashStr>;
        pub fn array_create_element<'a>(
            index: c_long,
            value: *const c_char,
        ) -> ArrayElement<'a>;
        pub fn array_copy_element<'a, 'b>(
            element: ArrayElement<'a>,
        ) -> ArrayElement<'b>;
        pub fn array_dispose_element(
            element: ArrayElement<'_>,
        );
        pub fn array_insert(
            array: Array<'_>,
            index: c_long,
            value: *const c_char,
        ) -> c_int;
        pub fn array_remove<'a>(
            array: Array<'a>,
        ) -> ArrayElement<'a>;
        /// return value is owned by bash.
        pub fn array_reference(
            array: Array<'_>,
            index: c_long,
        ) -> *const c_char;
        pub fn array_to_wordlist(
            array: Array<'_>,
        ) -> WordList;
        pub fn array_from_wordlist<'a>(
            words: WordList,
        ) -> Array<'a>;
        pub fn array_keys_to_word_list(
            array: Array<'_>,
        ) -> WordList;
        pub fn array_to_kvpair_list(
            array: Array<'_>,
        ) -> WordList;
        pub fn array_assign_list(
            array: Array<'_>,
            words: WordList,
        ) -> Array<'_>;
        // TODO: Investigate memory management of return value.
        // pub fn array_to_argv(
        //     array: Array<'_>,
        //     count: &mut c_int,
        // ) -> BashOwned<*const c_char>;
        pub fn array_from_argv(
            array: Array<'_>,
            argv: *const *const c_char,
            count: c_int,
        ) -> Array<'_>;
        pub fn array_to_kvpair(
            array: Array<'_>,
            quoted: CBool,
        ) -> Option<BashStr>;
        pub fn array_to_assign(
            array: Array<'_>,
        ) -> Option<BashStr>;
        pub fn array_to_string(
            array: Array<'_>,
            sep: *const c_char,
            quoted: CBool,
        ) -> Option<BashStr>;
        pub fn array_from_string<'a>(
            s: *const c_char,
            sep: *const c_char,
        ) -> Array<'a>;

        // externs.h

        pub fn evalexp(
            expr: *const c_char,
            flags: c_int,
            validp: &mut c_int,
        ) -> c_long;
        
    }
}