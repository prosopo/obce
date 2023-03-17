use darling::FromMeta;
use itertools::Itertools;
use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::quote;
use syn::{
    parse2,
    Error,
    Expr,
    ItemEnum,
};

use crate::{
    format_err_spanned,
    types::AttributeArgs,
    utils::AttributeParser,
};

fn default_require_ret_val() -> bool {
    true
}

#[derive(FromMeta)]
struct ErrorAttrs {
    #[darling(default = "default_require_ret_val")]
    require_ret_val: bool,
}

#[derive(FromMeta)]
struct ErrorVariantAttrs {
    critical: Option<()>,
    ret_val: Option<Expr>,
}

struct RetValInfo<'a> {
    variant_name: &'a Ident,
    ret_val: Expr,
}

pub fn generate(attrs: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    let mut enum_item: ItemEnum = parse2(input)?;
    let ident = enum_item.ident.clone();
    let enum_attrs = ErrorAttrs::from_list(&syn::parse2::<AttributeArgs>(attrs)?)?;

    let (impl_generics, ty_generics, where_clause) = enum_item.generics.split_for_impl();

    let mut critical_variant = None;

    let mut ret_val_variants = vec![];

    for variant in enum_item.variants.iter_mut() {
        let variant_name = &variant.ident;

        let (obce_attrs, mut other_attrs) = variant.attrs.iter().cloned().split_attrs()?;

        let variant_attrs = ErrorVariantAttrs::from_list(&obce_attrs)?;

        if variant_attrs.critical.is_some() {
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

        if let Some(ret_val) = variant_attrs.ret_val {
            ret_val_variants.push(RetValInfo { variant_name, ret_val });
        } else if enum_attrs.require_ret_val && !ret_val_variants.is_empty() {
            return Err(format_err_spanned!(
                variant,
                "you have to mark this variant with `ret_val` or set `require_ret_val` to `false`"
            ))
        }

        variant.attrs = other_attrs;
    }

    if let Some(expr) = ret_val_variants.iter().map(|info| &info.ret_val).duplicates().next() {
        return Err(format_err_spanned!(expr, "ret_val value is used twice"))
    }

    let formatted_ret_val = ret_val_variants.iter().map(|RetValInfo { variant_name, ret_val }| {
        quote! {
            #ident::#variant_name => Ok(Self::Converging(#ret_val)),
        }
    });

    let ret_val_impl = quote! {
        impl #impl_generics ::core::convert::TryFrom<#ident #ty_generics>
            for ::obce::substrate::pallet_contracts::chain_extension::RetVal
            #where_clause
        {
            type Error = #ident #ty_generics;

            fn try_from(value: #ident #ty_generics) -> Result<Self, #ident #ty_generics> {
                match value {
                    #(#formatted_ret_val)*
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
