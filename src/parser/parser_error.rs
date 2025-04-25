#[derive(Debug)]
pub struct ParserError {
    details: String,
}

impl ParserError {
    pub fn new(details: String) -> Self {
        Self { details }
    }
}
