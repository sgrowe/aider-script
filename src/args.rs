use clap::Parser;
use clio::Input;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(name = "aider-script")]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Template file, use '-' for stdin
    #[clap(value_parser)]
    template: Input,

    /// Template arguments
    pub template_arguments: Vec<String>,

    /// Outputs the message that would be passed to aider and then exits
    #[arg(short, long)]
    pub preview_message: bool,
}

impl Args {
    pub fn read_template(&mut self) -> anyhow::Result<String> {
        let mut message = String::new();

        self.template.read_to_string(&mut message)?;

        Ok(message)
    }

    pub fn get_template_name(&self) -> &str {
        self.template_filename().unwrap_or("template")
    }

    fn template_filename(&self) -> Option<&str> {
        let path_str = self.template.path().to_str()?;

        // Extract just the filename portion
        let path = std::path::Path::new(path_str);

        let file_name = path.file_name()?;

        let name = file_name.to_str()?;

        Some(name)
    }
}
