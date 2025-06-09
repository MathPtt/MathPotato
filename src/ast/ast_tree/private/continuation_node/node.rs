use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod get_id;
pub mod get_type;
pub mod new;
pub mod new_from_id_and_type;
pub mod set_id;
pub mod set_type;

/// Continuation Node
///
/// This node has an `id` and `node_type` value and these represents
/// continuation information in the processing of tokens. The parser is
/// recursive method and every single call has to know where the previous one
/// finished processing. This information is stored here.
#[derive(Clone, Debug, Display)]
#[display("ContinuationNode(id: {}, node_type: {}", id, node_type)]
pub struct ContinuationNode {
    id: Uuid,
    node_type: AstNodeType,
}

/// Continuation Node Api
///
/// This Api provides methods to access the data stored in the node.
pub trait ContinuationNodeApi {}
impl ContinuationNodeApi for ContinuationNode {}
