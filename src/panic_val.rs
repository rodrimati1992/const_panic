use crate::{
    fmt::{FmtArg, FmtKind},
    utils::{Sign, TailShortString, WasTruncated},
};

#[cfg(feature = "non_basic")]
use crate::fmt::{IsLast, ShortString};

/// An opaque enum of the values that this crate knows how to format,
/// along with some formatting metadata.
///
/// This has constructor functions to make a `PanicVal` from:
/// - `bool`
/// - Integers
/// - `&str`
/// - Arrays/Slices of primitives (with the "non_basic" feature, enabled by default)
/// - [`ShortString`](crate::fmt::ShortString)
/// (with the "non_basic" feature, enabled by default)
///
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
    #[cfg(feature = "non_basic")]
    ShortString(ShortString),
    Int(IntVal),
    #[cfg(feature = "non_basic")]
    Slice(crate::slice_stuff::Slice<'a>),
}

impl<'a> PanicVal<'a> {
    /// A `PanicVal` that formats to nothing.
    pub const EMPTY: Self = PanicVal::write_str("");

    /// How many spaces are printed before this
    pub const fn leftpad(&self) -> u8 {
        self.leftpad
    }
    /// How many spaces are printed after this
    pub const fn rightpad(&self) -> u8 {
        self.rightpad
    }
    /// Sets the amount of spaces printed before this to `fmtarg.indentation`.
    pub const fn with_leftpad(mut self, fmtarg: FmtArg) -> Self {
        self.leftpad = fmtarg.indentation;
        self
    }

    /// Sets the amount of spaces printed after this to `fmtarg.indentation`.
    pub const fn with_rightpad(mut self, fmtarg: FmtArg) -> Self {
        self.rightpad = fmtarg.indentation;
        self
    }

    /// Sets the amount of spaces printed before this
    pub const fn set_leftpad(mut self, fmtarg: FmtArg) -> Self {
        self.leftpad = fmtarg.indentation;
        self
    }

    /// Sets the amount of spaces printed after this
    pub const fn set_rightpad(mut self, fmtarg: FmtArg) -> Self {
        self.rightpad = fmtarg.indentation;
        self
    }

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

    /// Constructs a PanicVal from a [`ShortString`], which outputs the string verbatim.
    #[cfg(feature = "non_basic")]
    pub const fn write_short_str(string: ShortString) -> Self {
        Self {
            var: PanicVariant::ShortString(string),
            ..Self::EMPTY
        }
    }

    /// Constructs a `PanicVal` usable as a separator between fields or elements.
    ///
    /// This is sensitive to te [`fmtarg.is_alternate`] flag,
    /// for more details on that you can look at the docs for
    /// [`Separator::to_panicval`](crate::fmt::Separator#method.to_panicval)
    ///
    /// # Panics
    ///
    /// This may panic if `string.len()` is greater than 12.
    ///
    /// [`fmtarg.is_alternate`]: crate::FmtArg#structfield.is_alternate
    #[cfg(feature = "non_basic")]
    pub const fn from_element_separator(
        separator: &str,
        is_last_field: IsLast,
        fmtarg: FmtArg,
    ) -> Self {
        let (concat, rightpad) = match (is_last_field, fmtarg.is_alternate) {
            (IsLast::No, false) => (ShortString::concat(&[separator, " "]), 0),
            (IsLast::Yes, false) => (ShortString::new(""), 0),
            (IsLast::No, true) => (ShortString::concat(&[separator, "\n"]), fmtarg.indentation),
            (IsLast::Yes, true) => (ShortString::concat(&[separator, "\n"]), 0),
        };

        Self {
            var: PanicVariant::ShortString(concat),
            leftpad: 0,
            rightpad,
            fmt_kind: FmtKind::Display,
        }
    }

    pub(crate) const fn __new(var: PanicVariant<'a>, fmtarg: FmtArg) -> Self {
        Self {
            var,
            leftpad: 0,
            rightpad: 0,
            fmt_kind: fmtarg.fmt_kind,
        }
    }

    // Gets the bytes in the string and integer variants,
    // truncating them to `truncate_to`.
    //
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
            #[cfg(feature = "non_basic")]
            PanicVariant::ShortString(str) => {
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
pub struct IntVal(TailShortString<40>);

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

        // safety: buffer is only ever written ascii, so its automatically valid utf8.
        unsafe { Self(TailShortString::new(start as u8, buffer)) }
    }
}

impl crate::PanicFmt for PanicVal<'_> {
    type This = Self;
    type Kind = crate::fmt::IsCustomType;

    const PV_COUNT: usize = 1;
}

impl<'a> PanicVal<'a> {
    /// Wraps this `PanicVal` in a single-element array.
    pub const fn to_panicvals(&self, _: FmtArg) -> [PanicVal<'a>; 1] {
        [*self]
    }
    /// Returns a copy of this `PanicVal`.
    pub const fn to_panicval(&self, _: FmtArg) -> PanicVal<'a> {
        *self
    }
}
