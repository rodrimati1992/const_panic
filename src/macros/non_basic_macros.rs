/// Calls the `to_panicvals` method on many `$reff`s
/// then flattens them into a single array.
///
/// # Kinds of types
///
/// This takes two different kinds of types:
/// - [primitive](#primitive): which are represented using a single [`PanicVal`],
/// - [composite](#composite): which are represented using an array of `PanicVal`s.
///
/// ### Primitive
///
/// These have a `to_panicval` method which returns a single [`PanicVal`],
/// and don't require their type to be passed with the `Type => value` syntax
///
/// These are some primitive types:
/// - integers
/// - `bool`
/// - `&str`
/// - arrays
/// - slices
///
/// ### Composite
///
/// These have a `to_panicvals` method which returns an array of [`PanicVal`]s,
/// and require their type to be passed with the `Type => value` syntax
///
#[doc = formatting_docs!()]
///
///
/// # Examples
///
///
///
///
///
/// [`PanicVal`]: crate::PanicVal
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
#[macro_export]
macro_rules! flatten_panicvals {
    ($fmtargs:expr $(, $length:expr)?; $($args:tt)* ) => {{
        let mut fmtargs: $crate::FmtArg = $fmtargs;
        $crate::__to_pvf_inner!(fmtargs [$(length($length))?][$($args)* ,])
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_inner {
    (
        $fmtargs:ident
        [
            $(length($expected_length:expr))?
            $((($len:expr, $kind:ident $args:tt), $fmt_override:tt, $reff:expr))*
        ]
        [$(,)*]
    ) => ({
        const __ADDED_UP_LEN_SDOFKE09F__: $crate::__::usize = 0 $( + $len )*;
        const __LEN_SDOFKE09F__: $crate::__::usize =
            $crate::__to_pvf_used_length!(__ADDED_UP_LEN_SDOFKE09F__, $($expected_length)?);

        $(
            const _: () =
                $crate::__::assert_flatten_panicvals_length(
                    $expected_length,
                    __ADDED_UP_LEN_SDOFKE09F__,
                );
        )?
        $crate::__::flatten_panicvals::<__LEN_SDOFKE09F__>(&[
            $(
                $crate::__to_pvf_kind!($fmtargs $kind $args, $fmt_override, $reff)
            ),*
        ])
    });
    ($fmtargs:ident [$($prev:tt)*] [_, $($rem:tt)*]) => {
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* ((1, single()), _, $crate::PanicVal::EMPTY)]

            [$($rem)*]
        }
    };
    ($fmtargs:ident $prev:tt [$ty:ty => $($rem:tt)*]) => {
        $crate::__to_pvf_expr!{
            $fmtargs
            $prev
            (<$ty as $crate::__::PanicFmt>::PV_COUNT, many($ty))
            [$($rem)*]
        }
    };
    ($fmtargs:ident $prev:tt [$($rem:tt)*]) => {
        $crate::__to_pvf_expr!{
            $fmtargs
            $prev
            (1, single())
            [$($rem)*]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_expr {
    ($fmtargs:ident [$($prev:tt)*] $other:tt [$kw:tt: $reff:expr, $($rem:tt)*]) => {
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* ($other, $kw, $reff)]

            [$($rem)*]
        }
    };
    ($fmtargs:ident [$($prev:tt)*] $other:tt [$reff:literal, $($rem:tt)*])=>{
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* ($other, display, $reff)]

            [$($rem)*]
        }
    };
    ($fmtargs:ident [$($prev:tt)*] $other:tt [$reff:expr, $($rem:tt)*]) => {
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* ($other, _, $reff)]

            [$($rem)*]
        }
    };
    ($fmtargs:ident [$($prev:tt)*] $other:tt [$($rem:tt)*]) => {
        $crate::__::compile_error!(concat!(
            "expected expression, found:",
            stringify!($($rem)*)
        ))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_kind {
    ($fmtargs:ident single (), $fmt_override:tt, $reff:tt) => {
        &match &$reff {
            reff => [$crate::__::PanicFmt::PROOF
                .infer(reff)
                .coerce(reff)
                .to_panicval($crate::__set_fmt_from_kw!($fmt_override, $fmtargs))],
        }
    };
    ($fmtargs:ident many ($ty:ty), $fmt_override:tt, $reff:tt) => {
        $crate::__::panicvals_id::<{ <$ty as $crate::__::PanicFmt>::PV_COUNT }>(&match &$reff {
            reff => <$ty as $crate::__::PanicFmt>::PROOF
                .coerce(reff)
                .to_panicvals($crate::__set_fmt_from_kw!($fmt_override, $fmtargs)),
        })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_used_length {
    ( $added_up:expr, $expected_length:expr ) => {
        $crate::utils::max_usize($added_up, $expected_length)
    };
    ( $added_up:expr, ) => {
        $added_up
    };
}

/// Helper macro for defining and using a `macro_rules!` macro inline.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
#[macro_export]
macro_rules! inline_macro{
    (
        $(($($args:tt)*)),* $(,)?;
        ($($params:tt)*)
        =>
        $($code:tt)*
    ) => {
        macro_rules! __dummy__ {
            ($($params)*) => {$($code)*}
        }
        $(
            __dummy__!{ $($args)* }
        )*
    }
}
