use uuid::Uuid;

use crate::{ast::ast_tree::public::AstApi, parser::parser_error::ParseError};

pub mod implementation;

pub trait FindNextFeasibleContinuationNode: AstApi {
    /// Finds the next feasible continuation node in the AST Tree.
    ///
    /// Finding the next feasible continuation node depends on:
    /// - last processed token
    /// - actual token
    /// - precedences
    fn find_next_feasible_continuation_node_and_set_as_actual_continuation_node(
        &self,
    ) -> Result<Uuid, ParseError>;
}
