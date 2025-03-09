use std::vec;

use crate::markdown_doc::MarkdownDoc;

#[derive(Debug)]
pub struct CommandTemplate<'a> {
    argument_names: Vec<&'a str>,
    template_body: &'a str,
}

impl<'a> CommandTemplate<'a> {
    pub fn parse(s: &'a str) -> Self {
        let MarkdownDoc { frontmatter, body } = MarkdownDoc::parse(s);

        Self {
            argument_names: vec![],
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
