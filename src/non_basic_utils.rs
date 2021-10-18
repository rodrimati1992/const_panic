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
