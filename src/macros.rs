
#[macro_export]
macro_rules! cstr {
    ($string:literal) => {
        $string.as_ptr()
    };
}