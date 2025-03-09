use std::vec;
use yaml_rust2::YamlLoader;

use crate::markdown_doc::MarkdownDoc;

#[derive(Debug)]
pub struct CommandTemplate<'a> {
    argument_names: Vec<&'a str>,
    template_body: &'a str,
}

impl<'a> CommandTemplate<'a> {
    pub fn parse(s: &'a str) -> Self {
        let MarkdownDoc { frontmatter, body } = MarkdownDoc::parse(s);

        let mut argument_names = Vec::new();
        
        if !frontmatter.is_empty() {
            if let Ok(docs) = YamlLoader::load_from_str(frontmatter) {
                if !docs.is_empty() {
                    if let Some(args) = docs[0]["args"].as_vec() {
                        for arg in args {
                            if let Some(arg_str) = arg.as_str() {
                                argument_names.push(arg_str);
                            }
                        }
                    }
                }
            }
        }

        Self {
            argument_names,
            template_body: body,
        }
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

        let doc = CommandTemplate::parse(&markdown);

        assert!(doc.argument_names == vec!["FUNCTION"]);
    }
}
