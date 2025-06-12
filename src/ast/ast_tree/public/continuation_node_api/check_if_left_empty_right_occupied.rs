use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContNodeApiCheckIfLeftSideEmptyRightSideOccupied: ContinuationNodeStorageApi {
    fn cont_node_api_check_if_left_empty_right_occupied(
        &self,
        id: Uuid,
    ) -> Result<bool, ParseError>;
}
