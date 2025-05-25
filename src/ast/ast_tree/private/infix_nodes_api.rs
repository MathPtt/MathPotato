use crate::ast::ast_tree::public::infix_nodes_api::InfixNodesApi;

pub mod entity;
pub mod storage;

pub trait InfixAstTreeApi {}
impl InfixAstTreeApi for InfixNodesApi {}
