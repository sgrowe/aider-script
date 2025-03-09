pub fn substr_index_within_parent(substr: &str, doc: &str) -> usize {
    // From https://users.rust-lang.org/t/what-is-the-best-way-to-iterate-over-lines-of-a-string-with-offset/120021/10

    substr.as_ptr() as usize - doc.as_ptr() as usize
}
