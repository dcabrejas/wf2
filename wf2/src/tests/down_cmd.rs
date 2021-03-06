#[cfg(test)]
mod tests {
    use crate::cli_input::CLIInput;
    use crate::cli_output::CLIOutput;
    use crate::tests::_commands;

    #[test]
    fn test_down_01() {
        let args = vec!["prog", "--cwd", "/users/shane", "down"];
        let expected = "docker-compose -f /users/shane/.wf2_m2_shane/docker-compose.yml down";
        let cli_output = CLIOutput::from_input(CLIInput::_from_args(args));
        assert_eq!(
            vec![expected],
            _commands(cli_output.expect("test").tasks.unwrap())
        );
    }
}
