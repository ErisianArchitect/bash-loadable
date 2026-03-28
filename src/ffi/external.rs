
pub mod ffi {
    use core::ffi::{
        // CStr,
        c_void,
        c_char,
        c_int,
        c_long,
    };
    use crate::ffi::{
        word::{
            FFIWordFlags,
            FFIWord,
            FFIWordList,
            WordRef,
            WordListRef,
        },
        var::{
            ShellVar,
        }
    };
    unsafe extern "C" {
        // common.h
        pub fn remember_args(words: WordListRef<'_>, destructive: c_int);

        // command.h
        pub fn copy_word<'a, 'b>(word: WordRef<'b>) -> WordRef<'a>;
        pub fn copy_word_list<'a, 'b>(words: WordListRef<'b>) -> WordListRef<'a>;

        // make_cmd.h
        pub fn alloc_word_desc<'a>() -> WordRef<'a>;
        pub fn make_bare_word<'a>(word: *const c_char) -> WordRef<'a>;
        pub fn make_word_flags<'a>(word: WordRef<'a>, string: *const c_char) -> WordRef<'a>;
        pub fn make_word<'a>(word: *const c_char) -> WordRef<'a>;
        pub fn make_word_list<'a>(word: WordRef<'a>, head: WordListRef<'a>) -> WordListRef<'a>;
    
        // dispose_cmd.h
        pub fn dispose_word(word: WordRef<'_>);
        pub fn dispose_words(list: WordListRef<'_>);
    
        // Current working directory
        pub fn get_working_directory(for_whom: *const c_char) -> *const c_char;
        pub fn set_working_directory(name: *const c_char);
        // evalstring.c
        pub fn parse_and_execute(source: *const c_char, from_file: *const c_char, flags: c_int) -> c_int;
        pub fn eval_string(eval: *const c_char, from_file: *const c_char, flags: c_int) -> c_int;
        pub fn parse_and_execute_cleanup(old_running_trap: c_int);
        pub fn parse_string(source: *const c_char, from_file: *const c_char, flags: c_int, cmdp: *const *const c_void, endp: *const *const c_char) -> c_int;
    
        // evalfile.c
        pub fn maybe_execute_file(filename: *const c_char, force_noninteractive: c_int) -> c_int;
        pub fn force_execute_file(filename: *const c_char, force_noninteractive: c_int) -> c_int;
        pub fn source_file(filename: *const c_char, sflags: c_int) -> c_int;

        // variables.h
        pub fn find_variable<'a>(name: *const c_char) -> ShellVar<'a>;
        pub fn find_variable_noref<'a>(name: *const c_char) -> ShellVar<'a>;
        
        pub fn find_global_variable<'a>(name: *const c_char) -> ShellVar<'a>;
        pub fn find_global_variable_noref<'a>(name: *const c_char) -> ShellVar<'a>;
        pub fn find_shell_variable<'a>(name: *const c_char) -> ShellVar<'a>;
        pub fn find_tempenv_variable<'a>(name: *const c_char) -> ShellVar<'a>;

        pub fn get_variable_value(var: ShellVar<'_>) -> *const c_char;
        pub fn get_string_value(name: *const c_char) -> *const c_char;

        pub fn copy_variable<'a, 'b>(var: ShellVar<'a>) -> ShellVar<'b>;

        pub fn make_local_variable<'a>(name: *const c_char, flags: c_int) -> ShellVar<'a>;

        pub fn bind_variable<'a>(name: *const c_char, value: *const c_char, flags: c_int) -> ShellVar<'a>;
        pub fn bind_global_variable<'a>(name: *const c_char, value: *const c_char, flags: c_int) -> ShellVar<'a>;
        pub fn bind_variable_value<'a>(var: ShellVar<'a>, value: *const c_char, flags: c_int) -> ShellVar<'a>;
        pub fn bind_int_value<'a>(var: ShellVar<'a>, value: *const c_char, flags: c_int) -> ShellVar<'a>;
        pub fn bind_var_to_int<'a>(var: *const c_char, value: c_long, flags: c_int) -> ShellVar<'a>;

        pub fn unbind_variable(name: *const c_char) -> c_int;
        pub fn check_unbind_variable(name: *const c_char) -> c_int;
        pub fn unbind_nameref(name: *const c_char) -> c_int;
        pub fn unbind_variable_noref(name: *const c_char) -> c_int;


        pub fn dispose_variable(var: ShellVar<'_>);

    }
}