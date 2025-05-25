use std::any::type_name;

use uuid::Uuid;

use crate::{
    ast::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::nodes::infix_ast_node_internal::InfixAstNodeInternal,
    },
    parser::parser_error::ParseError,
};

use super::{does_node_exist::InfixAstTreeApiGet, InfixAstTreeApi, InfixNodesApi};

pub trait InfixAstTreeApiUpdate: InfixAstTreeApi {
    fn update(
        &mut self,
        id: Uuid,
        node: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError>;
}
impl InfixAstTreeApiUpdate for InfixNodesApi {
    fn update(
        &mut self,
        id: Uuid,
        node: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
        match self.does_node_exist(id) {
            false => Err(ParseError::new(format!(
                "There is no {:#?} type node with id: {}.",
                type_name::<AstNodeType>(),
                id
            ))),
            true => {
                self.tree.insert(id, node.clone());
            }
        }
    }
}
