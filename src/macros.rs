#[cfg(feature = "non_basic")]
#[macro_use]
mod non_basic_macros;

#[cfg(feature = "non_basic")]
#[macro_use]
mod macro_utils;

#[cfg(feature = "non_basic")]
#[macro_use]
mod impl_panicfmt;

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

/// Coerces `$reff` to a type that has a `to_panicvals` method,
/// which is expected to return a `[PanicVal<'_>; LEN]`.
///
/// # Limitations
///
#[doc = crate::doc_macros::limitation_docs!()]
///
/// # Example
///
/// This example uses [`const_panic::ArrayString`](crate::ArrayString)
/// to show what the values format into,
/// which requires the `"non_basic"` crate feature (enabled by default).
///
#[cfg_attr(feature = "non_basic", doc = "```rust")]
#[cfg_attr(not(feature = "non_basic"), doc = "```ignore")]
/// use const_panic::{ArrayString, FmtArg, IsCustomType, PanicFmt, PanicVal, coerce_fmt};
///
/// type AS = ArrayString<100>;
///
/// assert_eq!(
///     AS::from_panicvals(&coerce_fmt!(100u8).to_panicvals(FmtArg::DEBUG)).unwrap(),
///     "100",
/// );
///
/// assert_eq!(
///     AS::from_panicvals(&coerce_fmt!("hello\n").to_panicvals(FmtArg::DEBUG)).unwrap(),
///     r#""hello\n""#,
/// );
///
/// assert_eq!(
///     AS::from_panicvals(&coerce_fmt!(IsReal::No).to_panicvals(FmtArg::DEBUG)).unwrap(),
///     "No",
/// );
///
/// assert_eq!(
///     AS::from_panicvals(&coerce_fmt!(IsReal::Yes).to_panicvals(FmtArg::DEBUG)).unwrap(),
///     "Yes",
/// );
///
///
///
/// enum IsReal{Yes, No}
///
/// // Manually implementing panic formatting for a field-less enum
/// //
/// // All the code below is equivalent to this `impl_panicfmt` invocation:
/// // ```
/// // impl_panicfmt!{
/// //      impl IsReal;
/// //      enum IsReal { Yes, No }
/// // }
/// // ```
/// impl PanicFmt for IsReal {
///     type This = Self;
///     type Kind = IsCustomType;
///     const PV_COUNT: usize = 1;
/// }
///
/// impl IsReal {
///     pub const fn to_panicvals(&self, _f: FmtArg) -> [PanicVal<'_>; IsReal::PV_COUNT] {
///         let x = match self {
///             Self::Yes => "Yes",
///             Self::No => "No",
///         };
///         [PanicVal::write_str(x)]
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! coerce_fmt {
    ($reff:expr) => {
        match &$reff {
            reff => $crate::__::PanicFmt::PROOF.infer(reff).coerce(reff),
        }
    };
}

/// Panics with the concanenation of the arguments.
///
/// # Syntax
///
/// This macro uses this syntax:
/// ```text
/// concat_panic!(
///     $($fmtarg:expr;)?
///     $(
///         $( $format_override:tt: )? $arg_to_fmt:expr
///     ),*
///     $(,)?
/// )
/// ```
///
/// `$fmtarg` is an optional [`FmtArg`](crate::FmtArg) argument
/// which defaults to `FmtArg::DEBUG`,
/// determining how non-literal `$arg_to_fmt` arguments are formatted.
///
/// [`$format_override`](#formatting-overrides) overrides the `$fmtarg` argument,
/// changing how that `$arg_to_fmt` argument is formatted.
///
#[doc = formatting_docs!()]
///
/// # Limitations
///
#[doc = crate::doc_macros::limitation_docs!()]
///
/// # Examples
///
/// For basic examples of this macro,
/// you can look [at the root module](./index.html#examples)
///
/// ### All the syntax
///
/// This example demonstrates using all of the syntax of this macro.
///
/// ```compile_fail
/// use const_panic::{FmtArg, concat_panic, fmt};
///
/// const _: () = concat_panic!{
///     // the optional `$fmtarg` parameter.
///     // If this argument wasn't passed, it'd be `FmtArg::DEBUG`
///     FmtArg::ALT_DEBUG;
///
///     "\n\nshowing off literals:\n",
///     100u8,
///     "hello",
///
///     "\n\nnon-literals with formatting determined by the $fmtarg parameter:\n",
///     // this is considered a non-literal, because it's inside other tokens.
///     ("a non-literal"),
///     [100u8, 200],
///
///     "\n\nexplicitly debug formatted:\n",
///     debug: "foo",
///     // `{?}:` is The same as `debug:`
///     {?}: "bar",
///
///     "\n\nalternate debug formatted:\n",
///     alt_debug: ["foo"],
///     // `{#?}:` is The same as `alt_debug:`
///     {#?}: "bar",
///
///     "\n\ndisplay formatted:\n",
///     display: "baz",
///     // `{}:` is The same as `display:`
///     {}: ["qux", "aaa"],
///
///     "\n\nalternate display formatted:",
///     alt_display: ["bbb", "ccc"],
///     // `{#}:` is The same as `alt_display:`
///     {#}: ["bbb", "ccc"],
///
///     "\n\n",
/// };
///
/// ```
/// The above code produces this compile-time error:
/// ```text
/// error[E0080]: evaluation of constant value failed
///   --> src/macros.rs:94:15
///    |
/// 6  |   const _: () = concat_panic!{
///    |  _______________^
/// 7  | |     // the optional `$fmtarg` parameter.
/// 8  | |     // If this argument wasn't passed, it'd be `FmtArg::DEBUG`
/// 9  | |     FmtArg::ALT_DEBUG;
/// ...  |
/// 40 | |     "\n\n",
/// 41 | | };
///    | |_^ the evaluated program panicked at '
///
/// showing off literals:
/// 100hello
///
/// non-literals with formatting determined by the $fmtarg parameter:
/// "a non-literal"[
///     100,
///     200,
/// ]
///
/// explicitly debug formatted:
/// "foo""bar"
///
/// alternate debug formatted:
/// [
///     "foo",
/// ]"bar"
///
/// display formatted:
/// baz[qux, aaa]
///
/// alternate display formatted:[
///     bbb,
///     ccc,
/// ][
///     bbb,
///     ccc,
/// ]
///
/// ', src/macros.rs:6:15
///
/// ```
///
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
            $(
                $crate::Wrapper(
                    &$crate::coerce_fmt!($reff)
                    .to_panicvals($fmt_arg)
                ).deref_panic_vals(),
            )*
        ] {
            $args => $function_call,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __set_fmt_from_kw {
    (open, $fmtarg:ident) => {{
        $fmtarg = $fmtarg.indent();
        $fmtarg.set_display()
    }};
    (close, $fmtarg:ident) => {{
        $fmtarg = $fmtarg.unindent();
        $fmtarg.set_display()
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
