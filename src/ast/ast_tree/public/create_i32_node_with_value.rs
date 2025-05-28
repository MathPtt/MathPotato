use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::AstApi;
pub mod implementation;
pub mod node;

pub trait CreateI32NodeWithValue: AstApi {
    /// Creates an `I32Node` in the Abstract Syntax Tree wit the provided value.
    ///
    /// # Parameters
    /// - `i32_value` - the `i32` value to be stored in the node.
    ///
    /// # Returns
    /// - `Ok(Uuid)` - The newly created node's id value.
    /// - `Err(ParseError)` - If any error happens during the node creation.
    fn create_i32_node_with_value(&mut self, i32_value: i32) -> Result<Uuid, ParseError>;
}
