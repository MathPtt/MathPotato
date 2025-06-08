use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

#[derive(Clone, Debug, Display)]
#[display("RootNodeDto(id: {}, type: {})", id, node_type)]
pub struct RootNodeDto {
    id: Uuid,
    node_type: AstNodeType,
}

impl RootNodeDto {
    pub fn new(id: Uuid, node_type: AstNodeType) -> Self {
        Self { id, node_type }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn node_type(&self) -> &AstNodeType {
        &self.node_type
    }

    pub fn set_node_type(&mut self, node_type: AstNodeType) {
        self.node_type = node_type;
    }
}
