extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_handler(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let generate = quote! {
            //调用属性宏
            println!("log proc_macro_attribute is call !");
        #input_fn
    };
    generate.into()
}
