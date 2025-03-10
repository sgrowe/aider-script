use std::process::Command;

#[derive(Debug)]
pub struct AiderCommand {
    pub message: String,
    /// File paths which will be passed as read-only to aider (using `--read`)
    pub read_only: Vec<String>,
    /// File paths to be edited by aider (passed using `--edit`)
    pub edit: Vec<String>,
}

impl AiderCommand {
    pub fn message<S: Into<String>>(message: S) -> Self {
        AiderCommand {
            message: message.into(),
            read_only: vec![],
            edit: vec![],
        }
    }
    pub fn to_shell_command(&self) -> Command {
        let mut cmd = Command::new("aider");

        cmd.arg("-m").arg(self.message.trim());

        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_command() {
        let message = "Test message";
        let command = AiderCommand::message(message);

        let cmd = command.to_shell_command();

        // Check that the program is "aider"
        assert_eq!(cmd.get_program(), "aider");

        // Check that the args are ["-m", "Test message"]
        let args: Vec<_> = cmd.get_args().collect();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-m");
        assert_eq!(args[1], "Test message");
    }
}
