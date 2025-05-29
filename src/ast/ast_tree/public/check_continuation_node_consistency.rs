use crate::parser::parser_error::ParseError;

use super::AstApi;

pub mod implementation;

pub trait CheckContinuationNodeConsistency: AstApi {
    /// Checks the continuation node consistency for the tree at the point of processing.
    /// This means that the continuation node cannot have its left side empty while the right side
    /// occupied.
    ///
    /// # Returns
    /// - `Ok(true)` - when the consistency is ok.
    /// - `Err(ParseError)` - when the node is in inconsistent state. The returned `ParseError`
    /// includes all the details.
    fn check_continuation_node_consistency(&self) -> Result<bool, ParseError>;
}
