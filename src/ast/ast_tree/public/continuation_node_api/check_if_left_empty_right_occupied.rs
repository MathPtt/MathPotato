use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::ContinuationNodeApi;

pub mod implementation;

pub trait ContNodeApiCheckIfLeftSideEmptyRightSideOccupied: ContinuationNodeApi {
    fn cont_node_api_check_if_left_empty_right_occupied(
        &self,
        id: Uuid,
    ) -> Result<bool, ParseError>;
}
