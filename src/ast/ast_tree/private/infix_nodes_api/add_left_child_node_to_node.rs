use uuid::Uuid;

use crate::{
    ast::global::enums::ast_node_types_enum::AstNodeType, parser::parser_error::ParseError,
};

use super::{
    does_node_exist::InfixAstTreeApiGet, update::InfixAstTreeApiUpdate, InfixAstTreeApi,
    InfixNodesApi,
};

pub trait InfixAstTreeApiAddLeftChildNodeToNode: InfixAstTreeApi {
    /// Adds the `child_node_id` to the `subject_node_id` as child node to the left side.
    ///
    /// # Parameters
    /// - `target_node_id` - `Uuid`: the node where the operation will be executed, a.k.a. parent
    /// node.
    /// - `child_node_id` - `Uuid`: the node going to be a child node.
    ///
    /// # Returns
    /// - `Ok((Uuid, Uuid))` - when the oeration is executed successfully.
    /// - `Err(ParseError)` - when an error happened in the execution.
    fn add_left_child_node_to_node(
        &mut self,
        target_node_id: Uuid,
        child_node_id: Uuid,
    ) -> Result<(Uuid, Uuid), ParseError>;
}
impl InfixAstTreeApiAddLeftChildNodeToNode for InfixNodesApi {
    fn add_left_child_node_to_node(
        &mut self,
        subject_node_id: Uuid,
        child_node_id: Uuid,
    ) -> Result<(Uuid, Uuid), ParseError> {
        if self.does_node_exist(subject_node_id) != false
            && self.does_node_exist(child_node_id) != false
        {
            Err(ParseError::new(format!(
                "There is no {:#?} node with id: {}.",
                AstNodeType::InfixOperationAstNode,
                subject_node_id
            )))
        } else {
            match self.tree.get(&subject_node_id) {
                None => {
                    Err(ParseError::new(format!("Error happened retrieving parent node, id {}, after it's existence check was successful.", subject_node_id)))
                },
                Some(sn) => {
                    let mut snw = sn.clone();
                    snw.set_left_id(child_node_id);
                    snw.set_left_type(AstNodeType::InfixOperationAstNode);
                    match self.update(subject_node_id, snw) {
                        Err(e) => Err(ParseError::new(
                            format!("Error happened while writing back the modified {:#?} node with id: {}. Details: {:#?}", 
                                AstNodeType::InfixOperationAstNode, subject_node_id, e))),
                        Ok(result) => Ok((result.0, child_node_id))
                }
                }
            }
        }
    }
}
