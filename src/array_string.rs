use crate::PanicVal;

use core::{
    cmp::PartialEq,
    fmt::{self, Debug},
};

/// For precomputing a panic message.
pub struct ArrayString<const LEN: usize> {
    pub(crate) len: usize,
    pub(crate) buffer: [u8; LEN],
}

impl<const LEN: usize> ArrayString<LEN> {
    /// Constructs this string from a `&[&[PanicVal<'_>]]`.
    pub const fn concat_panicvals(args: &[&[PanicVal<'_>]]) -> Option<Self> {
        match crate::concat_panic_::make_panic_string::<LEN>(args) {
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
        self.len
    }

    const fn as_bytes(&self) -> &[u8] {
        let mut to_truncate = LEN - self.len;
        let mut out: &[u8] = &self.buffer;

        while to_truncate != 0 {
            if let [rem @ .., _] = out {
                out = rem;
            }
            to_truncate -= 1;
        }

        if out.len() != self.len {
            panic!("BUG!")
        }

        out
    }

    /// Gets the string.
    ///
    /// # Performance
    ///
    /// This takes a linear amount of time to run,
    /// proportional to `LEN - self.len()`.
    pub const fn get(&self) -> &str {
        // safety: make_panic_string delegates formatting to the `write_to_buffer`,
        // which is tested as producing valid utf8.
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

impl<const LEN: usize> Debug for ArrayString<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.get(), f)
    }
}

impl<const LEN: usize> PartialEq<str> for ArrayString<LEN> {
    fn eq(&self, str: &str) -> bool {
        self.get() == str
    }
}
impl<const LEN: usize> PartialEq<&str> for ArrayString<LEN> {
    fn eq(&self, str: &&str) -> bool {
        self == *str
    }
}
