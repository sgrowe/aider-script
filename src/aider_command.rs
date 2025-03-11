use std::process::Command;

#[derive(Debug)]
pub struct AiderCommand {
    pub message: String,
    /// File paths which will be passed as read-only to aider (using `--read`)
    pub read_only: Vec<String>,
    /// File paths to be edited by aider (passed using `--file`)
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

        // Add read-only files
        for file in &self.read_only {
            cmd.arg("--read").arg(file);
        }

        // Add files to edit
        for file in &self.edit {
            cmd.arg("--file").arg(file);
        }

        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_command() {
        let message = "Test message";
        let mut command = AiderCommand::message(message);
        command.read_only = vec!["file1.rs".to_string(), "file2.rs".to_string()];
        command.edit = vec!["file3.rs".to_string()];

        let cmd = command.to_shell_command();

        // Check that the program is "aider"
        assert_eq!(cmd.get_program(), "aider");

        // Check that the args include message, read-only files, and edit files
        let args: Vec<_> = cmd.get_args().collect();
        assert_eq!(args.len(), 8);
        assert_eq!(args[0], "-m");
        assert_eq!(args[1], "Test message");
        assert_eq!(args[2], "--read");
        assert_eq!(args[3], "file1.rs");
        assert_eq!(args[4], "--read");
        assert_eq!(args[5], "file2.rs");
        assert_eq!(args[6], "--file");
        assert_eq!(args[7], "file3.rs");
    }
}
