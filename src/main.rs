use clap::Parser;
use clio::Input;
use std::io::Read;
use std::process::Command;

use command_template::CommandTemplate;

mod aider_command;
mod command_template;
mod markdown_doc;
mod str;

#[derive(Parser, Debug)]
#[command(name = "aider-script")]
#[command(version, about, long_about = None)]
struct Args {
    /// Template file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    template: Input,
}

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();

    let mut message = String::new();

    args.template.read_to_string(&mut message)?;

    let mut cmd = create_aider_command(&message);
    cmd.status()?;

    Ok(())
}

fn create_aider_command(markdown: &str) -> Command {
    let cmd_template = CommandTemplate::parse(markdown);

    dbg!(&cmd_template);

    // Get expected CLI args from doc

    // extract arg values from env

    // substitute values into message template

    // TODO:
    // document.to_aider_command()

    // Add the body as the main message
    // cmd.arg("-m").arg(document.body.trim());

    Command::new("aider")
}
