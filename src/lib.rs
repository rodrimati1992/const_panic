//! For panicking with formatting in const contexts.
//!
//! This library exists because the panic macro was stabilized for use in const contexts
//! in Rust 1.57.0, without formatting support.
//!
//! All of the types that implement the [`PanicFmt`] trait can be formatted in panics.
//!
//! # Examples
//!
//! ### Basic
//!
//! ```compile_fail
//! use const_panic::concat_panic;
//!
//! const FOO: u32 = 10;
//! const BAR: u32 = 0;
//! const _: () = assert_non_zero(FOO, BAR);
//!
//! #[track_caller]
//! const fn assert_non_zero(foo: u32, bar: u32) {
//!     if foo == 0 || bar == 0 {
//!         concat_panic!("\nneither foo nor bar can be zero!\nfoo: ", foo, "\nbar: ", bar)
//!     }
//! }
//! ```
//! The above code fails to compile with this error:
//! ```text
//! error[E0080]: evaluation of constant value failed
//!  --> src/lib.rs:17:15
//!   |
//! 8 | const _: () = assert_non_zero(FOO, BAR);
//!   |               ^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
//! neither foo nor bar can be zero!
//! foo: 10
//! bar: 0', src/lib.rs:8:15
//! ```
//!
//! ### Parser
//!
//! Implementing a basic parser, using the `konst` crate
//! (version "0.2.13", which doesn't depend on `const_panic`)
//!
//! ```compile_fail
//! use konst::{
//!     parsing::{Parser, ErrorKind, ParseError},
//!     result,
//!     try_,
//! };
//!
//! const FOO: [u32; 2] = result::unwrap_or_else!(parse(",100"), |e| panic_(e));
//! const BAR: [u32; 2] = result::unwrap_or_else!(parse("100"), |e| panic_(e));
//!
//!
//! const fn parse(text: &str) -> Result<[u32; 2], ParseError> {
//!     let parser = Parser::from_str(text);
//!     
//!     let (first, parser) = try_!(parser.parse_u32());
//!     let parser = try_!(parser.strip_prefix_u8(b','));
//!     let (second, parser) = try_!(parser.parse_u32());
//!
//!     Ok([first, second])
//! }
//!
//! #[track_caller]
//! const fn panic_(err: ParseError) -> ! {
//!     let start = match err.kind() {
//!         ErrorKind::ParseInteger => "\ncould not parse integer",
//!         ErrorKind::Strip => "\nthere was no comma",
//!         _ => "error while parsing"
//!     };
//!
//!     const_panic::concat_panic!{
//!         // using `display:` to override the default formatter (Debug)
//!         // for non-literals.
//!         display: start,
//!         // literals are Display formatted by default.
//!         " at byte ",
//!         err.offset(),
//!     }
//! }
//!
//! ```
//! The above code fails to compile with these errors:
//! ```text
//! error[E0080]: evaluation of constant value failed
//!   --> src/lib.rs:19:66
//!    |
//! 10 | const FOO: [u32; 2] = result::unwrap_or_else!(parse(",100"), |e| panic_(e));
//!    |                                                                  ^^^^^^^^^ the evaluated program panicked at '
//! could not parse integer at byte 0', src/lib.rs:10:66
//!
//! error[E0080]: evaluation of constant value failed
//!   --> src/lib.rs:20:65
//!    |
//! 11 | const BAR: [u32; 2] = result::unwrap_or_else!(parse("100"), |e| panic_(e));
//!    |                                                                 ^^^^^^^^^ the evaluated program panicked at '
//! there was no comma at byte 3', src/lib.rs:11:65
//!
//!
//! ```
//!
//!
//!
//!
//!

#![no_std]

#[macro_use]
mod doc_macros;

#[macro_use]
mod macros;

mod concat_panic_;

mod debug_str_fmt;

pub mod fmt;

#[cfg(feature = "non_basic")]
mod non_basic_utils;

mod panic_val;

mod utils;

#[cfg(all(test, not(feature = "test")))]
compile_error! {r##"please use cargo test --features "test""##}

#[cfg(feature = "non_basic")]
mod slice_stuff;

#[cfg(any(doctest, feature = "array_string"))]
mod array_string;

#[cfg(any(doctest, feature = "array_string"))]
pub use crate::array_string::ArrayString;

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
    pub use core::compile_error;

    pub use crate::fmt::{FmtArg, PanicFmt};

    #[cfg(feature = "non_basic")]
    pub use crate::non_basic_utils::{flatten_panicvals, panicvals_id};
}

#[cfg(feature = "test")]
pub mod test_utils;

#[doc(hidden)]
#[cfg(feature = "test")]
pub mod for_tests {
    pub use crate::concat_panic_::{format_panic_message, NotEnoughSpace};
}
