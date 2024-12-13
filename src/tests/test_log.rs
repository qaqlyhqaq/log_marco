#[cfg(test)]
mod macro_test {

    #[test]
    fn main_fn_name()   {
        main();
    }

    #[log_macro::log_handler]
    fn main() {}
}
