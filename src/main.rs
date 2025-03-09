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

    // Split the markdown by separator lines with any number of dashes
    let parts: Vec<&str> = markdown.split_inclusive(&['\n'])
        .collect::<Vec<_>>()
        .windows(2)
        .enumerate()
        .filter_map(|(i, window)| {
            let line = window[0].trim_end();
            let next_line_starts_with_newline = window[1].starts_with('\n');
            
            if line.chars().all(|c| c == '-') && line.len() >= 3 && next_line_starts_with_newline {
                Some(i)
            } else {
                None
            }
        })
        .nth(0)
        .map_or_else(
            || vec![markdown],
            |separator_idx| {
                let lines: Vec<&str> = markdown.split_inclusive(&['\n']).collect();
                let frontmatter = lines[..separator_idx].concat();
                let body = lines[separator_idx + 1..].concat();
                vec![frontmatter, body]
            }
        );

    if parts.len() >= 2 {
        // Extract frontmatter and body
        let frontmatter = parts[0].trim().to_string();
        let body_string = parts[1..].join("\n----\n");
        let body = body_string.trim().to_string();

        // Add frontmatter as a separate argument if it's not empty
        if !frontmatter.is_empty() {
            cmd.arg("--frontmatter").arg(frontmatter);
        }

        // Add the body as the main message
        cmd.arg("-m").arg(body);
    } else {
        // If no separator found, use the entire markdown as the message
        cmd.arg("-m").arg(markdown);
    }

    cmd
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
