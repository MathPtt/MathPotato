use crate::ast::ast_tree::MathPotatoAstTree;

pub mod get_type;

pub trait NodesCatalogApi {}
impl NodesCatalogApi for MathPotatoAstTree {}
