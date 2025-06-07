use super::infix_operation_type_enum::InfixOperationTypeEnum;
use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiSetInfixNodeOperationType: InfixNodeApi {
    fn set_infix_node_operation_type(&mut self, infix_node_operation_type: InfixOperationTypeEnum);
}

impl InfixNodeApiSetInfixNodeOperationType for InfixNode {
    fn set_infix_node_operation_type(&mut self, infix_node_operation_type: InfixOperationTypeEnum) {
        self.operation_type = infix_node_operation_type;
    }
}
