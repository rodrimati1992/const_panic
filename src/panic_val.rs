use crate::utils::{truncate_str, ShortArrayVec, Sign, WasTruncated};

#[derive(Copy, Clone)]
#[non_exhaustive]
pub enum PanicVal<'a> {
    Str(&'a str),
    Int(IntVal),
}

impl<'a> PanicVal<'a> {
    pub const EMPTY: Self = PanicVal::Str("");

    pub(crate) const fn string(&self, truncate_to: usize) -> (&[u8], WasTruncated) {
        match self {
            Self::Str(str) => truncate_str(str.as_bytes(), truncate_to),
            Self::Int(int) => truncate_str(int.0.get(), truncate_to),
        }
    }
    pub(crate) const fn len(&self) -> usize {
        match self {
            Self::Str(str) => str.len(),
            Self::Int(int) => int.0.len(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct IntVal(ShortArrayVec<40>);

impl IntVal {
    pub(crate) const fn from_u128(n: u128) -> Self {
        Self::new(Sign::Positive, n)
    }
    pub(crate) const fn from_i128(n: i128) -> Self {
        let is_neg = if n < 0 {
            Sign::Negative
        } else {
            Sign::Positive
        };
        Self::new(is_neg, n.unsigned_abs())
    }
    const fn new(sign: Sign, mut n: u128) -> Self {
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
