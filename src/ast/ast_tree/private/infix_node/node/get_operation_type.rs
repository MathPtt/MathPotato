use super::InfixNode;
use super::InfixNodeApi;
use super::infix_operation_type_enum::InfixOperationTypeEnum;

pub trait InfixNodeApiGetOperationType: InfixNodeApi {
    /// Returns a reference of the operation type of the infix node.
    ///
    /// # Returns
    /// - `&InfixOperationTypeEnum` - the enum value set up for the node;
    fn get_infix_node_operation_type(&self) -> &InfixOperationTypeEnum;
}
impl InfixNodeApiGetOperationType for InfixNode {
    fn get_infix_node_operation_type(&self) -> &InfixOperationTypeEnum {
        &self.operation_type
    }
}
