#[cfg(feature = "non_basic")]
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

#[doc(hidden)]
#[macro_export]
macro_rules! __write_array_checked {
    ($array:expr, $len:expr, $value:expr) => {
        if $array.len() > $len {
            $array[$len] = $value;
            $len += 1;
        }
    };
}

/// Calls the `to_panicvals` method on `$reff`,
/// which is expected to return a `[PanicVal<'_>; LEN]`.
#[macro_export]
macro_rules! to_panicvals {
    ($fmtargs:expr; $reff:expr) => {
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
        $crate::__concat_func_setup!{
            (|args| $crate::concat_panic(args))
            []
            [$($args)*,]
        }
    )
}

// This macro takes the optional `$fmt:expr;` argument before everything else.
// But I had to parse the argument manually,
// because `$fmt:expr;` fails compilation instead of trying the following branches
// when the argument isn't valid expression syntax.
#[doc(hidden)]
#[macro_export]
macro_rules! __concat_func_setup {
    ($args:tt $prev:tt [$($fmt:tt).*; $($rem:tt)* ]) => ({
        let mut fmt: $crate::FmtArg = $($fmt).*;
        $crate::__concat_func!{fmt $args $prev [$($rem)*]}
    });
    ($args:tt $prev:tt [$(:: $(@$_dummy:tt@)?)? $($fmt:ident)::* ; $($rem:tt)* ]) => ({
        let mut fmt: $crate::FmtArg = $(:: $($_dummy)?)? $($fmt)::*;
        $crate::__concat_func!{fmt $args $prev [$($rem)*]}
    });
    ($args:tt $prev:tt $rem:tt) => ({
        let mut fmt: $crate::FmtArg = $crate::FmtArg::DEBUG;
        $crate::__concat_func!{fmt $args $prev $rem}
    });
}
#[doc(hidden)]
#[macro_export]
macro_rules! __concat_func {
    ($fmt:ident $args:tt [$($prev:tt)*] [$keyword:tt: $expr:expr, $($rem:tt)* ]) => {
        $crate::__concat_func!{
            $fmt
            $args
            [$($prev)* ($crate::__set_fmt_from_kw!($keyword, $fmt), $expr)]
            [$($rem)*]
        }
    };
    ($fmt:ident $args:tt [$($prev:tt)*] [$expr:literal, $($rem:tt)* ]) => {
        $crate::__concat_func!{
            $fmt
            $args
            [$($prev)* ($crate::__set_fmt_from_kw!(display, $fmt), $expr)]
            [$($rem)*]
        }
    };
    ($fmt:ident $args:tt [$($prev:tt)*] [$expr:expr, $($rem:tt)* ]) => {
        $crate::__concat_func!{
            $fmt
            $args
            [$($prev)* ($fmt, $expr)]
            [$($rem)*]
        }
    };
    ($fmt:ident (|$args:ident| $function_call:expr) [$(($fmt_arg:expr, $reff:expr))*] [$(,)*]) => {
        match &[
            $( $crate::Wrapper(&$crate::to_panicvals!($fmt_arg; $reff)).deref_panic_vals(), )*
        ] {
            $args => $function_call,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fmtarg_from_kw {
    ($kw:tt) => {
        $crate::__set_fmt_from_kw!($kw, $crate::fmt::FmtArg::DISPLAY)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __set_fmt_from_kw {
    (open, $fmtarg:ident) => {{
        $fmtarg = $fmtarg.indent();
        $fmtarg
    }};
    (close, $fmtarg:ident) => {{
        $fmtarg = $fmtarg.unindent();
        $fmtarg
    }};
    (display, $fmtarg:ident) => {
        $fmtarg.set_display().set_alternate(false)
    };
    ({}, $fmtarg:ident) => {
        $fmtarg.set_display().set_alternate(false)
    };
    (alt_display, $fmtarg:ident) => {
        $fmtarg.set_display().set_alternate(true)
    };
    ({#}, $fmtarg:ident) => {
        $fmtarg.set_display().set_alternate(true)
    };
    (debug, $fmtarg:ident) => {
        $fmtarg.set_debug().set_alternate(false)
    };
    ({?}, $fmtarg:ident) => {
        $fmtarg.set_debug().set_alternate(false)
    };
    (alt_debug, $fmtarg:ident) => {
        $fmtarg.set_debug().set_alternate(true)
    };
    ({#?}, $fmtarg:ident) => {
        $fmtarg.set_debug().set_alternate(true)
    };
    (_, $fmtarg:ident) => {
        $fmtarg
    };
    ($kw:tt, $fmtarg:ident) => {
        compile_error!(concat!(
            "unrecognized formatting specifier: ",
            stringify!($kw),
            "\n",
            "expected one of:\n",
            "- display/{}\n",
            "- debug/{?}\n",
        ))
    };
}
