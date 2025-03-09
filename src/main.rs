use std::fs;
use std::process::Command;

fn main() {
    let script_file_path = "ai/example.md";

    let message = fs::read_to_string(script_file_path).expect("Failed to read the file");

    // Refactor this into a function that takes `message` as an argument, and returns a `Command`, but does not execute it AI!
    let status = Command::new("aider")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to execute aider command");
}
