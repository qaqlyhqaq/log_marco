extern crate proc_macro;

use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn log_handler(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn:ItemFn = parse_macro_input!(item as ItemFn);


    let block = input_fn.block;
    let attrs = input_fn.attrs;
    let sig = input_fn.sig;
    let generate = quote! {
        #sig {
            println!("log proc_macro_attribute is call !");
            #block
            }
    };
    generate.into()
}
