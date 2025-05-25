use std::collections::HashMap;

use crate::ast::ast_tree::private::i32_nodes_api::I32AstTreeApi;

use super::I32NodesApi;

pub trait I32AstTreeApiNew: I32AstTreeApi {
    fn new() -> I32NodesApi;
}
impl I32AstTreeApiNew for I32NodesApi {
    fn new() -> I32NodesApi {
        I32NodesApi {
            tree: HashMap::new(),
        }
    }
}
