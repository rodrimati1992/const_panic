#[macro_use]
mod macros;

mod concat_panic_;
pub mod fmt;
#[cfg(feature = "all_items")]
mod non_basic_utils;
mod panic_val;
mod utils;

#[cfg(feature = "all_items")]
mod slice_stuff;

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
    pub use crate::fmt::{FmtArg, PanicFmt};

    #[cfg(feature = "all_items")]
    pub use crate::non_basic_utils::{flatten_panicvals, panicvals_id};
}
