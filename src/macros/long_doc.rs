/// ```rust, ignore
/// longdoc![
///     "First Paragraph",
///     "Second Paragraph.",
/// ]
/// ```
#[macro_export]
macro_rules! longdoc {
    [$(
        $paragraph:expr
    ),*$(,)?] => {
        $crate::util::docs::LongDoc::new([
            $(
                $crate::str_to_cstr!($paragraph),
            )*
        ]).as_ptr()
    };
}