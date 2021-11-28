//
// test/cli.rs
//

extern crate assert_cli;
extern crate serial_test;

#[cfg(test)]
mod integration {
    use assert_cli;
    use serial_test::serial;

    #[test]
    #[serial]
    fn missing_argument() {
        assert_cli::Assert::main_binary()
            .fails()
            .and()
            .stderr()
            .contains("Missing argument(s)")
            .unwrap();
    } // missing_argument()

    #[test]
    #[serial]
    fn invalid_input() {
        assert_cli::Assert::main_binary()
            .with_args(&["1, 2, 3, 4, 5", "8"])
            .fails()
            .and()
            .stderr()
            .contains("parse error")
            .unwrap();
    } // invalid_input()

    #[test]
    #[serial]
    fn valid_input() {
        assert_cli::Assert::main_binary()
            .with_args(&["1 2 3 4 5", "8"])
            .succeeds()
            .and()
            .stdout()
            .contains("[2, 4]")
            .unwrap();
    } // valid_input()
} // mod integration
