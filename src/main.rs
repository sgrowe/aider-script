use std::fs;
use std::process::Command;

use document::Document;

mod document;

fn main() {
    let script_file_path = "ai/example.md";

    let message = fs::read_to_string(script_file_path).expect("Failed to read the file");

    let mut cmd = create_aider_command(&message);
    cmd.status().expect("Failed to execute aider command");
}

fn create_aider_command(markdown: &str) -> Command {
    let document = parse_document(markdown);

    // TODO:
    // document.to_aider_command()

    let mut cmd = Command::new("aider");

    // Add the body as the main message
    cmd.arg("-m").arg(document.body.trim());

    cmd
}

fn parse_document(markdown: &str) -> Document {
    // `markdown` will be a markdown document with a frontmatter section enclosed between two lines of three dashes

    let lines = markdown.lines();

    let mut front_matter_start = None;
    let mut front_matter_end = None;

    for line in lines {
        if line == "---" {
            let offset = line.as_ptr() as usize - markdown.as_ptr() as usize;

            if front_matter_start.is_none() {
                front_matter_start = Some(offset);
            } else {
                front_matter_end = Some(offset);
                break;
            }
        }
    }

    match front_matter_start.zip(front_matter_end) {
        Some((s, e)) => Document {
            frontmatter: &markdown[s..e],
            body: &markdown[e..],
        },
        None => {
            // No front matter found
            Document {
                frontmatter: "",
                body: markdown,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aider_command_basic() {
        let message = "Test message";
        let cmd = create_aider_command(message);

        // Check that the program is "aider"
        assert_eq!(cmd.get_program(), "aider");
    }

    #[test]
    fn test_create_aider_command_args() {
        let message = "Test message";
        let cmd = create_aider_command(message);

        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();

        // Check that we have the -m flag followed by the message
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-m");
        assert_eq!(args[1].to_string_lossy(), message);
    }

    #[test]
    fn test_create_aider_command_with_frontmatter() {
        let message = "----\ntitle: Test Title\ntags: test, example\n\n----\n\nThis is the body of the document.";
        let cmd = create_aider_command(message);

        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();

        // Check that we have the --frontmatter flag followed by the frontmatter, then -m flag followed by the body
        assert_eq!(args.len(), 4);
        assert_eq!(args[0], "--frontmatter");
        assert_eq!(
            args[1].to_string_lossy(),
            "title: Test Title\ntags: test, example"
        );
        assert_eq!(args[2], "-m");
        assert_eq!(
            args[3].to_string_lossy(),
            "This is the body of the document."
        );
    }

    #[test]
    fn test_create_aider_command_with_multiple_separators() {
        let message = "----\nfrontmatter\n----\nbody part 1\n----\nbody part 2";
        let cmd = create_aider_command(message);

        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();

        // Check that only the first separator is used for frontmatter
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-m");
        assert_eq!(args[1].to_string_lossy(), "body part 1\n----\nbody part 2");
    }

    #[test]
    fn test_create_aider_command_with_variable_dash_count() {
        let message = "----\nfrontmatter\n-----\nbody content";
        let cmd = create_aider_command(message);

        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();

        // Check that separator with 5 dashes works
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-m");
        assert_eq!(args[1].to_string_lossy(), "body content");
    }

    #[test]
    fn test_create_aider_command_with_minimum_dashes() {
        let message = "frontmatter\n---\nbody content";
        let cmd = create_aider_command(message);

        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();

        // Check that separator with 3 dashes is ignored (not enough dashes)
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "-m");
        assert_eq!(args[1].to_string_lossy(), "frontmatter\n---\nbody content");
    }
}
