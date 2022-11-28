macro_rules! overf_fmt {
    ($len:expr; $($args:tt)*) => ( const_panic::concat_fmt!($len, $len + 1; $($args)*) )
}
macro_rules! trunc_fmt {
    ($len:expr; $($args:tt)*) => ( const_panic::concat_fmt!($len, $len; $($args)*).unwrap() )
}

mod main_tests {
    #[cfg(feature = "non_basic")]
    mod array_tests;

    #[cfg(feature = "non_basic")]
    mod arraystring_tests;

    mod assert_tests;

    mod char_tests;

    mod concat_macro_tests;

    #[cfg(feature = "rust_1_64")]
    mod rust_1_64_types_tests;

    #[cfg(feature = "non_basic")]
    mod impl_panicfmt_tests;

    #[cfg(feature = "non_basic")]
    mod option_fmt_tests;

    #[cfg(feature = "non_basic")]
    mod other_fmt_tests;

    #[cfg(feature = "derive")]
    mod derive_tests;

    mod integer_tests;

    mod misc_macros_tests;

    mod panicval_macros_tests;

    #[cfg(feature = "non_basic")]
    mod pvcount_tests;

    mod string_tests;
}
