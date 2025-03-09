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

        while let Some(line) = lines.next() {
            if line != "---" {
                continue;
            }

            if fm_start.is_none() {
                if let Some(line) = lines.next() {
                    let offset = (line.as_ptr() as usize) - markdown.as_ptr() as usize;

                    fm_start = Some(offset);
                }
            } else {
                let offset = (line.as_ptr() as usize) - markdown.as_ptr() as usize;
                fm_end = Some(offset);
                break;
            }
        }

        match fm_start.zip(fm_end) {
            Some((s, e)) => Self {
                frontmatter: &markdown[s..e].trim_end(),
                body: &markdown[e..].trim_end(),
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
        let markdown =
            fs::read_to_string("src/fixtures/01_args.md").expect("Failed to read fixture file");
        let doc = MarkdownDoc::parse(&markdown);

        assert!(doc.frontmatter == "args:\n  - FUNCTION");
    }

    #[test]
    fn test_parse_markdown_without_frontmatter() {
        let markdown = "
             # Example doc

             No frontmatter here.
             ";
        let doc = MarkdownDoc::parse(&markdown);

        assert!(doc.frontmatter == "");
    }
}
