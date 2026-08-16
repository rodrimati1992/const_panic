#![no_std]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::explicit_auto_deref)]

extern crate alloc;

#[cfg(test)]
extern crate std;

use proc_macro::TokenStream as TokenStream1;

use proc_macro2::TokenStream as TokenStream2;

mod datastructure;

mod derive_debug;

mod syntax;

mod utils;

#[cfg(test)]
mod test_utils;

#[proc_macro_derive(PanicFmt, attributes(pfmt))]
pub fn derive_const_debug(input: TokenStream1) -> TokenStream1 {
    syn::parse(input)
        .and_then(derive_debug::derive_constdebug_impl)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
