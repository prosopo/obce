// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::collections::HashMap;

use crate::{
    format_err_spanned,
    utils::{
        input_bindings,
        input_bindings_tuple,
        into_u32,
        AttributeParser,
    },
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use syn::{
    parse::Parser,
    parse2,
    parse_str,
    punctuated::Punctuated,
    Error,
    Expr,
    GenericArgument,
    Generics,
    Ident,
    ImplItem,
    ItemImpl,
    Lit,
    Meta,
    NestedMeta,
    Path,
    PathArguments,
    Token,
    Type,
};
use tuple::Map;

pub struct ChainExtensionImplementation;

impl ChainExtensionImplementation {
    pub fn generate(_attrs: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
        let impl_item: ItemImpl = parse2(input).unwrap();

        let mut original_implementation = impl_item.clone();

        let method_items = original_implementation.items.iter_mut().filter_map(|item| {
            if let ImplItem::Method(method_item) = item {
                Some(method_item)
            } else {
                None
            }
        });

        for method_item in method_items {
            let (_, other_attrs) = method_item.attrs.iter().cloned().split_attrs()?;

            method_item.attrs = other_attrs;
        }

        let chain_extension = Self::chain_extension_trait_impl(impl_item)?;

        Ok(quote! {
            // Implementation of the trait for `ExtensionContext` with main logic.
            #original_implementation

            // Implementation of `ChainExtension` from `contract-pallet`
            #chain_extension
        })
    }

    #[allow(non_snake_case)]
    fn chain_extension_trait_impl(mut impl_item: ItemImpl) -> Result<TokenStream, Error> {
        let context = Self::split_generics(&impl_item)?;
        let mut main_generics = impl_item.generics.clone();
        main_generics = filter_generics(main_generics, &context.lifetime1);
        main_generics = filter_generics(main_generics, &context.lifetime2);
        main_generics = filter_generics(main_generics, &context.env);
        let (main_impls, _, main_where) = main_generics.split_for_impl();

        let mut call_generics = impl_item.generics.clone();
        call_generics = filter_generics(call_generics, &context.lifetime1);
        call_generics = filter_generics(call_generics, &context.lifetime2);
        let (_, _, call_where) = call_generics.split_for_impl();

        let T = context.substrate;
        let E = context.env;
        let extension = context.extension;
        let namespace = quote! { ::obce::substrate::pallet_contracts::chain_extension:: };
        let trait_;
        let dyn_trait;
        if let Some((_, path, _)) = impl_item.trait_ {
            trait_ = path.clone();
            dyn_trait = quote! { dyn #path };
        } else {
            return Err(format_err_spanned!(impl_item, "expected impl trait block",))
        }

        let methods: Vec<_> = impl_item
            .items
            .iter_mut()
            .filter_map(|item| {
                if let ImplItem::Method(method) = item {
                    Some(method)
                } else {
                    None
                }
            })
            .map(|method| {
                let (obce_attrs, other_attrs) = method.attrs.iter().cloned().split_attrs()?;

                method.attrs = other_attrs;

                let hash = into_u32(&method.sig.ident);
                let method_name = &method.sig.ident;
                let input_bindings = input_bindings(&method.sig.inputs);
                let bindings_tuple = input_bindings_tuple(input_bindings.iter());
                let weight_tokens = handle_weight_attribute(&input_bindings, obce_attrs.iter())?;

                Result::<_, Error>::Ok(quote! {
                    <#dyn_trait as ::obce::codegen::MethodDescription<#hash>>::ID => {
                        let #bindings_tuple = env.read_as_unbounded(len)?;
                        #weight_tokens
                        let mut context = ::obce::substrate::ExtensionContext::new(self, env);
                        let result = <_ as #trait_>::#method_name(
                            &mut context
                            #(
                                , #input_bindings
                            )*
                        );
                        // If result is `Result` and `Err` is critical, return from the `call`.
                        // Otherwise encode the result into the buffer.
                        let result = ::obce::to_critical_error!(result)?;
                        <_ as ::scale::Encode>::using_encoded(&result, |w| context.env.write(w, true, None))?;
                    },
                })
            })
            .try_collect()?;

        Ok(quote! {
            impl #main_impls #namespace ChainExtension<#T> for #extension #main_where {
                fn call<#E>(&mut self, env: #namespace Environment<#E, #namespace InitState>)
                    -> ::core::result::Result<#namespace RetVal, ::obce::substrate::sp_runtime::DispatchError>
                    #call_where
                {
                    let mut env = env.buf_in_buf_out();
                    let len = env.in_len();

                    match env.func_id() {
                        #(#methods)*
                        _ => ::core::result::Result::Err(::obce::substrate::sp_runtime::DispatchError::Other(
                            "InvalidFunctionId"
                        ))?,
                    };

                    Ok(#namespace RetVal::Converging(0))
                }
            }

            impl #main_impls #namespace RegisteredChainExtension<#T> for #extension #main_where {
                const ID: ::core::primitive::u16 = <#dyn_trait as ::obce::codegen::ExtensionDescription>::ID;
            }
        })
    }

    fn split_generics(impl_item: &ItemImpl) -> Result<ExtensionContext, Error> {
        let lifetime1;
        let lifetime2;
        let env_generic;
        let substrate;
        let extension_ty;

        let wrong_type = Err(format_err_spanned!(
            impl_item.self_ty,
            "the type should be `ExtensionContext`",
        ));
        if let Type::Path(path) = impl_item.self_ty.as_ref() {
            if let Some(extension) = path.path.segments.last() {
                if let PathArguments::AngleBracketed(generic_args) = &extension.arguments {
                    if generic_args.args.len() == 5 {
                        lifetime1 = generic_args.args[0].clone();
                        lifetime2 = generic_args.args[1].clone();
                        env_generic = generic_args.args[2].clone();
                        substrate = generic_args.args[3].clone();
                        extension_ty = generic_args.args[4].clone();
                    } else {
                        return Err(format_err_spanned!(
                            extension.arguments,
                            "`ExtensionContext` should have 5 generics as `<'a, 'b, E, T, Extension>`",
                        ))
                    }
                } else {
                    return Err(format_err_spanned!(
                        extension.arguments,
                        "`ExtensionContext` should have `<'a, 'b, E, T, Extension>`",
                    ))
                }
            } else {
                return wrong_type
            }
        } else {
            return wrong_type
        }

        Ok(ExtensionContext {
            lifetime1,
            lifetime2,
            substrate,
            env: env_generic,
            extension: extension_ty,
        })
    }
}

