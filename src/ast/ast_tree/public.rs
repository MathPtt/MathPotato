use super::MathPotatoAstTree;

pub mod create_i32_node_with_value;
pub mod get_continuation_node_id_and_type;

pub trait AstApi {}
impl AstApi for MathPotatoAstTree {}
