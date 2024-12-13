extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput};
use quote::quote;



#[proc_macro_derive(MyDerive)]
pub fn my_derive(ast: &DeriveInput) -> TokenStream {



    let tokens = quote! {
        struct Hello;
    };

    tokens.into()
}

