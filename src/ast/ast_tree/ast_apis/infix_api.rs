use crate::ast::ast_tree::MathPotatoAstTree;

pub mod get_by_id;
pub mod get_node_count;
pub mod get_nodes;
pub mod put_node;
pub mod update_by_id;

/// The Infix Operation Node Api
///
/// This Api provides methods to manage the `InfixNode`s in the Abstract Syntax Tree.
///
/// # Infix Operation Node
///
/// The InfixAstNode represents oerations, like `-`, `+`, `*` or `/`, in an variable assignemt
/// expression. The set of mathematical operations has a precedence order which defines the
/// execution order. The Parser is responsible to build the AST in a correct way. This task
/// requires this Api.
pub trait InfixApi {}
impl InfixApi for MathPotatoAstTree {}
