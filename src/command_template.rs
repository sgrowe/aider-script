use std::fmt::Debug;

use tera::{Context, Tera};
use yaml_rust2::YamlLoader;

use crate::{aider_command::AiderCommand, filters, markdown_doc::MarkdownDoc};

#[derive(Debug)]
pub struct CommandTemplate<'a> {
    argument_names: Vec<String>,
    template_body: &'a str,
    template_name: &'a str,

    /// File paths which will be passed as read-only to aider (using `--read`)
    read_only: Vec<String>,
    /// File paths to be edited by aider (passed using `--file`)
    edit: Vec<String>,
}

impl<'a> CommandTemplate<'a> {
    pub fn parse_with_name(s: &'a str, name: &'a str) -> anyhow::Result<Self> {
        let MarkdownDoc { frontmatter, body } = MarkdownDoc::parse(s);

        let mut argument_names = Vec::new();
        let mut read_only = Vec::new();
        let mut edit = Vec::new();

        if !frontmatter.trim().is_empty() {
            let docs = YamlLoader::load_from_str(frontmatter)?;

            if let Some(args) = docs[0]["args"].as_vec() {
                for arg in args {
                    if let Some(arg_str) = arg.as_str() {
                        argument_names.push(arg_str.into());
                    }
                }
            }

            if let Some(read_files) = docs[0]["read"].as_vec() {
                for file in read_files {
                    if let Some(file_str) = file.as_str() {
                        read_only.push(file_str.into());
                    }
                }
            }

            if let Some(edit_files) = docs[0]["edit"].as_vec() {
                for file in edit_files {
                    if let Some(file_str) = file.as_str() {
                        edit.push(file_str.into());
                    }
                }
            }
        }

        Ok(Self {
            argument_names,
            template_body: body,
            template_name: name,
            read_only,
            edit,
        })
    }

    pub fn apply_args<T>(&self, args: &[T]) -> anyhow::Result<AiderCommand>
    where
        T: AsRef<str> + Debug,
    {
        // Validate that we have enough arguments
        if args.len() < self.argument_names.len() {
            if let Some(missing_arg) = self.argument_names.get(args.len()) {
                return Err(anyhow::anyhow!(
                    "Missing expected argument \"{}\".",
                    missing_arg
                ));
            }
        }

        // Create a Tera instance with a single template
        let mut tera = Tera::default();
        tera.add_raw_template(self.template_name, self.template_body)?;
        
        // Register our case conversion filters
        tera.register_filter("kebab", filters::KebabFilter);
        tera.register_filter("pascal", filters::PascalFilter);
        tera.register_filter("camel", filters::CamelFilter);
        tera.register_filter("snake", filters::SnakeFilter);

        // Create context with variables
        let mut context = Context::new();
        for (name, value) in self.argument_names.iter().zip(args) {
            context.insert(name, value.as_ref());
        }

        // Render the template
        let rendered = tera.render(self.template_name, &context)?;

        let mut command = AiderCommand::message(rendered);

        // Apply templating to read_only and edit file paths
        command.read_only = render_strings(&self.read_only, &mut tera, &context)?;
        command.edit = render_strings(&self.edit, &mut tera, &context)?;

        Ok(command)
    }
}

fn render_strings<P>(paths: P, tera: &mut Tera, context: &Context) -> tera::Result<Vec<String>>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    paths
        .into_iter()
        .map(|path| tera.render_str(path.as_ref(), context))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_markdown_with_frontmatter() {
        let markdown =
            fs::read_to_string("src/fixtures/01_args.md").expect("Failed to read fixture file");

        let doc = CommandTemplate::parse_with_name(&markdown, "01_args.md").unwrap();

        assert_eq!(doc.argument_names, vec!["FUNCTION"]);
        assert_eq!(doc.read_only, vec!["src/str.rs", "src/args.rs"]);
        assert_eq!(doc.edit, vec!["src/main.rs"]);
    }

    #[test]
    fn errors_if_required_args_are_not_given() {
        let markdown =
            fs::read_to_string("src/fixtures/01_args.md").expect("Failed to read fixture file");

        let doc = CommandTemplate::parse_with_name(&markdown, "01_args.md").unwrap();

        let cmd = doc.apply_args::<&str>(&[]).unwrap_err();

        assert!(cmd.to_string() == "Missing expected argument \"FUNCTION\".");
    }

    #[test]
    fn test_applies_given_arguments_to_the_template() {
        let markdown =
            fs::read_to_string("src/fixtures/01_args.md").expect("Failed to read fixture file");

        let doc = CommandTemplate::parse_with_name(&markdown, "01_args.md").unwrap();

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

    #[test]
    fn test_templates_file_paths() {
        let markdown = r#"---
args:
  - FUNCTION
read:
  - "src/{{ FUNCTION }}.rs"
edit:
  - "src/{{ FUNCTION }}_test.rs"
---
Test template"#;

        let doc = CommandTemplate::parse_with_name(markdown, "test_template").unwrap();
        let cmd = doc.apply_args(&["utils"]).unwrap();

        assert_eq!(cmd.read_only, vec!["src/utils.rs"]);
        assert_eq!(cmd.edit, vec!["src/utils_test.rs"]);
    }

    #[test]
    fn test_casing_filters() {
        let markdown = fs::read_to_string("src/fixtures/02_casing_filters.md")
            .expect("Failed to read fixture file");

        let doc = CommandTemplate::parse_with_name(&markdown, "02_casing_filters.md").unwrap();

        let cmd = doc.apply_args(&["myQueryName"]).unwrap();

        assert_eq!(
            cmd.message,
            "
- Create a new file called `src/component/my-query-name.tsx`
- Add a React component in that file which calls the `useMyQueryNameQuery()` hook and renders the result
- Render some text above the result saying \"Response from myQueryName\"

Snake case for the hell of it: my_query_name
".trim()
        )
    }
}
