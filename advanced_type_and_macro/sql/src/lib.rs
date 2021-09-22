extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use syn;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let s_token = input.to_string();
    let dialect = MySqlDialect {}; // or AnsiDialect, or your own dialect ...

    // check sql syntax
    let _ = Parser::parse_sql(&dialect, &s_token).unwrap();
    let q = quote! {
        stringify!(#s_token)
    };
    q.into()
}