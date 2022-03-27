#[macro_export]
macro_rules! format_tokenstream {
    ($($arg:tt)*) => {
        TokenStream::from_str(&format!($($arg)*)).unwrap()
    };
}
