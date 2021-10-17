use crate::PanicVal;

pub const fn panicvals_id<'a, 'b, const LEN: usize>(
    array: &'b [PanicVal<'a>; LEN],
) -> &'b [PanicVal<'a>] {
    array
}

pub const fn flatten_panicvals<'a, const LEN: usize>(
    mut input: &[&[PanicVal<'a>]],
) -> [PanicVal<'a>; LEN] {
    let mut out = [PanicVal::EMPTY; LEN];
    let mut len = 0usize;

    while let [mut outer, ref rinput @ ..] = *input {
        while let [arg, ref router @ ..] = *outer {
            out[len] = arg;
            len += 1;
            outer = router;
        }
        input = rinput
    }

    out
}

pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    if l < r {
        l
    } else {
        r
    }
}

#[derive(Copy, Clone)]
pub(crate) struct ShortArrayVec<const LEN: usize> {
    pub(crate) start: u8,
    pub(crate) buffer: [u8; LEN],
}

impl<const LEN: usize> ShortArrayVec<LEN> {
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

    pub(crate) const fn len(&self) -> usize {
        LEN - self.start as usize
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Sign {
    Negative,
    Positive,
}

#[derive(Copy, Clone)]
pub(crate) enum WasTruncated {
    Yes,
    No,
}

// truncates a utf8-encoded string to the character before the `truncate_to` index
//
pub(crate) const fn truncate_str(mut bytes: &[u8], truncate_to: usize) -> (&[u8], WasTruncated) {
    if bytes.len() <= truncate_to {
        (bytes, WasTruncated::No)
    } else {
        while bytes.len() > truncate_to {
            while let [ref rem @ .., b] = *bytes {
                bytes = rem;

                // if it's a non-continuation byte, break
                if (b as i8) >= -0x40 {
                    break;
                }
            }
        }

        (bytes, WasTruncated::Yes)
    }
}
