#[cfg(test)]
mod tests {
    use crate::cli_input::CLIInput;
    use crate::cli_output::CLIOutput;
    use crate::tests::_commands;
    use std::path::PathBuf;
    use wf2_core::task::Task;

    #[test]
    fn test_push_dir() {
        let args = vec!["prog", "push", "vendor/shane"];
        let cwd = "/users/acme";
        let expected_commands = vec![
            "docker exec wf2__acme__php rm -rf /var/www/vendor/shane",
            "docker exec -u www-data wf2__acme__php mkdir -p /var/www/vendor",
            "docker cp /users/acme/vendor/shane wf2__acme__php:/var/www/vendor",
        ];
        test_push(args, cwd, expected_commands);
    }

    #[test]
    fn test_push_single_file() {
        let args = vec!["prog", "push", "composer.json"];
        let cwd = "/users/acme";
        let expected_commands = vec![
            "docker exec wf2__acme__php rm -rf /var/www/composer.json",
            "docker cp /users/acme/composer.json wf2__acme__php:/var/www",
        ];
        test_push(args, cwd, expected_commands);
    }

    #[test]
    fn test_push_invalid_files() {
        let args = vec!["prog", "push", "app/"];
        test_push_invalid(args);
        let args = vec!["prog", "push", "app/code"];
        test_push_invalid(args);
        let args = vec!["prog", "push", "app/code/Acme/Lib/File"];
        test_push_invalid(args);
        let args = vec!["prog", "push", "vendor/magento", "app/code"];
        test_push_invalid(args);
    }

    #[test]
    fn test_push_invalid_files_with_force() {
        let args = vec!["prog", "push", "app/code/Acme/Lib/File", "-f"];
        let cwd = "/users/acme";
        let expected_commands = vec![
            "docker cp /users/acme/app/code/Acme/Lib/File wf2__acme__php:/var/www/app/code/Acme/Lib",
        ];
        test_push(args, cwd, expected_commands);
    }

    fn test_push(args: Vec<&str>, cwd: impl Into<PathBuf>, expected_commands: Vec<&str>) {
        let input = CLIInput::_from_args(args)._with_cwd(cwd);
        let cli_output = CLIOutput::from_input(input);
        let tasks = cli_output.expect("test").tasks.unwrap();
        assert_eq!(_commands(tasks), expected_commands);
    }

    fn test_push_invalid(args: Vec<&str>) {
        let cwd = "/users/acme";
        let input = CLIInput::_from_args(args)._with_cwd(cwd);
        let cli_output = CLIOutput::from_input(input);
        let tasks = cli_output.expect("test").tasks.unwrap();
        match tasks.get(0) {
            Some(Task::NotifyError { .. }) => {}
            _ => unreachable!(),
        }
    }
}
