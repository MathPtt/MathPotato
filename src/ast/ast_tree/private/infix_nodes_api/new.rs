use std::collections::HashMap;

use super::{InfixAstTreeApi, InfixNodesApi};

pub trait InfixAstTreeApiNew: InfixAstTreeApi {
    fn new() -> Self;
}
impl InfixAstTreeApiNew for InfixNodesApi {
    fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }
}
