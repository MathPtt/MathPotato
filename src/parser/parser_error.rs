#[derive(Debug)]
pub struct PotatoParserError {
    details: String,
}

impl PotatoParserError {
    pub fn new(details: String) -> Self {
        Self { details }
    }
}
