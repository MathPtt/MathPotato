use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::{InfixAstNodeInternal, InfixAstNodeInternalApi};

pub trait InfixAstNodeInternalSetLeftType: InfixAstNodeInternalApi {
    fn set_left_type(&mut self, left_type: AstNodeType) -> Result<AstNodeType, ParseError>;
}
impl InfixAstNodeInternalSetLeftType for InfixAstNodeInternal {
    fn set_left_type(&mut self, left_type: AstNodeType) -> Result<AstNodeType, ParseError> {
        self.left_type = left_type.clone();
        Ok(left_type)
    }
}
