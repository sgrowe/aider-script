use std::borrow::Cow;

use yaml_rust2::YamlLoader;

use crate::{aider_command::AiderCommand, markdown_doc::MarkdownDoc};

#[derive(Debug)]
pub struct CommandTemplate<'a> {
    argument_names: Vec<String>,
    template_body: &'a str,
}

impl<'a> CommandTemplate<'a> {
    pub fn parse(s: &'a str) -> anyhow::Result<Self> {
        let MarkdownDoc { frontmatter, body } = MarkdownDoc::parse(s);

        let mut argument_names = Vec::new();

        if !frontmatter.trim().is_empty() {
            let docs = YamlLoader::load_from_str(frontmatter)?;

            if let Some(args) = docs[0]["args"].as_vec() {
                for arg in args {
                    if let Some(arg_str) = arg.as_str() {
                        argument_names.push(arg_str.into());
                    }
                }
            }
        }

        Ok(Self {
            argument_names,
            template_body: body,
        })
    }

    pub fn apply_args(&self, args: &[&str]) -> anyhow::Result<AiderCommand> {
        let mut message = Cow::Borrowed(self.template_body);

        for (name, value) in self.argument_names.iter().zip(args) {
            message = Cow::Owned(message.replace(name, value));
        }

        Ok(AiderCommand {
            message: message.into_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_markdown_with_frontmatter() {
        let markdown =
            fs::read_to_string("src/fixtures/01_args.md").expect("Failed to read fixture file");

        let doc = CommandTemplate::parse(&markdown).unwrap();

        assert_eq!(doc.argument_names, vec!["FUNCTION"]);
    }

    #[test]
    fn test_applies_given_arguments_to_the_template() {
        let markdown =
            fs::read_to_string("src/fixtures/01_args.md").expect("Failed to read fixture file");

        let doc = CommandTemplate::parse(&markdown).unwrap();

        let cmd = doc.apply_args(&["my_func_1"]).unwrap();

        assert_eq!(
            cmd.message,
            "# Add unit tests for my_func_1

## Step 1 - think about what should be tested

Read `my_func_1` and think about how a Senior Rust Software Engineer would want to test it.

## Step 2 - add placeholder tests

Add placeholders for each of those unit tests using `todo!()`

Example:

```rs
#[test]
fn test_my_func_1_does_X() {
    todo!()
}
```

## Step 3 - implement tests

Now implement those unit tests"
        )
    }
}
