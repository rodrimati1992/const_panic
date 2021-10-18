#[cfg(feature = "all_items")]
#[macro_use]
mod non_basic_macros;

#[doc(hidden)]
#[macro_export]
macro_rules! __write_array {
    ($array:expr, $len:expr, $value:expr) => {
        $array[$len] = $value;
        $len += 1;
    };
}

/// Constructs a `PanicVal` from a type that has a `to_panicval` method.
#[macro_export]
macro_rules! panicval {
    ($fmtargs:expr, $reff:expr) => {
        match &$reff {
            reff => $crate::__::PanicFmt::PROOF
                .infer(reff)
                .coerce(reff)
                .to_panicval($fmtargs),
        }
    };
}

/// Calls the `to_panicvals` method on `$reff`,
/// which is expected to return a `[PanicVal<'_>; LEN]`.
#[macro_export]
macro_rules! to_panicvals {
    ($fmtargs:expr, $reff:expr) => {
        match &$reff {
            reff => $crate::__::PanicFmt::PROOF
                .infer(reff)
                .coerce(reff)
                .to_panicvals($fmtargs),
        }
    };
}

#[macro_export]
macro_rules! concat_panic {
    ($($args:tt)*) => (
        $crate::__concat_func!{
            (|args| $crate::concat_panic(args))
            []
            [$($args)*,]
        }
    )
}

#[macro_export]
macro_rules! __concat_func{
    ($args:tt [$($prev:tt)*] [$keyword:ident: $expr:expr, $($rem:tt)* ]) => {
        $crate::__concat_func!{
            $args
            [$($prev)* ($crate::__fmtarg_from_kw!($keyword), $expr)]
            [$($rem)*]
        }
    };
    ($args:tt [$($prev:tt)*] [$expr:literal, $($rem:tt)* ]) => {
        $crate::__concat_func!{
            $args
            [$($prev)* ($crate::FmtArg::DISPLAY, $expr)]
            [$($rem)*]
        }
    };
    ($args:tt [$($prev:tt)*] [$expr:expr, $($rem:tt)* ]) => {
        $crate::__concat_func!{
            $args
            [$($prev)* ($crate::FmtArg::DEBUG, $expr)]
            [$($rem)*]
        }
    };
    ((|$args:ident| $function_call:expr) [$(($fmt_arg:expr, $reff:expr))*] [$(,)*]) => {
        match &[
            $( $crate::Wrapper(&$crate::to_panicvals!($fmt_arg, $reff)).deref_panic_vals(), )*
        ] {
            $args => $function_call,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fmtarg_from_kw {
    (display) => {
        $crate::FmtArg::DISPLAY
    };
    (debug) => {
        $crate::FmtArg::DEBUG
    };
    ($kw:ident) => {
        compile_error!(concat!(
            "unrecognized keyword: ",
            stringify!($kw),
            "\n",
            "expected one of:\n",
            "- display\n",
            "- debug\n",
        ))
    };
}
