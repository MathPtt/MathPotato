use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{
    ast_node_types_enum::AstNodeType, infix_operation_type_enum::InfixOperationTypeEnum,
    internal::infix_ast_node_internal::InfixAstNodeInternal,
};

#[derive(Debug, Clone)]
pub struct InfixAstNode {
    pub id: Uuid,
    pub operation_type: InfixOperationTypeEnum,
    pub left_id: Uuid,
    pub left_type: AstNodeType,
    pub right_id: Uuid,
    pub right_type: AstNodeType,
}

impl InfixAstNode {
    pub fn to_internal(&self) -> Result<InfixAstNodeInternal, ParseError> {
        Ok(InfixAstNodeInternal {
            operation_type: self.operation_type.clone(),
            left_id: self.left_id,
            left_type: self.left_type.clone(),
            right_id: self.right_id,
            right_type: self.right_type.clone(),
        })
    }

    pub fn from_internal_and_id(
        id: Uuid,
        internal_node: InfixAstNodeInternal,
    ) -> Result<InfixAstNode, ParseError> {
        Ok(InfixAstNode {
            id,
            operation_type: internal_node.operation_type.clone(),
            left_id: internal_node.left_id,
            left_type: internal_node.left_type.clone(),
            right_id: internal_node.right_id,
            right_type: internal_node.right_type.clone(),
        })
    }

    pub fn new_with_type_and_left_child_node(
        infix_operation_type: InfixOperationTypeEnum,
        left_node_type: AstNodeType,
        left_node_id: Uuid,
    ) -> InfixAstNode {
        InfixAstNode {
            id: Uuid::new_v4(),
            left_id: left_node_id,
            left_type: left_node_type,
            right_type: AstNodeType::None,
            right_id: Uuid::nil(),
            operation_type: infix_operation_type,
        }
    }

    pub(crate) fn check_if_left_empty_right_occupied(&self) -> Result<(), ParseError> {
        if self.left_id != Uuid::nil()
            && self.left_type != AstNodeType::None
            && self.right_id != Uuid::nil()
            && self.right_type != AstNodeType::None
        {
            Err(ParseError::new(String::from("InfixOperationAstNode is in an impossible state. It has its left side empty, but right side occupied.")))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_i32node_to_the_right(
        &mut self,
        id: Uuid,
    ) -> Result<(Uuid, InfixAstNode), ParseError> {
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

    pub fn get_operation_type(&self) -> InfixOperationTypeEnum {
        self.operation_type.clone()
    }

    pub fn get_left_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.left_id == Uuid::nil() {
            None
        } else {
            Some((self.left_id, self.left_type.clone()))
        }
    }

    pub fn get_right_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some((self.right_id, self.right_type.clone()))
        }
    }

    pub fn get_right_node(&self) -> Option<(Uuid, AstNodeType)> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some((self.right_id, self.right_type.clone()))
        }
    }

    pub(crate) fn is_left_occupied(&self) -> bool {
        self.left_type != AstNodeType::None && self.left_id != Uuid::nil()
    }

    pub(crate) fn is_right_occupied(&self) -> bool {
        self.right_type != AstNodeType::None && self.right_id != Uuid::nil()
    }
}
