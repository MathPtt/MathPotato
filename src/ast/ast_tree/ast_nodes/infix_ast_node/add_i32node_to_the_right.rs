use uuid::Uuid;

use crate::{ast::ast_node_types_enum::AstNodeType, parser::parser_error::ParseError};

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeAddI32NodeToTheRight: InfixAstNodeApi {
    fn add_i32node_to_the_right(&mut self, id: Uuid) -> Result<(Uuid, InfixAstNode), ParseError>;
}
impl InfixAstNodeAddI32NodeToTheRight for InfixAstNode {
    fn add_i32node_to_the_right(&mut self, id: Uuid) -> Result<(Uuid, InfixAstNode), ParseError> {
        if self.right_id != Uuid::nil() && self.right_type != AstNodeType::None {
            Err(ParseError::new(format!(
                "The right side is not empty. It is occupied by id: {:#?} and type: {:#?}",
                self.right_id, self.right_type
            )))
        } else {
            self.right_id = id;
            self.right_type = AstNodeType::I32AstNode;
            Ok((id, self.clone()))
        }
    }
}
