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

// Add a unit test that reads in `src/fixtures/01_args.md`, calls MarkdownDoc::parse on it, and asserts that the returned frontmatter is `args: \n  - FUNCTION` AI!
