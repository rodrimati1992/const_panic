use crate::{
    fmt::{FmtArg, FmtKind, PanicFmt},
    PanicVal, StdWrapper,
};

use core::num::{IntErrorKind, ParseIntError};

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_82")))]
impl PanicFmt for ParseIntError {
    type This = Self;
    type Kind = crate::fmt::IsStdType;
    const PV_COUNT: usize = 1;
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_82")))]
impl StdWrapper<&ParseIntError> {
    /// Formats a `ParseIntError` (supports both Debug and Display formatting).
    pub const fn to_panicvals(
        self,
        fmtarg: FmtArg,
    ) -> [PanicVal<'static>; ParseIntError::PV_COUNT] {
        [self.to_panicval(fmtarg)]
    }

    /// Formats a `ParseIntError` (supports both Debug and Display formatting).
    pub const fn to_panicval(self, fmtarg: FmtArg) -> PanicVal<'static> {
        macro_rules! debug_fmt {
            ($variant:ident) => {
                if fmtarg.is_alternate {
                    concat!("ParseIntError {\n    kind: ", stringify!($variant), ",\n}")
                } else {
                    concat!("ParseIntError { kind: ", stringify!($variant), " }")
                }
            };
        }

        let this = self.0;
        match fmtarg.fmt_kind {
            FmtKind::Display => PanicVal::write_str(match this.kind() {
                IntErrorKind::Empty => "cannot parse integer from empty string",
                IntErrorKind::InvalidDigit => "invalid digit found in string",
                IntErrorKind::PosOverflow => "number too large to fit in target type",
                IntErrorKind::NegOverflow => "number too small to fit in target type",
                IntErrorKind::Zero => "number would be zero for non-zero type",
                _ => "<ParseIntError>",
            }),
            FmtKind::Debug => PanicVal::write_str(match this.kind() {
                IntErrorKind::Empty => debug_fmt!(Empty),
                IntErrorKind::InvalidDigit => debug_fmt!(InvalidDigit),
                IntErrorKind::PosOverflow => debug_fmt!(PosOverflow),
                IntErrorKind::NegOverflow => debug_fmt!(NegOverflow),
                IntErrorKind::Zero => debug_fmt!(Zero),
                _ => "<IntErrorKind>",
            }),
        }
    }
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_82")))]
impl PanicFmt for IntErrorKind {
    type This = Self;
    type Kind = crate::fmt::IsStdType;
    const PV_COUNT: usize = 1;
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_82")))]
impl StdWrapper<&IntErrorKind> {
    /// Formats a `IntErrorKind` (supports only Debug formatting).
    pub const fn to_panicvals(self, fmtarg: FmtArg) -> [PanicVal<'static>; IntErrorKind::PV_COUNT] {
        [self.to_panicval(fmtarg)]
    }

    /// Formats a `IntErrorKind` (supports only Debug formatting).
    pub const fn to_panicval(self, _: FmtArg) -> PanicVal<'static> {
        PanicVal::write_str(match *self.0 {
            IntErrorKind::Empty => "Empty",
            IntErrorKind::InvalidDigit => "InvalidDigit",
            IntErrorKind::PosOverflow => "PosOverflow",
            IntErrorKind::NegOverflow => "NegOverflow",
            IntErrorKind::Zero => "Zero",
            _ => "<IntErrorKind>",
        })
    }
}

#[cfg(feature = "non_basic")]
const _: () = {
    use crate::fmt::{self as cfmt, ComputePvCount};

    use core::ffi::c_str::{FromBytesUntilNulError, FromBytesWithNulError};

    #[cfg_attr(
        feature = "docsrs",
        doc(cfg(all(feature = "non_basic", feature = "rust_1_82")))
    )]
    impl PanicFmt for FromBytesWithNulError {
        type This = Self;
        type Kind = crate::fmt::IsStdType;
        const PV_COUNT: usize = ComputePvCount {
            field_amount: 1,
            summed_pv_count: <usize>::PV_COUNT,
            delimiter: cfmt::TypeDelim::Braced,
        }
        .call();
    }

    #[cfg_attr(
        feature = "docsrs",
        doc(cfg(all(feature = "non_basic", feature = "rust_1_82")))
    )]
    impl StdWrapper<&FromBytesWithNulError> {
        /// Formats a `FromBytesWithNulError`.
        pub const fn to_panicvals(
            self,
            fmtarg: FmtArg,
        ) -> [PanicVal<'static>; FromBytesWithNulError::PV_COUNT] {
            match (fmtarg.fmt_kind, *self.0) {
                (FmtKind::Display, FromBytesWithNulError::InteriorNul { position }) => {
                    crate::utils::flatten_panicvals(&[&[
                        PanicVal::write_str(
                            "data provided contains an interior nul byte at byte pos ",
                        ),
                        PanicVal::from_usize(position, fmtarg),
                    ]])
                }
                (FmtKind::Display, FromBytesWithNulError::NotNulTerminated) => {
                    crate::utils::flatten_panicvals(&[&[PanicVal::write_str(
                        "data provided is not nul terminated",
                    )]])
                }
                (FmtKind::Debug, FromBytesWithNulError::InteriorNul { position }) => {
                    flatten_panicvals! {fmtarg;
                        "InteriorNul",
                        open: cfmt::OpenBrace,
                            "position: ", usize => position, cfmt::COMMA_TERM,
                        close: cfmt::CloseBrace,
                    }
                }
                (FmtKind::Debug, FromBytesWithNulError::NotNulTerminated) => {
                    crate::utils::flatten_panicvals(&[&[PanicVal::write_str("NotNulTerminated")]])
                }
            }
        }
    }

    #[cfg_attr(
        feature = "docsrs",
        doc(cfg(all(feature = "non_basic", feature = "rust_1_82")))
    )]
    impl PanicFmt for FromBytesUntilNulError {
        type This = Self;
        type Kind = crate::fmt::IsStdType;
        const PV_COUNT: usize = ComputePvCount {
            field_amount: 1,
            summed_pv_count: <()>::PV_COUNT,
            delimiter: cfmt::TypeDelim::Tupled,
        }
        .call();
    }

    #[cfg_attr(
        feature = "docsrs",
        doc(cfg(all(feature = "non_basic", feature = "rust_1_82")))
    )]
    impl StdWrapper<&FromBytesUntilNulError> {
        /// Formats a `FromBytesUntilNulError`.
        pub const fn to_panicvals(
            self,
            fmtarg: FmtArg,
        ) -> [PanicVal<'static>; FromBytesUntilNulError::PV_COUNT] {
            match fmtarg.fmt_kind {
                FmtKind::Display => crate::utils::flatten_panicvals(&[&[PanicVal::write_str(
                    "data provided does not contain a nul",
                )]]),
                FmtKind::Debug => flatten_panicvals! {fmtarg;
                    "FromBytesUntilNulError",
                    open: cfmt::OpenParen,
                        () => (), cfmt::COMMA_TERM,
                    close: cfmt::CloseParen,
                },
            }
        }
    }
};
