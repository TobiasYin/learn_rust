extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

fn impl_route(ast: DeriveInput) -> TokenStream {
    TokenStream::from(ast.to_token_stream())
}