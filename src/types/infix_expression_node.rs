use uuid::Uuid;

use super::potato_token_types::PotatoTokenTypes;

/// It represents an infix operator in the Abstract Syntax Tree.
///
/// # Example
/// ```
/// a + b
/// ```
/// where the `+` is the infix operator
#[derive(PartialEq, Debug)]
pub struct InfixExpressionAstNode {
    pub token_type: PotatoTokenTypes,
    pub left: Uuid,
    pub right: Uuid,
}

impl InfixExpressionAstNode {
    pub fn new(token_type: PotatoTokenTypes, left: Uuid, right: Uuid) -> InfixExpressionAstNode {
        InfixExpressionAstNode {
            token_type,
            left,
            right,
        }
    }
    pub fn new_with_token_type(token_type: PotatoTokenTypes) -> InfixExpressionAstNode {
        InfixExpressionAstNode {
            token_type,
            left: Uuid::nil(),
            right: Uuid::nil(),
        }
    }
    pub fn new_with_left(token_type: PotatoTokenTypes, left: Uuid) -> InfixExpressionAstNode {
        InfixExpressionAstNode {
            token_type,
            left,
            right: Uuid::nil(),
        }
    }
    pub fn new_with_right(token_type: PotatoTokenTypes, right: Uuid) -> InfixExpressionAstNode {
        InfixExpressionAstNode {
            token_type,
            left: Uuid::nil(),
            right,
        }
    }
}
