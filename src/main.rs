use std::process::Command;
use std::fs;

fn main() {
    let script_file_path = "ai/example.md";

    let message = fs::read_to_string(script_file_path)
        .expect("Failed to read the file");

    let status = Command::new("aider")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to execute aider command");
}