struct ExtensionContext {
    // Lifetime `'a`
    lifetime1: GenericArgument,
    // Lifetime `'b`
    lifetime2: GenericArgument,
    // Generic `E`
    env: GenericArgument,
    // Generic `T`
    substrate: GenericArgument,
    // Generic `Extension`
    extension: GenericArgument,
}

fn filter_generics(mut generics: Generics, filter: &GenericArgument) -> Generics {
    let filter: Vec<_> = filter
        .to_token_stream()
        .into_iter()
        .map(|token| token.to_string())
        .collect();
    generics.params = generics
        .params
        .clone()
        .into_iter()
        .filter(|param| {
            let param: Vec<_> = param
                .to_token_stream()
                .into_iter()
                .map(|token| token.to_string())
                .collect();
            !is_subsequence(&param, &filter)
        })
        .collect();

    if let Some(where_clause) = &mut generics.where_clause {
        where_clause.predicates = where_clause
            .predicates
            .clone()
            .into_iter()
            .filter(|predicate| {
                let predicate: Vec<_> = predicate
                    .to_token_stream()
                    .into_iter()
                    .map(|token| token.to_string())
                    .collect();
                !is_subsequence(&predicate, &filter)
            })
            .collect();
    }

    generics
}

fn is_subsequence<T: PartialEq + core::fmt::Debug>(src: &[T], search: &[T]) -> bool {
    if search.len() > src.len() {
        return false
    }

    for i in 0..(src.len() - search.len() + 1) {
        if &src[i..(i + search.len())] == search {
            return true
        }
    }
    false
}

fn handle_weight_attribute<'a, I: IntoIterator<Item = &'a NestedMeta>>(
    input_bindings: &[Ident],
    iter: I,
) -> Result<Option<TokenStream>, Error> {
    let weight_params = iter.into_iter().find_map(|attr| {
        let NestedMeta::Meta(Meta::List(list)) = attr else {
            return None;
        };

        if !list.path.is_ident("weight") {
            return None
        }

        let params = list
            .nested
            .iter()
            .filter_map(|param| {
                match param {
                    NestedMeta::Meta(Meta::NameValue(value)) => {
                        Some((
                            value.path.get_ident()?.to_string(),
                            match &value.lit {
                                Lit::Str(st) => st.value(),
                                _ => return None,
                            },
                        ))
                    }
                    _ => None,
                }
            })
            .collect::<HashMap<_, _>>();

        Some((params, attr))
    });

    Ok(if let Some((weight_params, attr)) = weight_params {
        let dispatch_path = weight_params
            .get("dispatch")
            .ok_or_else(|| format_err_spanned!(attr, "unable to find \"dispatch\" attribute"))?;

        let segments = parse_str::<Path>(dispatch_path)?.segments.into_iter();
        let segments_len = segments.len();

        if segments_len < 3 {
            return Err(format_err_spanned!(
                attr,
                "dispatch path should contain at least three segments"
            ))
        }

        let (pallet_ns, _, method_name) = segments
            .enumerate()
            .group_by(|(idx, _)| if *idx < segments_len - 2 { 0 } else { *idx })
            .into_iter()
            .map(|(_, group)| group.map(|(_, segment)| segment))
            .next_tuple::<(_, _, _)>()
            .unwrap()
            .map(Punctuated::<_, Token![::]>::from_iter);

        let dispatch_args = if let Some(args) = weight_params.get("args") {
            let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
            parser.parse_str(args)?.to_token_stream()
        } else {
            quote! {
                #(#input_bindings,)*
            }
        };

        let call_variant_name = format_ident!("new_call_variant_{}", method_name.last().unwrap().ident);

        Some(quote! {
            let __call_variant = &#pallet_ns ::Call::<T>::#call_variant_name(#dispatch_args);
            let __dispatch_info = <#pallet_ns ::Call<T> as ::obce::substrate::frame_support::dispatch::GetDispatchInfo>::get_dispatch_info(__call_variant);
            env.charge_weight(__dispatch_info.weight)?;
        })
    } else {
        None
    })
}
