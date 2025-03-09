use crate::str::substr_index_within_parent;

#[derive(Debug)]
pub struct MarkdownDoc<'a> {
    pub frontmatter: &'a str,
    pub body: &'a str,
}

impl<'a> MarkdownDoc<'a> {
    pub fn parse(markdown: &'a str) -> Self {
        // `markdown` will be a markdown document with a frontmatter section enclosed between two lines of three dashes

        let mut lines = markdown.lines();

        // Extract front matter
        let mut fm_start = None;
        let mut fm_end = None;
        let mut body_start = None;

        while let Some(line) = lines.next() {
            if line != "---" {
                continue;
            }

            if fm_start.is_none() {
                // Start of front matter

                fm_start = lines
                    .next()
                    .map(|l| substr_index_within_parent(l, markdown));
            } else {
                // End of front matter

                let offset = substr_index_within_parent(line, markdown);

                fm_end = Some(offset);
                body_start = lines
                    .next()
                    .map(|l| substr_index_within_parent(l, markdown));

                break;
            }
        }

        let frontmatter = match fm_start.zip(fm_end) {
            Some((s, e)) => markdown[s..e].trim_end(),
            None => {
                // No front matter found
                ""
            }
        };

        let body = markdown[body_start.unwrap_or(0)..].trim();

        Self { frontmatter, body }
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
        let doc = MarkdownDoc::parse(&markdown);

        assert_eq!(doc.frontmatter, "args:\n  - FUNCTION");

        assert!(doc.body.starts_with("# Add unit tests for FUNCTION\n"));
    }

    #[test]
    fn test_parse_markdown_without_frontmatter() {
        let markdown = "
             # Example doc

             No frontmatter here.
             ";
        let doc = MarkdownDoc::parse(markdown);

        assert_eq!(doc.frontmatter, "");
        assert!(doc.body.starts_with("# Example doc"));
    }
}
