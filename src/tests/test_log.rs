#[cfg(test)]
mod macro_test {

    #[test]
    fn it_works() -> Result<(), &'static str> {
        f();
        Ok(())
    }

    #[log_macro::log_handler]
    fn f() {}
}
