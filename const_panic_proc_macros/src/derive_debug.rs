use crate::{
    datastructure::{DataStructure, DataVariant, GenParamKind, StructKind},
    syntax::ImplHeader,
};

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::quote;

use syn::{punctuated::Punctuated, DeriveInput, Ident};

use alloc::{string::ToString, vec::Vec};

use self::attribute_parsing::{Configuration, GenParamIgnorance};

mod attribute_parsing;

#[cfg(test)]
mod tests;

pub(crate) fn derive_constdebug_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let ds = &DataStructure::new(&input);
    let config = attribute_parsing::parse_attributes(ds)?;
    let crate_path = &config.crate_path;

    let name = ds.name;

    if config.impls.is_empty() {
        let not_ignored = config
            .gen_params_props
            .iter()
            .zip(ds.generics.type_params())
            .filter(|(x, _)| matches!(x.ignored, GenParamIgnorance::Included))
            .map(|(_, tp)| &tp.ident)
            .collect::<Vec<_>>();

        if !not_ignored.is_empty() {
            let not_ignored = not_ignored.into_iter();
            let msg = alloc::format!(
                concat!(
                    "these type parameters were not ignored or replaced with concrete types:\n",
                    "    {0}\n",
                    "You must use either or both of these attributes:\n",
                    "- `#[pfmt(ignore({0}))]`:",
                    "if the type parameters are only used in marker types (eg: `PhantomData`).\n",
                    "- `#[pfmt(impl ...)]`:",
                    "To implement panic formatting with concrete types for those type parameters",
                    "(this attribute can be used multiple times to add impls).\n",
                ),
                quote!(#(#not_ignored),*)
            );
            return Err(syn::Error::new(Span::call_site(), msg));
        }
    }

    let (impl_generics, ty_generics, where_clause) = ds.generics.split_for_impl();
    let preds = Punctuated::new();
    let preds = where_clause.map_or(&preds, |x| &x.predicates).into_iter();
    let preds = quote!(#( #preds, )*);

    let delimiters = ds
        .variants
        .iter()
        .map(|v| match v.kind {
            StructKind::Tupled => quote!(__cp_bCj7dq3Pud::TypeDelim::Tupled),
            StructKind::Braced => quote!(__cp_bCj7dq3Pud::TypeDelim::Braced),
        })
        .collect::<Vec<TokenStream2>>();

    let mut field_counters = ds.variants.iter().enumerate().map(|(v_index, v)| {
        let field_amount = v.fields.len();
        let field_tys = v.fields.iter().map(|x| x.ty);
        let delimiter = &delimiters[v_index];

        quote!(
            __cp_bCj7dq3Pud::ComputePvCount {
                field_amount: #field_amount,
                summed_pv_count: {
                    0
                    #( + <#field_tys as __cp_bCj7dq3Pud::PanicFmt>::PV_COUNT )*
                },
                delimiter: #delimiter,
            }.call()
        )
    });

    let pv_count_init;
    let match_prefix;

    match ds.data_variant {
        DataVariant::Struct => {
            pv_count_init = field_counters.next().unwrap();
            match_prefix = quote!();
        }
        DataVariant::Enum => {
            pv_count_init = quote!(
                __cp_bCj7dq3Pud::utils::slice_max_usize(&[
                    #(
                        #field_counters
                    ),*
                ])
            );
            match_prefix = quote!(Self::);
        }
        DataVariant::Union => {
            return Err(syn::Error::new(
                Span::call_site(),
                "unions are not supported",
            ))
        }
    }

    let args_for_inherent_impl = ArgsForInherentImpl {
        comma_sep: Ident::new("COMMA_SEP", Span::call_site()),
        comma_term: Ident::new("COMMA_TERM", Span::call_site()),
        ds,
        delimiters: &delimiters,
        match_prefix,
    };

    let single_impl_ihapvcs: ImplHeaderAndPvCountSelf;
    let vec_impl_ihapvcs: Vec<ImplHeaderAndPvCountSelf>;
    let impl_ihapvcs: &[ImplHeaderAndPvCountSelf];

    if config.impls.is_empty() {
        let replaced_args = ds
            .generics
            .params
            .iter()
            .zip(&config.gen_params_props)
            .map(|(gp, gpp)| gpp.tokenize_arg(gp));

        single_impl_ihapvcs = ImplHeaderAndPvCountSelf {
            impl_header: quote! {
                impl #impl_generics #name #ty_generics
                where
                    #preds
            },
            pvcount_self: quote!(#name<#(#replaced_args),*>),
        };

        impl_ihapvcs = core::slice::from_ref(&single_impl_ihapvcs);
    } else {
        vec_impl_ihapvcs = config
            .impls
            .iter()
            .map(
                |ImplHeader {
                     generics,
                     self_args,
                 }| {
                    let (impl_generics, _, where_clause) = generics.split_for_impl();

                    let additional_preds = Punctuated::new();
                    let additional_preds = where_clause
                        .map_or(&additional_preds, |x| &x.predicates)
                        .into_iter();

                    let impl_header = quote! {
                        impl #impl_generics #name #self_args
                        where
                            #( #additional_preds, )*
                            #preds
                    };

                    let replaced_args = self_args
                        .args
                        .iter()
                        .zip(&config.gen_params_props)
                        .map(|(ga, gpp)| gpp.tokenize_arg(ga));

                    let pvcount_self = quote!(#name<#(#replaced_args),*>);

                    ImplHeaderAndPvCountSelf {
                        impl_header,
                        pvcount_self,
                    }
                },
            )
            .collect();
        impl_ihapvcs = &vec_impl_ihapvcs;
    }

    let impl_ihapvcs_mapped = impl_ihapvcs
        .iter()
        .map(|impl_ihapvc| emit_inherent_impl(&config, impl_ihapvc, &args_for_inherent_impl));

    let ty_params = ds
        .generics
        .type_params()
        .zip(
            config
                .gen_params_props
                .iter()
                .filter(|x| matches!(x.kind, GenParamKind::Type)),
        )
        .filter(|(_, prop)| matches!(prop.ignored, GenParamIgnorance::Included))
        .map(|(t, _)| &t.ident);

    let ret = quote! {
        use #crate_path as __cp_bCj7dq3Pud;

        impl #impl_generics __cp_bCj7dq3Pud::PanicFmt for #name #ty_generics
        where
            #preds
            #(#ty_params: __cp_bCj7dq3Pud::PanicFmt,)*
        {
            type This = Self;
            type Kind = __cp_bCj7dq3Pud::IsCustomType;

            const PV_COUNT: __cp_bCj7dq3Pud::__::usize = #pv_count_init;
        }

        #(#impl_ihapvcs_mapped)*
    };

    let ret = quote!(
        const _: () = {
            #ret
        };
    );

    if config.debug_print {
        panic!("\n\n\n{}\n\n\n", ret);
    }
    Ok(ret)
}

struct ImplHeaderAndPvCountSelf {
    impl_header: TokenStream2,
    // The Self type with generic arguments replaced so that they can be used
    // to get the PV_COUNT associated constant
    pvcount_self: TokenStream2,
}

struct ArgsForInherentImpl<'a> {
    comma_sep: Ident,
    comma_term: Ident,
    ds: &'a DataStructure<'a>,
    delimiters: &'a [TokenStream2],
    match_prefix: TokenStream2,
}

fn emit_inherent_impl(
    Configuration { display_fmt, .. }: &Configuration<'_>,
    ImplHeaderAndPvCountSelf {
        impl_header,
        pvcount_self,
    }: &ImplHeaderAndPvCountSelf,
    ArgsForInherentImpl {
        comma_sep,
        comma_term,
        ds,
        delimiters,
        match_prefix,
    }: &ArgsForInherentImpl<'_>,
) -> TokenStream2 {
    let get_pv_count = quote!(<#pvcount_self as __cp_bCj7dq3Pud::PanicFmt>::PV_COUNT);

    let branches = ds.variants.iter().enumerate().map(|(v_index, v)| {
        let vname = v.name;
        let vsname = vname.to_string();

        let last_field_pos = v.fields.len().saturating_sub(1);
        let field_names = v.fields.iter().map(|f| &f.ident);
        let field_patia = v.fields.iter().map(|f| &f.pattern_ident);
        let delimiter = &delimiters[v_index];

        let field_fmt = v.fields.iter().map(|f| {
            let field_patib = &f.pattern_ident;

            let comma = if f.index.pos == last_field_pos {
                &comma_term
            } else {
                &comma_sep
            };

            let field_name_colon = if let StructKind::Braced = v.kind {
                let fname = ::alloc::format!("{}: ", f.ident);

                quote!(
                    &[__cp_bCj7dq3Pud::PanicVal::write_str(#fname)],
                )
            } else {
                TokenStream2::new()
            };

            quote!(
                #field_name_colon
                &__cp_bCj7dq3Pud::PanicFmt::PROOF
                    .infer(#field_patib)
                    .coerce(#field_patib)
                    .to_panicvals(fmtarg),
                &__cp_bCj7dq3Pud::fmt::#comma
                    .to_panicvals(fmtarg),
            )
        });

        if v.fields.is_empty() {
            quote!(
                #match_prefix #vname { #(#field_names: #field_patia,)* } => {
                    __cp_bCj7dq3Pud::__::flatten_panicvals::<{#get_pv_count}>(&[&[
                        __cp_bCj7dq3Pud::PanicVal::write_str(#vsname)
                    ]])
                }
            )
        } else {
            quote!(#match_prefix #vname { #(#field_names: #field_patia,)* } => {
                let (open, close) = #delimiter.get_open_and_close();

                __cp_bCj7dq3Pud::__::flatten_panicvals::<{#get_pv_count}>(&[
                    &[
                        __cp_bCj7dq3Pud::PanicVal::write_str(#vsname),
                        open.to_panicval(fmtarg)
                    ],
                    #( #field_fmt )*
                    &close.to_panicvals(fmtarg.unindent()),
                ])
            })
        }
    });

    let ondebug = quote! (
        fmtarg = fmtarg.indent();

        match self {
            #(#branches)*
        }
    );

    let dofmt = match display_fmt {
        Some(display_fmt_) => quote!(
            match fmtarg.fmt_kind {
                __cp_bCj7dq3Pud::fmt::FmtKind::Debug => { #ondebug }
                _ => (#display_fmt_)(self, fmtarg),
            }
        ),
        None => ondebug,
    };

    quote!(
        #impl_header
        {
            pub const fn to_panicvals(
                &self,
                mut fmtarg: __cp_bCj7dq3Pud::FmtArg,
            ) -> [__cp_bCj7dq3Pud::PanicVal<'_>; #get_pv_count] {
                #dofmt
            }
        }
    )
}
