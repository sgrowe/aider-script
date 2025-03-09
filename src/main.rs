use std::fs;
use std::process::Command;

fn create_aider_command(message: String) -> Command {
    let mut cmd = Command::new("aider");
    cmd.arg("-m").arg(message);
    cmd
}

fn main() {
    let script_file_path = "ai/example.md";

    let message = fs::read_to_string(script_file_path).expect("Failed to read the file");

    let mut cmd = create_aider_command(message);
    let status = cmd.status().expect("Failed to execute aider command");
}
