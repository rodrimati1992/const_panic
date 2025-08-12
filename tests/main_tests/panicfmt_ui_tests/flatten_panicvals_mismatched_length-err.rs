use const_panic::{FmtArg, PanicVal, flatten_panicvals};

const _: [PanicVal<'_>; 20] = flatten_panicvals!(FmtArg::DEBUG; 10u8, 20u8);

fn main() {}