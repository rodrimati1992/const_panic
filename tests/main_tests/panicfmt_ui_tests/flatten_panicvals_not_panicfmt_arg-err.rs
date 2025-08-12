use const_panic::{FmtArg, PanicVal, flatten_panicvals};

struct NotFmt;

const _: &[PanicVal<'_>] = &flatten_panicvals!(FmtArg::DEBUG; NotFmt);

const _: &[PanicVal<'_>] = &flatten_panicvals!(FmtArg::DEBUG; NotFmt => NotFmt);

fn main() {}