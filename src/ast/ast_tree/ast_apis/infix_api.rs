use crate::ast::ast_tree::MathPotatoAstTree;

pub mod infix_api_get_node_count;
pub mod infix_api_get_nodes;
pub mod infix_api_put_node;
pub mod infix_api_update_by_id;
pub mod infix_get_by_id;

pub trait InfixApi {}
impl InfixApi for MathPotatoAstTree {}
