#![no_std]

#[macro_use]
mod macros;

mod concat_panic_;
mod debug_str_fmt;
pub mod fmt;
#[cfg(feature = "all_items")]
mod non_basic_utils;
mod panic_val;
mod utils;

#[cfg(all(test, not(feature = "test")))]
compile_error! {r##"please use cargo test --features "test""##}

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

#[cfg(feature = "test")]
pub mod test_utils;

#[doc(hidden)]
#[cfg(feature = "test")]
pub mod for_tests {
    pub use crate::concat_panic_::{format_panic_message, NotEnoughSpace};
}
