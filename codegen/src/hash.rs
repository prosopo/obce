use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Error,
    Ident,
};

use crate::utils::into_u32;

pub fn generate(input: TokenStream) -> Result<TokenStream, Error> {
    let method_name: Ident = syn::parse2(input)?;

    let hash = into_u32(method_name);

    Ok(quote! {
        #hash
    })
}
