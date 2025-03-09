use std::process::Command;

fn main() {
    let script_file_path = "ai/example.md";

    // Read in `message` from the file at `script_file_path` and pass it in below AI!

    let status = Command::new("aider")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to execute aider command");
}
