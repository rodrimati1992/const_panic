use crate::{FmtArg, PanicVal};

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

pub const fn max_usize(l: usize, r: usize) -> usize {
    if l > r {
        l
    } else {
        r
    }
}

pub const fn slice_max_usize(mut slice: &[usize]) -> usize {
    let mut max = 0;

    while let [x, ref rem @ ..] = *slice {
        max = max_usize(max, x);
        slice = rem;
    }

    max
}

#[doc(hidden)]
#[track_caller]
pub const fn assert_flatten_panicvals_length(expected_larger: usize, actual_value: usize) {
    if actual_value > expected_larger {
        crate::concat_panic(&[&[
            PanicVal::write_str("length passed to flatten_panicvals macro ("),
            PanicVal::from_usize(expected_larger, FmtArg::DISPLAY),
            PanicVal::write_str(") is smaller than the computed length ("),
            PanicVal::from_usize(actual_value, FmtArg::DISPLAY),
            PanicVal::write_str(")"),
        ]]);
    }
}
