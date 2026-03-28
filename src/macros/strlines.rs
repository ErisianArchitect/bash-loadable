

#[macro_export]
macro_rules! strlines {
    (@append: $line:expr) => {
        $line
    };
    (@append: $line:expr, $($rest:expr),+$(,)?) => {
        concat!($line, "\n", $crate::strlines!(@append: $($rest),*))
    };
    ($($line:expr),+$(,)?) => {
        $crate::strlines!(@append: $($line),*)
    };
}