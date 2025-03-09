use std::fs;
use std::process::Command;

fn main() {
    let script_file_path = "ai/example.md";

    let message = fs::read_to_string(script_file_path).expect("Failed to read the file");

    let mut cmd = create_aider_command(&message);
    cmd.status().expect("Failed to execute aider command");
}

fn create_aider_command(markdown: &str) -> Command {
    // `markdown` will be a markdown document with a frontmatter section enclosed between two lines made entirely of dashes (e.g. `----`)
    let mut cmd = Command::new("aider");

    let document = extract_frontmatter(markdown);

    // Add frontmatter as a separate argument if it's not empty
    if !document.frontmatter.trim().is_empty() {
        cmd.arg("--frontmatter").arg(document.frontmatter.trim());
    }

    // Add the body as the main message
    cmd.arg("-m").arg(document.body.trim());

    cmd
}

struct Document<'a> {
    frontmatter: &'a str,
    body: &'a str,
}

fn extract_frontmatter(markdown: &str) -> Document {
    // Try to find a separator line with any number of dashes (at least 3)
    if let Some(separator_pos) = markdown
        .lines()
        .enumerate()
        .find(|(_, line)| line.trim().chars().all(|c| c == '-') && line.trim().len() >= 3)
        .map(|(idx, _)| idx)
    {
        // Split the content at the separator
        let lines: Vec<&str> = markdown.lines().collect();

        // Calculate the correct indices for frontmatter and body
        let frontmatter_start = 0;
        let frontmatter_end = lines[..separator_pos].join("\n").len();
        
        // Find the position after the separator line
        let separator_line = lines[separator_pos];
        let separator_pos_in_str = markdown.find(separator_line).unwrap_or(0) + separator_line.len();
        
        // Find the start of the body (skipping any newlines after the separator)
        let body_start = if let Some(pos) = markdown[separator_pos_in_str..].find(|c: char| c != '\n') {
            separator_pos_in_str + pos
        } else {
            separator_pos_in_str
        };
        
        Document {
            frontmatter: &markdown[frontmatter_start..frontmatter_end],
            body: &markdown[body_start..],
        }
    } else {
        // No separator found
        Document {
            frontmatter: "",
            body: markdown,
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
        let message =
            "title: Test Title\ntags: test, example\n\n----\n\nThis is the body of the document.";
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
        let message = "frontmatter\n----\nbody part 1\n----\nbody part 2";
        let cmd = create_aider_command(message);

        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();

        // Check that only the first separator is used for frontmatter
        assert_eq!(args.len(), 4);
        assert_eq!(args[0], "--frontmatter");
        assert_eq!(args[1].to_string_lossy(), "frontmatter");
        assert_eq!(args[2], "-m");
        assert_eq!(args[3].to_string_lossy(), "body part 1\n----\nbody part 2");
    }

    #[test]
    fn test_create_aider_command_with_variable_dash_count() {
        let message = "frontmatter\n-----\nbody content";
        let cmd = create_aider_command(message);

        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();

        // Check that separator with 5 dashes works
        assert_eq!(args.len(), 4);
        assert_eq!(args[0], "--frontmatter");
        assert_eq!(args[1].to_string_lossy(), "frontmatter");
        assert_eq!(args[2], "-m");
        assert_eq!(args[3].to_string_lossy(), "body content");
    }

    #[test]
    fn test_create_aider_command_with_minimum_dashes() {
        let message = "frontmatter\n---\nbody content";
        let cmd = create_aider_command(message);

        // Convert args to a Vec for easier testing
        let args: Vec<_> = cmd.get_args().collect();

        // Check that separator with 3 dashes works
        assert_eq!(args.len(), 4);
        assert_eq!(args[0], "--frontmatter");
        assert_eq!(args[1].to_string_lossy(), "frontmatter");
        assert_eq!(args[2], "-m");
        assert_eq!(args[3].to_string_lossy(), "body content");
    }
}
