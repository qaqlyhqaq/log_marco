#[cfg(test)]
mod macro_test {

    //normal case , not test on default .
    #[test]
    #[ignore]
    fn main_fn()   {
        main();
    }

    #[test]
    fn not_main_fn_name()   {
        main_not_main_case();
    }




    #[log_macro::log_handler()]
    fn main() {}

    #[log_macro::log_handler(allow_not_main)]
    fn main_not_main_case() {}
}
