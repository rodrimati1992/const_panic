macro_rules! overf_fmt {
    ($len:expr; $($args:tt)*) => ( const_panic::concat_fmt!($len, $len + 1; $($args)*) )
}
macro_rules! trunc_fmt {
    ($len:expr; $($args:tt)*) => ( const_panic::concat_fmt!($len, $len; $($args)*).unwrap() )
}

macro_rules! test_val {
    ($value:expr $(, no_alternate $(@$no_alternate:tt)?)?) => ({
        use const_panic::{StdWrapper, FmtArg};

        let val = $value;

        {
            let debug = format!("{:?}", val);
            assert_eq!(
                trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::DEBUG)),
                &*debug,
            );
        }

        $(#[cfg(any())] $($no_alternate)?)?
        {
            let alt_debug = format!("{:#?}", val);
            assert_eq!(
                trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::ALT_DEBUG)),
                &*alt_debug,
            );
        }

        {
            let display = format!("{}", val);
            assert_eq!(
                trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::DISPLAY)),
                &*display,
            );
        }

        $(#[cfg(any())] $($no_alternate)?)?
        {
            let alt_display = format!("{:#}", val);
            assert_eq!(
                trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::ALT_DISPLAY)),
                &*alt_display,
            );
        }
    })
}

mod main_tests {
    #[cfg(feature = "non_basic")]
    mod array_tests;

    #[cfg(feature = "non_basic")]
    mod arraystring_tests;

    mod assert_tests;

    mod char_tests;

    #[cfg(feature = "non_basic")]
    mod concat_macro_tests;

    #[cfg(feature = "rust_1_64")]
    mod rust_1_64_types_tests;

    #[cfg(feature = "rust_1_82")]
    mod rust_1_82_types_tests;

    #[cfg(feature = "rust_1_88")]
    mod rust_1_88_types_tests;

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

    mod utils_tests;
}
