use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::{InfixNode, InfixNodeApi};

pub trait InfixNodeApiSetLeftType: InfixNodeApi {
    fn set_left_type(&mut self, left_type: AstNodeType) -> Result<AstNodeType, ParseError>;
}
impl InfixNodeApiSetLeftType for InfixNode {
    fn set_left_type(&mut self, left_type: AstNodeType) -> Result<AstNodeType, ParseError> {
        self.left_type = left_type.clone();
        Ok(left_type)
    }
}
