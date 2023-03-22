use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse2,
    Error,
    ItemStruct,
};

pub fn ink(_: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    let struct_item: ItemStruct = parse2(input)?;

    let struct_name = &struct_item.ident;

    Ok(quote! {
        #struct_item

        #[cfg(feature = "ink")]
        impl ::obce::ink_lang::ChainExtensionInstance for #struct_name {
            type Instance = #struct_name;

            fn instantiate() -> Self::Instance {
                #struct_name
            }
        }
    })
}
