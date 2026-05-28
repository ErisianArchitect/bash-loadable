pub mod alias;
pub mod argv;
pub mod array;
pub mod bash_owned;
pub mod bash_str;
pub mod builtin;
pub mod command;
pub mod eval;
pub mod external;
pub mod fn_ptr;
pub mod hash_table;
pub mod pattern;
pub mod strvec;
pub mod var;
pub mod word;

use core::ffi::{
    c_int,
};