use std::fmt::Debug;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn impl_serialize(tokens: TokenStream) -> TokenStream {
    // let DeriveInput { ident, .. } = parse_macro_input!(tokens);
    let output = format!("{} #[derive(Debug)]", tokens);
    // let output = quote! {
    //   impl serde::Serialize for #ident {
    //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //     where
    //         S: serde::Serializer,
    //     {
    //         todo!()
    //     }
    //   }
    // };

    output.parse().unwrap()
}
