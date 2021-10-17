/// Calls the `to_panicvals` method on many `$reff`s
/// then flattens them into a single array.
///
/// This requires passing the type of each element to get the length of the
/// returned array and add them.
#[macro_export]
macro_rules! to_panicvals_flatten {
    ($fmtargs:expr; $($args:tt)* ) => {
        $crate::__to_pvf_inner!($fmtargs [][$($args)* ,])
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_inner {
    (
        $fmtargs:tt
        [$(($len:expr, $kind:ident $args:tt, $reff:expr))*]
        [$(,)*]
    ) => {
        $crate::__::flatten_panicvals::<{ 0 $( + $len )* }>(&[
            $(
                $crate::__to_pvf_kind!($fmtargs $kind $args, $reff)
            ),*
        ])
    };
    ($fmtargs:tt [$($prev:tt)*] [$ty:ty => $reff:expr, $($rem:tt)*]) => {
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* (<$ty as $crate::__::PanicFmt>::PV_COUNT, many($ty), $reff)]

            [$($rem)*]
        }
    };
    ($fmtargs:tt [$($prev:tt)*] [$reff:expr, $($rem:tt)*]) => {
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* (1, single(), $reff)]

            [$($rem)*]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_kind {
    ($fmtargs:tt single (), $reff:tt) => {
        &match &$reff {
            reff => [$crate::__::PanicFmt::PROOF
                .infer(reff)
                .coerce(reff)
                .to_panicval($fmtargs)],
        }
    };
    ($fmtargs:tt many ($ty:ty), $reff:tt) => {
        $crate::__::panicvals_id::<{ <$ty as $crate::__::PanicFmt>::PV_COUNT }>(&match &$reff {
            reff => <$ty as $crate::__::PanicFmt>::PROOF
                .coerce(reff)
                .to_panicvals($fmtargs),
        })
    };
}
