use std::fs;
use std::process::Command;

use command_template::CommandTemplate;
use markdown_doc::MarkdownDoc;

mod aider_command;
mod command_template;
mod markdown_doc;

fn main() {
    let script_file_path = "src/fixtures/01_args.md";

    let message = fs::read_to_string(script_file_path).expect("Failed to read the file");

    let mut cmd = create_aider_command(&message);
    cmd.status().expect("Failed to execute aider command");
}

fn create_aider_command(markdown: &str) -> Command {
    let cmd_template = CommandTemplate::parse(markdown);

    dbg!(&cmd_template);

    // Get expected CLI args from doc

    // extract arg values from env

    // substitute values into message template

    // TODO:
    // document.to_aider_command()

    let cmd = Command::new("aider");

    // Add the body as the main message
    // cmd.arg("-m").arg(document.body.trim());

    cmd
}
