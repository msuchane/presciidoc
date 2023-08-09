/// A String loaded from a file with AsciiDoc markup.
pub struct AsciiDocText<'a> {
    pub text: &'a str,
    numbered_lines: Vec<(usize, &'a str)>,
}

impl<'a> AsciiDocText<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            numbered_lines: text.lines().enumerate().collect(),
        }
    }
}
