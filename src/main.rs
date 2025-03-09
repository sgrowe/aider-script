use std::fs;
use std::process::Command;

fn main() {
    let script_file_path = "ai/example.md";

    let message = fs::read_to_string(script_file_path).expect("Failed to read the file");

    let mut cmd = create_aider_command(message);
    let status = cmd.status().expect("Failed to execute aider command");
}

fn create_aider_command(message: String) -> Command {
    let mut cmd = Command::new("aider");

    cmd.arg("-m").arg(message);

    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aider_command_basic() {
        let message = String::from("Test message");
        let cmd = create_aider_command(message);
        
        // Check that the program is "aider"
        assert_eq!(cmd.get_program(), "aider");
    }

    #[test]
    fn test_create_aider_command_args() {
        let message = String::from("Test message");
        let cmd = create_aider_command(message.clone());
        
        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();
        
        // Check that we have the -m flag followed by the message
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-m");
        assert_eq!(args[1], message);
    }
}
