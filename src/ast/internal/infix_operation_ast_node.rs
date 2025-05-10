use crate::ast::ast_node_types_enum::AstNodeType;
use crate::ast::infix_operation_type_enum::InfixOperationTypeEnum;
use crate::parser::parser_error::ParseError;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InfixAstNodeInternal {
    operation_type: InfixOperationTypeEnum,
    left_id: Uuid,
    left_type: AstNodeType,
    right_id: Uuid,
    right_type: AstNodeType,
}

impl InfixAstNodeInternal {
    pub fn new_with_type_and_left_child_node(
        operation_type: InfixOperationTypeEnum,
        left_type: AstNodeType,
        left: Uuid,
    ) -> InfixAstNodeInternal {
        InfixAstNodeInternal {
            operation_type,
            left_type,
            left_id: left,
            right_id: Uuid::nil(),
            right_type: AstNodeType::None,
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

    /// Checks if the InfixOperationAstNode node is in the right state.
    ///
    /// # Concept
    ///
    /// An InfixOperationAstNode left and right side is occupied in the following order:
    /// - Left side (since we read from the left to right)
    /// - Right side
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

    pub(crate) fn is_left_occupied(&self) -> bool {
        self.left_type != AstNodeType::None && self.left_id != Uuid::nil()
    }

    pub(crate) fn is_right_occupied(&self) -> bool {
        self.right_type != AstNodeType::None && self.right_id != Uuid::nil()
    }

    pub(crate) fn add_i32node_to_the_right(
        &mut self,
        id: Uuid,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
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
