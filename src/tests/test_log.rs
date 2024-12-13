#[cfg(test)]
mod macro_test {

    #[test]
    fn main_fn_name()   {
        main();
    }

    #[test]
    fn not_main_fn_name()   {
        main();
    }

    #[log_macro::log_handler(allow_not_main)]
    fn main() {}
}
