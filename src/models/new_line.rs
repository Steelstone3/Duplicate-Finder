use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct NewLine {
    pub line_number: u128,
    pub content: String,
}

impl Display for NewLine {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Line Number:: {}\nContent:: {}", self.line_number, self.content)
    }
}