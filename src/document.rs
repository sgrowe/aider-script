pub struct Document<'a> {
    // TODO: remove `pub`
    pub frontmatter: &'a str,
    pub body: &'a str,
}
