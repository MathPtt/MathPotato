use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{GetContinuationNodeIdAndTypeResult, GetContinuationNodeIdAndTypeResultApi};

pub trait GetContinuationNodeIdAndTypeResultGetId: GetContinuationNodeIdAndTypeResultApi {
    fn get_id(&self) -> Result<Uuid, ParseError>;
}
impl GetContinuationNodeIdAndTypeResultGetId for GetContinuationNodeIdAndTypeResult {
    fn get_id(&self) -> Result<Uuid, ParseError> {
        if self.id == Uuid::nil() {
            Err(ParseError::new(format!(
                "No continuation node exist. The id is nil.",
            )))
        } else {
            Ok(self.id.clone())
        }
    }
}
