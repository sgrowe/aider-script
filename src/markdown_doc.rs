pub struct MarkdownDoc<'a> {
    // TODO: remove `pub`
    pub frontmatter: &'a str,
    pub body: &'a str,
}

impl<'a> MarkdownDoc<'a> {
    pub fn parse(markdown: &'a str) -> Self {
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
            Some((s, e)) => Self {
                frontmatter: &markdown[s..e],
                body: &markdown[e..],
            },
            None => {
                // No front matter found
                Self {
                    frontmatter: "",
                    body: markdown,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_markdown_with_frontmatter() {
        let markdown = fs::read_to_string("src/fixtures/01_args.md").expect("Failed to read fixture file");
        let doc = MarkdownDoc::parse(&markdown);
        
        assert!(doc.frontmatter.contains("args:"));
        assert!(doc.frontmatter.contains("- FUNCTION"));
        assert!(doc.body.contains("# Add unit tests for FUNCTION"));
    }
}
