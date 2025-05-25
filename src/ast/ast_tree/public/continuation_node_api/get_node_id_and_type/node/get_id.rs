use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{GetNodeIdAndTypeResult, GetNodeIdAndTypeResultApi};

pub trait ContinuationNodeApiGetId: GetNodeIdAndTypeResultApi {
    fn get_id(&self) -> Result<Uuid, ParseError>;
}
impl ContinuationNodeApiGetId for GetNodeIdAndTypeResult {
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
