use crate::ast::ast_tree::public::infix_nodes_api::InfixNodesApi;

pub mod add_left_child_node_to_node;
pub mod does_node_exist;
pub mod entity;
pub mod get_all;
pub mod len;
pub mod put;
pub mod storage;
pub mod update;

pub trait InfixAstTreeApi {}
impl InfixAstTreeApi for InfixNodesApi {}
