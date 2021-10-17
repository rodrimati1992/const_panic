/// Constructs a `PanicVal` from a type that has a `to_panicval` method.
#[macro_export]
macro_rules! panicval {
    ($reff:expr) => {
        match &$reff {
            reff => $crate::__::PanicFmt::PROOF
                .infer(reff)
                .coerce(reff)
                .to_panicval(),
        }
    };
}

/// Calls the `to_panicvals` method on `$reff`,
/// which is expected to return a `[PanicVal<'_>; LEN]`.
#[macro_export]
macro_rules! to_panicvals {
    ($reff:expr) => {
        match &$reff {
            reff => $crate::__::PanicFmt::PROOF
                .infer(reff)
                .coerce(reff)
                .to_panicvals(),
        }
    };
}

#[macro_export]
macro_rules! concat_panic {
    ($($reff:expr),* $(,)*) => (
        // formatting it like this so that it prints `$crate::concat_panic(args)` by itself
        match &[
            $( Wrapper(&$crate::to_panicvals!($reff)).deref_panic_vals(), )*
        ] {
            args => {
                $crate::concat_panic(args)
            }
        }

    )
}
