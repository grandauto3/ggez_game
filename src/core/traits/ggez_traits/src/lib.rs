use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use syn::{DeriveInput, parse_macro_input};
use quote::quote;


#[proc_macro_derive(AutoRegister)]
pub fn auto_register(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = ast.ident;
    TokenStream::from(quote! { impl #struct_name {

    }
    })
}

fn impl_auto_register() -> TokenStream2 {
    quote! {}
}