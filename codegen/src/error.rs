use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse2,
    parse_str,
    Error,
    Expr,
    ItemEnum,
    Lit,
    Meta,
    NestedMeta,
};

use crate::{
    format_err_spanned,
    utils::AttributeParser,
};

pub fn generate(_attrs: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    let mut enum_item: ItemEnum = parse2(input)?;
    let ident = enum_item.ident.clone();
    let (impl_generics, ty_generics, where_clause) = enum_item.generics.split_for_impl();

    let mut critical_variant = None;

    let mut ret_val_variants = vec![];

    for variant in enum_item.variants.iter_mut() {
        let variant_name = &variant.ident;

        let (obce_attrs, mut other_attrs) = variant.attrs.iter().cloned().split_attrs()?;

        for arg in obce_attrs {
            let critical = matches!(
                &arg,
                NestedMeta::Meta(Meta::Path(value))
                if value.is_ident("critical")
            );

            if critical {
                other_attrs.push(syn::parse_quote! {
                    #[cfg(feature = "substrate")]
                });

                let previous_critical_variant = critical_variant.replace(quote! {
                    #[cfg(feature = "substrate")]
                    impl #impl_generics ::obce::substrate::SupportCriticalError for #ident #ty_generics #where_clause {
                        fn try_to_critical(self) -> Result<::obce::substrate::CriticalError, Self> {
                            match self {
                                Self::#variant_name(error) => Ok(error),
                                _ => Err(self)
                            }
                        }
                    }
                });

                if let Some(variant) = previous_critical_variant {
                    return Err(format_err_spanned!(
                        variant,
                        "only one enum variant can be marked as `#[obce(critical)]`",
                    ))
                }
            }

            let ret_val = if let NestedMeta::Meta(Meta::NameValue(meta)) = &arg {
                if meta.path.get_ident().filter(|ident| *ident == "ret_val").is_some() {
                    match &meta.lit {
                        Lit::Str(lit_str) => Some(parse_str::<Expr>(&lit_str.value())?),
                        _ => return Err(format_err_spanned!(meta.lit, "expected string literal")),
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(ret_val) = ret_val {
                ret_val_variants.push(quote! {
                    #ident::#variant_name => Ok(Self::Converging(#ret_val)),
                });
            }
        }

        variant.attrs = other_attrs;
    }

    let ret_val_impl = quote! {
        impl #impl_generics ::core::convert::TryFrom<#ident #ty_generics>
            for ::obce::substrate::pallet_contracts::chain_extension::RetVal
            #where_clause
        {
            type Error = #ident #ty_generics;

            fn try_from(value: #ident #ty_generics) -> Result<Self, #ident #ty_generics> {
                match value {
                    #(#ret_val_variants)*
                    _ => Err(value)
                }
            }
        }
    };

    Ok(quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, ::scale::Encode, ::scale::Decode)]
        #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
        #enum_item

        #critical_variant

        #[cfg(feature = "substrate")]
        #ret_val_impl
    })
}
