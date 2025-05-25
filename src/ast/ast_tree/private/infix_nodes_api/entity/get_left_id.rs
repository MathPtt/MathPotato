use uuid::Uuid;

use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::{InfixAstNodeInternal, InfixAstNodeInternalApi};

pub trait InfixAstNodeInternalGetLeftId: InfixAstNodeInternalApi {
    fn get_left_id(&self) -> Result<Uuid, ParseError>;
}
impl InfixAstNodeInternalGetLeftId for InfixAstNodeInternal {
    fn get_left_id(&self) -> Result<Uuid, ParseError> {
        if self.left_id == Uuid::nil() {
            Err(ParseError::new(format!(
                "There is no left if for {:#?} with id:",
                AstNodeType::InfixOperationAstNode
            )))
        } else {
            Ok(self.left_id)
        }
    }
}
