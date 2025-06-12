use uuid::Uuid;

use crate::{ast::ast_tree::MathPotatoAstTree, parser::parser_error::ParseError};

use super::ContNodeApiUpdateNodeIdAndType;

impl ContNodeApiUpdateNodeIdAndType for MathPotatoAstTree {
    fn continuation_node_api_update_node_id_and_type(
        &mut self,
        id: Uuid,
        ast_node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError> {
        match self
            .continuation_node
            .set_id_and_type(id, ast_node_type.clone())
        {
            Err(e) => Err(ParseError::new(format!(
                "Error happened while updating {:#?}. Input: id: {}, type: {:#?}. Details: {:#?}",
                self.continuation_node, id, ast_node_type, e
            ))),
            Ok(res) => Ok(res),
        }
    }
}
