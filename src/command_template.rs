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
