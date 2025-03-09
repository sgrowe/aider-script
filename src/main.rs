use anyhow::Ok;
use args::Args;
use clap::Parser;

use command_template::CommandTemplate;

mod aider_command;
mod args;
mod command_template;
mod markdown_doc;
mod str;

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();

    let template = args.read_template()?;
    let cmd_template = CommandTemplate::parse(&template)?;

    let aider_cmd = cmd_template.apply_args(&args.template_arguments)?;

    if args.preview_message {
        println!("Generated message:");
        println!("------------------");
        println!();
        println!("{}", aider_cmd.message);

        return Ok(());
    }

    let mut cmd = aider_cmd.to_shell_command();
    cmd.status()?;

    Ok(())
}
