use super::{
    integer_statement_ast_node::IntegerStatementAstNode,
    integer_value_expression_ast_node::IntegerValueExpressionAstNode,
};

/// Represents the nodes in the Potato Abstract Syntax Tree.
/// The different enum values represent the types the nodes can have in Potato Language's Abstract
/// Syntax Tree.
///
/// # Node types
///
/// - `None` - when a node doesn't have any value, it is a `null` like value, the default one.
/// -`IntegerStatementAstNode` - when integer value, `i32`, assignment is happening.
/// - `IntegerValueExpressionAstNode` - represents an Integer, `i32`, value in the AST tree.
///
#[derive(PartialEq, Debug, Default, Clone)]
pub enum PotatoAstNode {
    /// represents an integer value assignment,
    /// like `Integer asd = 1 + 2;`
    IntegerStatementAstNode(IntegerStatementAstNode),
    /// Represents an Integer, `i32`, value in the AST
    IntegerValueExpressionAstNode(IntegerValueExpressionAstNode),
    /// represents a `null` like value
    #[default]
    None,
}

pub trait PotatoAstNodeGUID {
    fn get_uuid();
}
