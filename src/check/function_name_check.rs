use syn::{ItemFn, parse_macro_input};

//func name check ,
//if not main ,then panic once error message !
#[inline]
pub(crate) fn check( item: proc_macro::TokenStream)-> proc_macro::TokenStream{

    let stream_ = item.clone();

    let input_fn = parse_macro_input!(stream_ as ItemFn);

    if input_fn.sig.ident != "main"{
        panic!("main: expected function name but got {}", input_fn.sig.ident);
    }

    item

}