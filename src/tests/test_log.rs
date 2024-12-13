#[cfg(test)]
mod test {

    #[test]
    fn it_works() -> Result<(), &'static str> {
        f();
        Ok(())
    }

    #[log_macro::log_handler(fu)]
    fn f() {}
}
