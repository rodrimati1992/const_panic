macro_rules! overf_fmt {
    ($len:expr; $($args:tt)*) => ( const_panic::concat_fmt!($len, $len + 1; $($args)*) )
}
macro_rules! trunc_fmt {
    ($len:expr; $($args:tt)*) => ( const_panic::concat_fmt!($len, $len; $($args)*).unwrap() )
}

mod main_tests {
    #[cfg(feature = "all_items")]
    mod array_tests;
    mod integer_tests;
}
