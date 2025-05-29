use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::{InfixNode, InfixNodeApi};

pub trait InfixNodeApiGetRightType: InfixNodeApi {
    fn get_right_type(&self) -> &AstNodeType;
}
impl InfixNodeApiGetRightType for InfixNode {
    fn get_right_type(&self) -> &AstNodeType {
        &self.right_type
    }
}
