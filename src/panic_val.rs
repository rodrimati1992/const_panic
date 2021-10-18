use crate::{
    utils::{truncate_str, ShortArrayVec, Sign, WasTruncated},
    FmtArg,
};

#[non_exhaustive]
#[derive(Copy, Clone)]
pub struct PanicVal<'a> {
    pub(crate) var: PanicVariant<'a>,
    leftpad: u8,
    rightpad: u8,
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub(crate) enum PanicVariant<'a> {
    Str(&'a str),
    Int(IntVal),
    #[cfg(feature = "all_items")]
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

    pub(crate) const fn __new(var: PanicVariant<'a>, _f: FmtArg) -> Self {
        Self {
            var,
            leftpad: 0,
            rightpad: 0,
        }
    }

    pub(crate) const fn __string(
        &self,
        mut truncate_to: usize,
    ) -> (usize, usize, &[u8], WasTruncated) {
        let leftpad = self.leftpad as usize;
        if leftpad >= truncate_to {
            return (leftpad - truncate_to, 0, &[], WasTruncated::Yes);
        } else {
            truncate_to -= leftpad;
        };

        let (string, was_trunc) = match &self.var {
            PanicVariant::Str(str) => truncate_str(str.as_bytes(), truncate_to),
            PanicVariant::Int(int) => {
                if int.0.len() <= truncate_to {
                    (int.0.get(), WasTruncated::No)
                } else {
                    (&[] as &[u8], WasTruncated::Yes)
                }
            }
            #[cfg(feature = "all_items")]
            PanicVariant::Slice(_) => panic!("this method should only be called on non-slices"),
        };
        truncate_to -= string.len();

        let rightpad = crate::utils::min_usize(self.rightpad as usize, truncate_to);

        (leftpad, rightpad, string, was_trunc)
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
