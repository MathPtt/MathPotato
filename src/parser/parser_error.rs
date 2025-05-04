#[derive(Debug)]
pub struct ParseError {
    pub details: String,
}

impl ParseError {
    pub fn new(details: String) -> Self {
        Self { details }
    }
}
