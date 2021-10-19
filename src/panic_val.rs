use crate::{
    fmt::{FmtArg, FmtKind},
    utils::{ShortArrayVec, Sign, WasTruncated},
};

#[non_exhaustive]
#[derive(Copy, Clone)]
pub struct PanicVal<'a> {
    pub(crate) var: PanicVariant<'a>,
    leftpad: u8,
    rightpad: u8,
    fmt_kind: FmtKind,
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub(crate) enum PanicVariant<'a> {
    Str(&'a str),
    Int(IntVal),
    #[cfg(feature = "non_basic")]
    Slice(crate::slice_stuff::Slice<'a>),
}

impl<'a> PanicVal<'a> {
    pub const EMPTY: Self = PanicVal::from_str("", FmtArg::DISPLAY);

    /// Constructs a PanicVal which outputs the contents of `string` verbatim.
    ///
    /// Equivalent to `PanicVal::from_str(string, FmtArg::DISPLAY)`
    pub const fn write_str(string: &'a str) -> Self {
        PanicVal {
            var: PanicVariant::Str(string),
            leftpad: 0,
            rightpad: 0,
            fmt_kind: FmtKind::Display,
        }
    }

    /// How many spaces are printed before this
    pub const fn leftpad(&self) -> u8 {
        self.leftpad
    }
    /// How many spaces are printed after this
    pub const fn rightpad(&self) -> u8 {
        self.rightpad
    }

    pub(crate) const fn __new(var: PanicVariant<'a>, f: FmtArg) -> Self {
        Self {
            var,
            leftpad: 0,
            rightpad: 0,
            fmt_kind: f.fmt_kind,
        }
    }

    pub(crate) const fn __string(
        &self,
        mut truncate_to: usize,
    ) -> (usize, usize, &[u8], FmtKind, WasTruncated) {
        let leftpad = self.leftpad as usize;
        if leftpad > truncate_to {
            return (
                leftpad - truncate_to,
                0,
                &[],
                FmtKind::Display,
                WasTruncated::Yes(0),
            );
        } else {
            truncate_to -= leftpad;
        };

        let string;
        let was_trunc;
        let fmt_kind;

        match &self.var {
            PanicVariant::Str(str) => {
                string = str.as_bytes();
                fmt_kind = self.fmt_kind;
                was_trunc = if let FmtKind::Display = self.fmt_kind {
                    crate::utils::truncated_str_len(string, truncate_to)
                } else {
                    crate::utils::truncated_debug_str_len(string, truncate_to)
                };
            }
            PanicVariant::Int(int) => {
                string = int.0.get();
                fmt_kind = FmtKind::Display;
                was_trunc = if int.0.len() <= truncate_to {
                    WasTruncated::No
                } else {
                    WasTruncated::Yes(0)
                };
            }
            #[cfg(feature = "non_basic")]
            PanicVariant::Slice(_) => panic!("this method should only be called on non-slices"),
        };
        truncate_to -= was_trunc.get_length(string);

        let rightpad = crate::utils::min_usize(self.rightpad as usize, truncate_to);

        (leftpad, rightpad, string, fmt_kind, was_trunc)
    }
}

#[derive(Copy, Clone)]
pub struct IntVal(ShortArrayVec<40>);

impl IntVal {
    pub(crate) const fn from_u128(n: u128, f: FmtArg) -> Self {
        Self::new(Sign::Positive, n, f)
    }
    pub(crate) const fn from_i128(n: i128, f: FmtArg) -> Self {
        let is_neg = if n < 0 {
            Sign::Negative
        } else {
            Sign::Positive
        };
        Self::new(is_neg, n.unsigned_abs(), f)
    }
    const fn new(sign: Sign, mut n: u128, _f: FmtArg) -> Self {
        let mut start = 40usize;
        let mut buffer = [0u8; 40];

        loop {
            start -= 1;
            let digit = (n % 10) as u8;
            buffer[start] = b'0' + digit;
            n /= 10;
            if n == 0 {
                break;
            }
        }

        if let Sign::Negative = sign {
            start -= 1;
            buffer[start] = b'-';
        }

        Self(ShortArrayVec {
            start: start as u8,
            buffer,
        })
    }
}

impl crate::PanicFmt for PanicVal<'_> {
    type This = Self;
    type Kind = crate::fmt::IsCustomType;

    const PV_COUNT: usize = 1;
}

impl<'a> PanicVal<'a> {
    pub const fn to_panicvals(&self, _: FmtArg) -> [PanicVal<'a>; 1] {
        [*self]
    }
    pub const fn to_panicval(&self, _: FmtArg) -> PanicVal<'a> {
        *self
    }
}
