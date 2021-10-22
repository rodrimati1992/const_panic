/// Implements the [`PanicFmt`](crate::PanicFmt)
/// trait and the `to_panicvals` method it requires.
///
/// This macro only accepts concrete types, and types with `'_` lifetime arguments,
/// because it uses them in places where generic types don't yet work.
///
/// # Examples
///
/// ### Struct formatting
///
/// ```rust
/// use const_panic::{ArrayString, FmtArg, impl_panicfmt};
///
/// fn main(){
///     let foo = Foo {
///         x: &[3, 5, 8, 13],
///         y: 21,
///         z: Bar(false, true),
///     };
///     
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&foo.to_panicvals(FmtArg::DEBUG)).unwrap(),
///         "Foo { x: [3, 5, 8, 13], y: 21, z: Bar(false, true) }",
///     );
///     assert_eq!(
///         ArrayString::<200>::from_panicvals(&foo.to_panicvals(FmtArg::ALT_DEBUG)).unwrap(),
///         concat!(
///             "Foo {\n",
///             "    x: [\n",
///             "        3,\n",
///             "        5,\n",
///             "        8,\n",
///             "        13,\n",
///             "    ],\n",
///             "    y: 21,\n",
///             "    z: Bar(\n",
///             "        false,\n",
///             "        true,\n",
///             "    ),\n",
///             "}",
///         ),
///     );
/// }
///
///
/// struct Foo<'a> {
///     x: &'a [u8],
///     y: u8,
///     z: Bar,
/// }
///
/// // Implementing `PanicFmt` and the `to_panicvals` method for
/// // `Foo<'a>` (with any lifetime).
/// //
/// // Only `Foo<'_>` or `Foo<'static>` can work here, due to what the macro expands into.
/// impl_panicfmt!{
///     impl Foo<'_>;
///     
///     struct Foo {
///         // removing all lifetimes in fields (or replacing them with `'_`) is required
///         x: &[u8],
///         y: u8,
///         z: Bar,
///     }
/// }
///
///
/// struct Bar(bool, bool);
///
/// impl_panicfmt!{
///     impl Bar;
///     
///     struct Bar(bool, bool)
/// }
///
/// ```
///
/// ### Enum Formatting
///
/// ```rust
/// use const_panic::{ArrayString, FmtArg, impl_panicfmt};
///
/// fn main() {
///     let up: Qux<u8> = Qux::Up;
///     // Debug formatting the Up variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&up.to_panicvals(FmtArg::DEBUG)).unwrap(),
///         "Up",
///     );
///
///
///     let down: Qux<u16> = Qux::Down { x: 21, y: 34, z: 55 };
///     // Debug formatting the Down variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&down.to_panicvals(FmtArg::DEBUG)).unwrap(),
///         "Down { x: 21, y: 34, z: 55 }",
///     );
///     // Alternate-Debug formatting the Down variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&down.to_panicvals(FmtArg::ALT_DEBUG)).unwrap(),
///         concat!(
///             "Down {\n",
///             "    x: 21,\n",
///             "    y: 34,\n",
///             "    z: 55,\n",
///             "}",
///         )
///     );
///
///
///     let left: Qux<u32> = Qux::Left(89);
///     // Debug formatting the Left variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&left.to_panicvals(FmtArg::DEBUG)).unwrap(),
///         "Left(89)",
///     );
///     // Alternate-Debug formatting the Left variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&left.to_panicvals(FmtArg::ALT_DEBUG)).unwrap(),
///         concat!(
///             "Left(\n",
///             "    89,\n",
///             ")",
///         )
///     );
/// }
///
/// enum Qux<T> {
///     Up,
///     Down { x: T, y: T, z: T },
///     Left(u64),
/// }
///
///
/// // Because of limitations of stable const evaluation,
/// // you have to use macros to invoke the `impl_panicfmt` macro
/// // for more than one concrete type.
/// //
/// // This macro invocation implements panic formatting for
/// // - `Qux<u8>`
/// // - `Qux<u16>`
/// // - `Qux<u32>`
/// const_panic::inline_macro! {
///     (u8),
///     (u16),
///     (u32);
///
///     ($T:ty) =>
///
///     impl_panicfmt!{
///         impl Qux<$T>;
///         
///         // causes the returned `PanicVal`s from `Qux::to_panicvals`
///         // to be `PanicVal<'static>` instead of `PanicVal<'_>`
///         // (the default is that it borrows from the `self` argument)
///         lifetime = 'static;
///         
///         enum Qux {
///             Up,
///             Down { x: $T, y: $T, z: $T },
///             Left(u64),
///         }
///     }
/// }
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
#[macro_export]
macro_rules! impl_panicfmt {
    (
        impl $type:path;

        $(lifetime = $lt:lifetime;)?

        struct $typename:ident $({ $($braced:tt)* })? $(( $($tupled:tt)* ))? $(;)?
    ) => (
        $crate::__impl_panicfmt_step_ccc!{
            (
                impl $type;
                struct
                lifetime($($lt,)? '_,)
            )
            []
            [
                $typename $({ $($braced)* })? $(( $($tupled)* ))?
            ]
        }
    );
    (
        impl $type:path;

        $(lifetime = $lt:lifetime;)?

        enum $typename:ident{
            $(
                $variant:ident $({ $($braced:tt)* })? $(( $($tupled:tt)* ))?
            ),*
            $(,)?
        }
    ) => (
        $crate::__impl_panicfmt_step_ccc!{
            (
                impl $type;
                enum
                lifetime   ($($lt,)? '_,)
            )
            []
            [
                $(
                    $variant $({ $($braced)* })? $(( $($tupled)* ))?,
                )*
            ]
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules!  __impl_panicfmt_step_ccc {
    (
        $kept:tt
        $prev_variants:tt
        [
            $variant:ident
            $($(@$is_brace:tt@)? {$($br_field:ident: $br_ty:ty),* $(,)*})?
            $($(@$is_tuple:tt@)? ( $($tup_ty:ty),* $(,)* ))?
            $(,$($rem_variants:tt)*)?
        ]
    ) => {
        $crate::zip_counter_and_last!{
            $crate::__impl_panicfmt_step_ccc_inner!{
                $kept
                $prev_variants
                $variant
                (
                    $($($is_brace)? Braced)?
                    $($($is_tuple)? Tupled)?
                    Braced
                )
                [$($($rem_variants)*)?]
            }
            ($($(($br_field, $br_ty))*)? $($((, $tup_ty))*)?)
            (
                (0 fi0) (1 fi1) (2 fi2) (3 fi3) (4 fi4) (5 fi5) (6 fi6) (7 fi7)
                (8 fi8) (9 fi9) (10 fi10) (11 fi11) (12 fi12) (13 fi13) (14 fi14) (15 fi15)
                (16 fi16) (17 fi17) (18 fi18) (19 fi19) (20 fi20) (21 fi21) (22 fi22) (23 fi23)
                (24 fi24) (25 fi25) (26 fi26) (27 fi27) (28 fi28) (29 fi29) (30 fi30) (31 fi31)
                (32 fi32) (33 fi33) (34 fi34) (35 fi35) (36 fi36) (37 fi37) (38 fi38) (39 fi39)
                (40 fi40) (41 fi41) (42 fi42) (43 fi43) (44 fi44) (45 fi45) (46 fi46) (47 fi47)
                (48 fi48) (49 fi49) (50 fi50) (51 fi51) (52 fi52) (53 fi53) (54 fi54) (55 fi55)
                (56 fi56) (57 fi57) (58 fi58) (59 fi59) (60 fi60) (61 fi61) (62 fi62) (63 fi63)
            )
        }
    };
    // Parsing unit variants / structs
    (
        $kept:tt
        [$($prev_variants:tt)*]
        [
            $variant:ident
            $(, $($rem_variants:tt)*)?
        ]
    ) => {
        $crate::__impl_panicfmt_step_ccc!{
            $kept
            [$($prev_variants)* ($variant Braced) ]
            [$($($rem_variants)*)?]
        }
    };
    // Finished parsing variants/structs,
    //
    (
        ($($kept:tt)*)
        [$($variants:tt)*]
        []
    ) => {
        $crate::__impl_panicfmt_step_finished!{
            $($kept)*
            variants( $($variants)* )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules!  __impl_panicfmt_step_ccc_inner {
    (
        $kept:tt
        [$($prev_variants:tt)*]
        $variant:ident
        ($delim:ident $($ignored0:tt)*)
        [$($rem_variants:tt)*]

        $(prefix (($($p_fname:ident)?, $p_ty:ty) ($p_index:tt $p_fi_index:tt)))*
        $(last (($($l_fname:ident)?, $l_ty:ty) ($l_index:tt $l_fi_index:tt)) )?
    ) => {
        $crate::__impl_panicfmt_step_ccc!{
            $kept
            [
                $($prev_variants)*
                (
                    $variant
                    $delim
                    ($($l_index + 1,)? 0,)
                    =>
                    $(prefix (($($p_fname)? $p_index), ($($p_fname)? $p_fi_index), $p_ty))*
                    $(last (($($l_fname)? $l_index), ($($l_fname)? $l_fi_index), $l_ty))?
                )
            ]
            [$($rem_variants)*]
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules!  __impl_panicfmt_step_finished {
    (
        impl $type:path;

        $type_kind:ident

        lifetime($lt:lifetime , $($ignored0:tt)*)

        variants($(
            (
                $variant:ident
                $delimiter:ident
                ($field_amount:expr, $($ignored2:tt)*)
                =>
                $(
                    $is_last_field:ident
                    (
                        ($fpati:tt $($ignored3:tt)?),
                        ($fname:tt $($ignored4:tt)?),
                        $ty:ty
                    )
                )*
            )
        )*)
    ) => (
        impl $crate::PanicFmt for $type {
            type This = Self;
            type Kind = $crate::fmt::IsCustomType;
            const PV_COUNT: $crate::__::usize = $crate::utils::slice_max_usize(&[
                $(
                    $crate::fmt::ComputePvCount{
                        field_amount: $field_amount,
                        summed_pv_count: 0 $( + <$ty as $crate::PanicFmt>::PV_COUNT )*,
                        delimiter: $crate::fmt::TypeDelim::$delimiter
                    }.call()
                ),*
            ]);
        }

        impl $type {
            pub const fn to_panicvals(
                &self,
                mut fmt: $crate::FmtArg,
            ) -> [$crate::PanicVal<$lt>; <$type as $crate::PanicFmt>::PV_COUNT] {
                match self {
                    $(
                        $crate::__ipm_pattern!($type_kind $variant{$($fpati: $fname,)* ..}) =>
                            $crate::__ipm_fmt!{
                                (<$type as $crate::PanicFmt>::PV_COUNT)
                                $delimiter
                                $variant
                                fmt
                                ( $($is_last_field ($fname, $ty))* )
                            },
                    )*
                }

            }
        }
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules!  __ipm_pattern {
    (struct $name:ident {$($patterns:tt)*}) => {
        $name {$($patterns)*}
    };
    (enum $name:ident {$($patterns:tt)*}) => {
        Self::$name {$($patterns)*}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules!  __ipm_fmt {
    (
        ($count:expr) $delimiter:ident $typename:ident $fmt:ident
        ( $($is_last_field:ident ($fname:ident, $ty:ty))+ )
    ) => ({
        let (open, close) = $crate::fmt::TypeDelim::$delimiter.get_open_and_close();

        $crate::__::flatten_panicvals::<{$count}>(&[
            &[
                $crate::PanicVal::write_str($crate::__::stringify!($typename)),
                {
                    $fmt = $fmt.indent();
                    open.to_panicval($fmt)
                }
            ],
            $(
                $crate::__ipm_pv_fmt_field_name!($delimiter $fname),
                &<$ty as $crate::PanicFmt>::PROOF
                    .coerce($fname)
                    .to_panicvals($fmt),
                &$crate::__ipm_pv_comma!($is_last_field)
                    .to_panicvals($fmt),
            )*
            &[
                {
                    $fmt = $fmt.unindent();
                    close.to_panicval($fmt)
                }
            ],
        ])
    });
    (
        ($count:expr) $delimiter:ident $typename:ident $fmt:ident
        ()
    ) => {
        $crate::__::flatten_panicvals::<{$count}>(&[
            &[$crate::PanicVal::write_str($crate::__::stringify!($typename))]
        ])
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_pv_fmt_field_name {
    (Tupled $field_name:ident) => {
        &[]
    };
    (Braced $field_name:ident) => {
        &[$crate::PanicVal::write_str($crate::__::concat!(
            $crate::__::stringify!($field_name),
            ": "
        ))]
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_pv_comma {
    (prefix) => {
        $crate::fmt::COMMA_SEP
    };
    (last) => {
        $crate::fmt::COMMA_TERM
    };
}
