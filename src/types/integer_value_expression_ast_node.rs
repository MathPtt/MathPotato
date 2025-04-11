use uuid::Uuid;

/// Represents an Integer, `i32`, value in a tree of expressions. The expression tree, an Abstract
/// Syntax Tree, represents operations and values needed to be executed in order to get a final
/// result which will be the value of an Integer type variable.
///
/// The `expression_tree` is the Abstract Syntax Tree representing flow of operations.
///
/// # Examples
///
/// ```
/// Integer var_name = 1;
/// Integer var_name2 = 1 + 2;
/// Integer var_name3 = (1 + 2 - 12) * 4;
/// ```
#[derive(PartialEq, Debug, Default, Clone)]
pub struct IntegerValueExpressionAstNode {
    pub guid: Uuid,
    pub value: i32,
    pub parent: Uuid,
}

impl IntegerValueExpressionAstNode {
    pub fn new_with_value_and_parent(value: i32, parent: Uuid) -> IntegerValueExpressionAstNode {
        let guid = Uuid::new_v4();
        IntegerValueExpressionAstNode {
            guid,
            value,
            parent,
        }
    }
    pub fn new_with_value(value: i32) -> IntegerValueExpressionAstNode {
        let guid = Uuid::new_v4();
        IntegerValueExpressionAstNode {
            guid,
            value,
            parent: Uuid::nil(),
        }
    }
    pub fn new_with_guid_and_value(guid: Uuid, value: i32) -> IntegerValueExpressionAstNode {
        IntegerValueExpressionAstNode {
            guid,
            value,
            parent: Uuid::nil(),
        }
    }
}
