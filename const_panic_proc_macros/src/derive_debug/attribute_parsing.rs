use crate::{
    datastructure::{DataStructure, DataVariant, Field, Struct},
    utils::{ParseBufferExt, SynResultExt},
};

use syn::{
    parse::{ParseBuffer, Parser},
    Attribute, Token,
};

use core::marker::PhantomData;

mod keyword {
    syn::custom_keyword!(debug_print);
}


#[derive(Copy, Clone)]
pub(crate) enum ParseCtx<'a> {
    Container,
    Variant(usize, &'a Struct<'a>),
    Field(&'a Field<'a>),
}

struct ParsedAttributes<'a> {
    debug_print: bool,
    crate_path: syn::Path,
    _marker: PhantomData<&'a ()>,
}

pub(super) struct Configuration<'a> {
    pub(super) debug_print: bool,
    pub(super) crate_path: syn::Path,
    _marker: PhantomData<&'a ()>,
}

pub(super) fn parse_attributes<'a>(ds: &'a DataStructure<'a>) -> syn::Result<Configuration<'a>> {
    let mut this = ParsedAttributes{
        debug_print: false,
        crate_path: syn::parse_quote!(::const_panic),
        _marker: PhantomData,
    };

    let mut res = syn::Result::Ok(());

    for attr in ds.attrs {
        res.combine_err(parse_attribute(&mut this, ds, ParseCtx::Container, attr));
    }

    if ds.data_variant == DataVariant::Enum {
        for (i, v) in ds.variants.iter().enumerate() {
            let ctx = ParseCtx::Variant(i, v);
            for attr in v.attrs {
                res.combine_err(parse_attribute(&mut this, ds, ctx, attr));
            }
        }
    }

    for v in &ds.variants {
        for f in &v.fields {
            for attr in f.attrs {
                res.combine_err(parse_attribute(&mut this, ds, ParseCtx::Field(f), attr));
            }
        }
    }

    res?;

    finish(this, ds)
}

fn parse_attribute<'a>(
    this: &mut ParsedAttributes<'a>,
    ds: &'a DataStructure<'a>,
    ctx: ParseCtx<'a>,
    attribute: &Attribute,
) -> syn::Result<()> {
    if attribute.path.is_ident("pfmt") {
        let closure = move|input: &'_ ParseBuffer<'_>| {
            parse_helper_attribute(this, ds, ctx, input)
        };

        if attribute.tokens.is_empty() {
            Parser::parse2(closure, crate::TokenStream2::new())
        } else {
            attribute.parse_args_with(closure)
        }
    } else {
        Ok(())
    }
}

fn parse_helper_attribute<'a>(
    this: &mut ParsedAttributes<'a>,
    _ds: &'a DataStructure<'a>,
    ctx: ParseCtx<'a>,
    input: &'_ ParseBuffer<'_>,
) -> syn::Result<()> {
    let empty = &crate::utils::Empty(input.span());

    if let Some(_) = input.peek_parse(keyword::debug_print)? {
        check_is_container(&ctx, empty)?;

        this.debug_print = true;
    } else if let Some(_) = input.peek_parse(Token!(crate))? {
        check_is_container(&ctx, empty)?;

        input.parse::<Token!(=)>()?;
        this.crate_path = input.parse::<syn::Path>()?;
    } else {
        let span = input.parse::<syn::Ident>()?.span();
        return Err(syn::Error::new(span, "Invalid attribute"));
    }
    Ok(())
}

fn finish<'a>(
    this: ParsedAttributes<'a>,
    _ds: &'a DataStructure<'a>,
) -> syn::Result<Configuration<'a>> {
    let ParsedAttributes {
        debug_print,
        crate_path,
        _marker,
    } = this;

    Ok(Configuration{
        debug_print,
        crate_path,
        _marker,
    })
}


pub(crate) fn check_is_container(
    ctx: &ParseCtx<'_>,
    sp: &dyn syn::spanned::Spanned,
) -> syn::Result<()> {
    if matches!(ctx, ParseCtx::Container) {
        Ok(())
    } else {
        Err(syn::Error::new(sp.span(), "Can only use this attribute above the type definition"))
    }
}

