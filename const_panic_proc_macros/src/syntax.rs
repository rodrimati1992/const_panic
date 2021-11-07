use proc_macro2::TokenTree as TokenTree2;

use syn::{parse::ParseStream, token::Impl, Generics, Ident};

use alloc::format;

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ImplHeader {
    pub(crate) generics: Generics,
    pub(crate) self_args: syn::AngleBracketedGenericArguments,
}

impl ImplHeader {
    pub(crate) fn parse<'a>(name: &'a Ident, input: ParseStream<'_>) -> syn::Result<Self> {
        let _ = input.parse::<Impl>()?;
        let mut generics = input.parse::<Generics>()?;

        input.step(|cursor| match cursor.token_tree() {
            Some((TokenTree2::Ident(ident), cursor)) if *name == ident => Ok(((), cursor)),
            _ => Err(cursor.error(format!("expected `{}`", name))),
        })?;

        let self_args = input.parse()?;

        generics.where_clause = input.parse()?;

        Ok(Self {
            generics,
            self_args,
        })
    }
}
