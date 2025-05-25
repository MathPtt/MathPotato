use derive_more::Display;

#[derive(Debug, Display)]
#[display("ParseError(details: {})", details)]
pub struct ParseError {
    pub details: String,
}

impl ParseError {
    pub fn new(details: String) -> Self {
        Self { details }
    }
}
