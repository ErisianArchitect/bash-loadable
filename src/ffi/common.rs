
pub mod ffi {
    use core::ffi::{
        c_void,
        c_char,
        c_int,
    };
    unsafe extern "C" {
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
    }
}
