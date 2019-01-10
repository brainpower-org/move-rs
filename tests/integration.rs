extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;


    //#[test]
    fn with_hello() {
        assert_cli::Assert::main_binary()
            .with_args(&["-p"])
            .stdout()
            .contains("Hello, world")
            .unwrap();
    }
}