#[doc(hidden)]
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
                    $crate::fmt::PvCountForStruct{
                        field_amount: $field_amount,
                        summed_pv_count: 0 $( + <$ty as $crate::PanicFmt>::PV_COUNT )*,
                        delimiter: $crate::fmt::StructDelim::$delimiter
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
        let (open, close) = $crate::fmt::StructDelim::$delimiter.get_open_and_close();

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
