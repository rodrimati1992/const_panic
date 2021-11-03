use crate::datastructure::{DataStructure, DataVariant, StructKind};

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::quote;

use syn::{
    punctuated::Punctuated,
    DeriveInput, GenericParam, Ident,
};

use alloc::{
    vec::Vec,
    string::ToString,
};


mod attribute_parsing;

pub(crate) fn derive_constdebug_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let ds = &DataStructure::new(&input);
    let config = attribute_parsing::parse_attributes(ds)?;
    let crate_path = &config.crate_path;

    let name = ds.name;

    if ds.generics.params.iter().any(|g| matches!(g, GenericParam::Type{..})) {
        return Err(syn::Error::new(
            Span::call_site(),
            "Cannot derive PanicFmt for types with type or const parameters"
        ));
    }

    let (impl_generics, ty_generics, where_clause) = ds.generics.split_for_impl();
    let preds = Punctuated::new(); 
    let predsa = where_clause.map_or(&preds, |x| &x.predicates).into_iter();
    let predsb = predsa.clone();

    let delimiters = ds.variants.iter()
        .map(|v|{
            match v.kind {
                StructKind::Tupled => quote!(__cp_bCj7dq3Pud::TypeDelim::Tupled),
                StructKind::Braced => quote!(__cp_bCj7dq3Pud::TypeDelim::Braced),
            }
        })
        .collect::<Vec<TokenStream2>>();


    let mut field_counters = ds.variants.iter()
        .enumerate()
        .map(|(v_index, v)| {
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
    if ds.data_variant == DataVariant::Struct {
        pv_count_init = field_counters.next().unwrap();
        match_prefix = quote!();
    } else {
        pv_count_init = quote!(
            __cp_bCj7dq3Pud::utils::slice_max_usize(&[
                #(
                    #field_counters
                ),*
            ])
        );
        match_prefix = quote!(Self::);
    };

    let get_pv_count = {
        let under_lt = quote!('_);
        let zero = quote!(0);
        let lts = ds.generics.lifetimes().map(|_| &under_lt);
        let const_params = ds.generics.const_params().map(|_| &zero);

        quote!(
            <#name<#(#lts,)* #(#const_params,)*> as __cp_bCj7dq3Pud::PanicFmt>::PV_COUNT
        )
    };

    let comma_sep = Ident::new("COMMA_SEP", Span::call_site());
    let comma_term = Ident::new("COMMA_TERM", Span::call_site());

    let branches = ds.variants.iter()
        .enumerate()
        .map(|(v_index, v)| {
            let vname = v.name;
            let vsname = vname.to_string();

            if v.fields.is_empty() {
                quote!(
                    #match_prefix #vname {} => {
                        __cp_bCj7dq3Pud::__::flatten_panicvals::<{#get_pv_count}>(&[&[
                            __cp_bCj7dq3Pud::PanicVal::write_str(#vsname)
                        ]])
                    }
                )
            } else {
                let last_field_pos = v.fields.len().saturating_sub(1);
                let field_names = v.fields.iter().map(|f| &f.ident);
                let field_patia = v.fields.iter().map(|f| &f.pattern_ident);
                let delimiter = &delimiters[v_index];

                let field_fmt = v.fields.iter()
                    .map(|f| {
                        let field_ty = f.ty;
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
                            &<#field_ty as __cp_bCj7dq3Pud::PanicFmt>::PROOF
                                .coerce(#field_patib)
                                .to_panicvals(fmtarg),
                            &__cp_bCj7dq3Pud::fmt::#comma
                                .to_panicvals(fmtarg),
                        )
                    });

                quote!(
                    #match_prefix #vname { #(#field_names: #field_patia,)* } => {
                        let (open, close) = #delimiter.get_open_and_close();

                        __cp_bCj7dq3Pud::__::flatten_panicvals::<{#get_pv_count}>(&[
                            &[
                                __cp_bCj7dq3Pud::PanicVal::write_str(#vsname),
                                open.to_panicval(fmtarg)
                            ],
                            #( #field_fmt )*
                            &close.to_panicvals(fmtarg.unindent()),
                        ])
                    }
                )
            }

        });

    let ret = quote!{
        const _: () = {
            use #crate_path as __cp_bCj7dq3Pud;

            impl #impl_generics __cp_bCj7dq3Pud::PanicFmt for #name #ty_generics
            where
                #( #predsa, )*
            {
                type This = Self;
                type Kind = __cp_bCj7dq3Pud::IsCustomType;

                const PV_COUNT: __cp_bCj7dq3Pud::__::usize = #pv_count_init;
            }


            impl #impl_generics #name #ty_generics
            where
                #( #predsb, )*
            {
                pub const fn to_panicvals(
                    &self,
                    mut fmtarg: __cp_bCj7dq3Pud::FmtArg,
                ) -> [__cp_bCj7dq3Pud::PanicVal<'_>; #get_pv_count] {
                    fmtarg = fmtarg.indent();

                    match self {
                        #(#branches)*
                    }
                }
            }

        };
    };

    if config.debug_print {
        panic!("\n\n\n{}\n\n\n", ret);
    }
    Ok(ret)
}



