#[macro_export]
macro_rules! cstr {
    ($string:expr) => {
        ($string).as_ptr()
    };
}

#[macro_export]
macro_rules! str_to_cstr {
    ($string:expr) => {
        concat!($string, "\0").as_ptr().cast::<core::ffi::c_char>()
    };
}