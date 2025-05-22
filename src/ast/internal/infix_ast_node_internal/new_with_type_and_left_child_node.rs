use uuid::Uuid;

use crate::ast::{
    ast_node_types_enum::AstNodeType, infix_operation_type_enum::InfixOperationTypeEnum,
};

use super::{InfixAstNodeInternal, InfixAstNodeInternalApi};

pub trait InfixAstNodeInternalNewWithTypeAndLeftChildNode: InfixAstNodeInternalApi {
    fn new_with_type_and_left_child_node(
        operation_type: InfixOperationTypeEnum,
        left_type: AstNodeType,
        left: Uuid,
    ) -> InfixAstNodeInternal;
}
impl InfixAstNodeInternalNewWithTypeAndLeftChildNode for InfixAstNodeInternal {
    fn new_with_type_and_left_child_node(
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
}
