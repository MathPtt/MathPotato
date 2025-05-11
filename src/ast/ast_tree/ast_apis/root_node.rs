use crate::ast::ast_tree::MathPotatoAstTree;

pub mod add_root_node_id_and_type;
pub mod get_root_node_id;
pub mod get_root_node_type;
pub mod root_node_api_get_infix_by_id;
pub mod update_root_node_id_and_type;
pub trait RootNodeApi {}
impl RootNodeApi for MathPotatoAstTree {}
