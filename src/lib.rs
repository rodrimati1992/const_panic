mod concat_panic_;
pub mod fmt;
mod macros;
mod panic_val;
mod utils;
mod wrapper;

mod fmt_impls {
    mod basic_fmt_impls;
}

pub use crate::{
    concat_panic_::concat_panic,
    fmt::{FmtArg, PanicFmt},
    panic_val::{IntVal, PanicVal},
    wrapper::Wrapper,
};

#[doc(hidden)]
pub mod __ {
    pub use crate::{
        fmt::{FmtArg, PanicFmt},
        utils::{flatten_panicvals, panicvals_id},
    };
}
