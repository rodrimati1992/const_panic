use const_panic::{FmtArg, PanicVal, flatten_panicvals};

struct NotFmt;

const _: &[PanicVal<'_>] = &flatten_panicvals!(FmtArg::DEBUG; 0);

const _: &[PanicVal<'_>] = &flatten_panicvals!(FmtArg::DEBUG, 10; 0);

fn main() {}