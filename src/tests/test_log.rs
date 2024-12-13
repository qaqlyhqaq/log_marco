#[cfg(test)]
mod test {

    #[test]
    fn it_works() -> Result<(), &'static str> {
        f(1usize);
        Ok(())
    }

    #[log_macro::log_handler]
    fn f(i:usize) {}
}
