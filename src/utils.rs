//! Utility functions

use crate::debug_str_fmt::ForEscaping;

#[cfg(feature = "non_basic")]
mod non_basic_utils;

#[cfg(feature = "non_basic")]
pub use self::non_basic_utils::*;

pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    if l < r {
        l
    } else {
        r
    }
}

#[derive(Copy, Clone)]
pub(crate) struct TailShortString<const LEN: usize> {
    start: u8,
    buffer: [u8; LEN],
}

impl<const LEN: usize> TailShortString<LEN> {
    ///
    /// # Safety
    ///
    /// `buffer` must be valid utf8 starting from the `start` index.
    #[inline(always)]
    pub(crate) const unsafe fn new(start: u8, buffer: [u8; LEN]) -> Self {
        Self { start, buffer }
    }

    pub(crate) const fn len(&self) -> usize {
        LEN - self.start as usize
    }

    pub(crate) const fn get(&self) -> &[u8] {
        let mut rem = self.start;
        let mut out: &[u8] = &self.buffer;
        while rem != 0 {
            if let [_, rem @ ..] = out {
                out = rem;
            }
            rem -= 1;
        }
        out
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Sign {
    Negative,
    Positive,
}

#[derive(Copy, Clone)]
pub(crate) enum WasTruncated {
    Yes(usize),
    No,
}

impl WasTruncated {
    pub(crate) const fn get_length(self, s: &[u8]) -> usize {
        match self {
            WasTruncated::Yes(x) => x,
            WasTruncated::No => s.len(),
        }
    }
}

const fn is_char_boundary(b: u8) -> bool {
    (b as i8) >= -0x40
}

// truncates a utf8-encoded string to the character before the `truncate_to` index
//
pub(crate) const fn truncated_str_len(bytes: &[u8], truncate_to: usize) -> WasTruncated {
    if bytes.len() <= truncate_to {
        WasTruncated::No
    } else {
        let mut i = truncate_to;
        while i != 0 {
            // if it's a non-continuation byte, break
            if is_char_boundary(bytes[i]) {
                break;
            }
            i -= 1;
        }

        WasTruncated::Yes(i)
    }
}

pub(crate) const fn truncated_debug_str_len(bytes: &[u8], truncate_to: usize) -> WasTruncated {
    let blen = bytes.len();

    // `* 4` because the longest escape is written like `\xNN` which is 4 bytes
    // `+ 2` for the quote characters
    if blen * 4 + 2 <= truncate_to {
        WasTruncated::No
    } else if truncate_to == 0 {
        WasTruncated::Yes(0)
    } else {
        let mut i = 0;
        // = 1 for opening quote char
        let mut fmtlen = 1;
        loop {
            let next_i = next_char_boundary(bytes, min_usize(i + 1, bytes.len()));

            let mut j = i;
            while j < next_i {
                fmtlen += ForEscaping::byte_len(bytes[j]);
                j += 1;
            }

            if fmtlen > truncate_to {
                break;
            } else if next_i == bytes.len() {
                i = next_i;
                break;
            } else {
                i = next_i;
            }
        }

        if i == blen && fmtlen < truncate_to {
            WasTruncated::No
        } else {
            WasTruncated::Yes(i)
        }
    }
}

const fn next_char_boundary(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && !is_char_boundary(bytes[i]) {
        i += 1;
    }
    i
}
