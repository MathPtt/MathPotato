#[derive(Debug)]
pub struct MathPotatoParserError {
    details: String,
}

impl MathPotatoParserError {
    pub fn new(details: String) -> Self {
        Self { details }
    }
}
