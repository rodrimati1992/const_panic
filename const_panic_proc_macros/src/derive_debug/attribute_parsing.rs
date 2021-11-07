use crate::{
    datastructure::{DataStructure, DataVariant, Field, GenParamKind, Struct},
    syntax::ImplHeader,
    utils::{ParseBufferExt, SynResultExt},
    TokenStream2,
};

use syn::{
    parse::{ParseBuffer, Parser},
    Attribute, GenericParam, Ident, Token,
};

use quote::{quote, ToTokens};

use alloc::vec::Vec;

use core::marker::PhantomData;

mod keyword {
    syn::custom_keyword!(debug_print);
    syn::custom_keyword!(ignore);
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
    impls: Vec<ImplHeader>,
    gen_params_props: Vec<GenParamProps<'a>>,
    type_const_params: Vec<Ident>,
    _marker: PhantomData<&'a ()>,
}

pub(super) struct Configuration<'a> {
    pub(super) debug_print: bool,
    pub(super) crate_path: syn::Path,
    pub(super) impls: Vec<ImplHeader>,
    pub(super) gen_params_props: Vec<GenParamProps<'a>>,
    _marker: PhantomData<&'a ()>,
}

pub(super) fn parse_attributes<'a>(ds: &'a DataStructure<'a>) -> syn::Result<Configuration<'a>> {
    let mut this = ParsedAttributes {
        debug_print: false,
        crate_path: syn::parse_quote!(::const_panic),
        impls: Vec::new(),
        gen_params_props: ds
            .generics
            .params
            .iter()
            .map(|ga| {
                let (kind, ignored) = match ga {
                    GenericParam::Lifetime { .. } => {
                        (GenParamKind::Lifetime, GenParamIgnorance::Ignored)
                    }
                    GenericParam::Type { .. } => (GenParamKind::Type, GenParamIgnorance::Included),
                    GenericParam::Const { .. } => (GenParamKind::Const, GenParamIgnorance::Ignored),
                };

                GenParamProps { kind, ignored }
            })
            .collect(),
        type_const_params: ds
            .generics
            .params
            .iter()
            .filter_map(|ga| match ga {
                GenericParam::Lifetime { .. } => None,
                GenericParam::Type(x) => Some(x.ident.clone()),
                GenericParam::Const(x) => Some(x.ident.clone()),
            })
            .collect(),
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
        let closure =
            move |input: &'_ ParseBuffer<'_>| parse_helper_attribute(this, ds, ctx, input);

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
    ds: &'a DataStructure<'a>,
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
    } else if input.peek(Token!(impl)) {
        check_is_container(&ctx, empty)?;
        this.impls.push(ImplHeader::parse(ds.name, input)?);
    } else if let Some(_) = input.peek_parse(keyword::ignore)? {
        check_is_container(&ctx, empty)?;

        let contents;
        let _ = syn::parenthesized!(contents in input);

        if contents.is_empty() {
            return Ok(());
        }
        loop {
            let ident = contents.parse::<syn::Ident>()?;
            let gpi: usize = this
                .type_const_params
                .iter()
                .position(|x| *x == ident)
                .map(|pos| pos + ds.lifetime_count)
                .ok_or_else(|| {
                    syn::Error::new(ident.span(), "Expected name of a type or const parameter")
                })?;

            let gen_props = &mut this.gen_params_props[gpi];

            gen_props.ignored = if let Some(_) = contents.peek_parse(syn::Token!(=))? {
                let replacement = match gen_props.kind {
                    GenParamKind::Lifetime => unreachable!(),
                    GenParamKind::Type => contents.parse::<syn::Type>()?.into_token_stream(),
                    GenParamKind::Const => contents.parse::<syn::Expr>()?.into_token_stream(),
                };
                GenParamIgnorance::Replaced(replacement)
            } else {
                GenParamIgnorance::Ignored
            };

            if contents.is_empty() {
                break;
            } else {
                contents.parse::<syn::Token!(,)>()?;
            }
        }
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
        impls,
        gen_params_props,
        type_const_params: _,
        _marker,
    } = this;

    Ok(Configuration {
        debug_print,
        crate_path,
        impls,
        gen_params_props,
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
        Err(syn::Error::new(
            sp.span(),
            "Can only use this attribute above the type definition",
        ))
    }
}

pub(super) struct GenParamProps<'a> {
    pub(super) kind: GenParamKind,
    pub(super) ignored: GenParamIgnorance<'a>,
}

pub(super) enum GenParamIgnorance<'a> {
    Included,
    Ignored,
    Replaced(TokenStream2),
    #[allow(dead_code)]
    ReplacedB(&'a TokenStream2),
}

impl<'a> GenParamProps<'a> {
    #[allow(dead_code)]
    pub fn reborrow(&self) -> GenParamProps<'_> {
        GenParamProps {
            kind: self.kind,
            ignored: self.ignored.reborrow(),
        }
    }

    pub fn tokenize_arg<A>(&self, arg: &A) -> TokenStream2
    where
        A: ToTokens,
    {
        match (self.kind, &self.ignored) {
            (_, GenParamIgnorance::Included) => arg.to_token_stream(),
            (GenParamKind::Lifetime, GenParamIgnorance::Ignored) => quote!('_),
            (GenParamKind::Type, GenParamIgnorance::Ignored) => quote!(()),
            (GenParamKind::Const, GenParamIgnorance::Ignored) => {
                quote!({ __cp_bCj7dq3Pud::__::ConstDefault::DEFAULT })
            }
            (_, GenParamIgnorance::Replaced(x)) => x.to_token_stream(),
            (_, GenParamIgnorance::ReplacedB(x)) => x.to_token_stream(),
        }
    }
}

impl<'a> GenParamIgnorance<'a> {
    pub fn reborrow(&self) -> GenParamIgnorance<'_> {
        match self {
            GenParamIgnorance::Included => GenParamIgnorance::Included,
            GenParamIgnorance::Ignored => GenParamIgnorance::Ignored,
            GenParamIgnorance::Replaced(x) => GenParamIgnorance::ReplacedB(x),
            GenParamIgnorance::ReplacedB(x) => GenParamIgnorance::ReplacedB(*x),
        }
    }
}
