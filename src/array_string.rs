use crate::{FmtArg, PanicFmt, PanicVal};

use core::{
    cmp::PartialEq,
    fmt::{self, Debug},
};

/// For precomputing a panic message.
#[derive(Copy, Clone)]
pub struct ArrayString<const CAP: usize> {
    pub(crate) len: u32,
    pub(crate) buffer: [u8; CAP],
}

const fn add_up_lengths(mut strings: &[&str]) -> usize {
    let mut len = 0;
    while let [x, rem @ ..] = strings {
        len += x.len();
        strings = rem;
    }
    len
}

impl<const CAP: usize> ArrayString<CAP> {
    /// Constructs an `ArrayString` from a `&str`
    ///
    /// # Panics
    ///
    /// Panics if `string` is larger than `CAP`.
    pub const fn new(string: &str) -> Self {
        Self::concat(&[string])
    }

    /// Constructs an `ArrayString` by concatenating zero or more `&str`s
    ///
    /// # Panics
    ///
    /// Panics if the concatenated string would be longer than `CAP`.
    pub const fn concat(strings: &[&str]) -> Self {
        let mut len = 0u32;
        let mut buffer = [0u8; CAP];

        let mut mstrings = strings;
        while let [string, ref rem @ ..] = *mstrings {
            mstrings = rem;
            let mut bytes = string.as_bytes();
            while let [x, ref rem @ ..] = *bytes {
                if len == u32::MAX || len as usize >= CAP {
                    crate::concat_panic(&[&[
                        PanicVal::write_str("The input strings were longer than "),
                        PanicVal::from_usize(CAP, FmtArg::DISPLAY),
                        PanicVal::write_str(", concatenated length: "),
                        PanicVal::from_usize(add_up_lengths(strings), FmtArg::DISPLAY),
                        PanicVal::write_str(", strings: "),
                        PanicVal::from_slice_str(strings, FmtArg::DEBUG),
                    ]])
                }

                bytes = rem;
                buffer[len as usize] = x;
                len += 1;
            }
        }

        Self { len, buffer }
    }

    /// Constructs this string from a `&[&[PanicVal<'_>]]`.
    pub const fn concat_panicvals(args: &[&[PanicVal<'_>]]) -> Option<Self> {
        match crate::concat_panic_::make_panic_string::<CAP>(args) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    /// Constructs this string from a `&[PanicVal<'_>]`.
    pub const fn from_panicvals(args: &[PanicVal<'_>]) -> Option<Self> {
        Self::concat_panicvals(&[args])
    }

    /// How long the string is in bytes.
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Accesses the string as a byte slice.
    ///
    /// # Performance
    ///
    /// This takes a linear amount of time to run, proportional to `CAP - self.len()`.
    pub const fn as_bytes(&self) -> &[u8] {
        let mut to_truncate = CAP - self.len();
        let mut out: &[u8] = &self.buffer;

        while to_truncate != 0 {
            if let [rem @ .., _] = out {
                out = rem;
            }
            to_truncate -= 1;
        }

        if out.len() != self.len() {
            panic!("BUG!")
        }

        out
    }

    /// Gets the string.
    ///
    /// # Performance
    ///
    /// This takes a linear amount of time to run, proportional to `CAP - self.len()`.
    pub const fn get(&self) -> &str {
        // safety: make_panic_string delegates formatting to the `write_to_buffer` macro,
        // which is tested as producing valid utf8.
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub const fn to_panicvals(&self, f: FmtArg) -> [PanicVal<'_>; 1] {
        [PanicVal::from_str(self.get(), f)]
    }
    pub const fn to_panicval(&self, f: FmtArg) -> PanicVal<'_> {
        PanicVal::from_str(self.get(), f)
    }
}

impl<const CAP: usize> Debug for ArrayString<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.get(), f)
    }
}

impl<const CAP: usize> PartialEq<str> for ArrayString<CAP> {
    fn eq(&self, str: &str) -> bool {
        self.get() == str
    }
}
impl<const CAP: usize> PartialEq<&str> for ArrayString<CAP> {
    fn eq(&self, str: &&str) -> bool {
        self == *str
    }
}

impl<const CAP: usize> PanicFmt for ArrayString<CAP> {
    type This = Self;
    type Kind = crate::fmt::IsCustomType;
    const PV_COUNT: usize = 1;
}
