use uuid::Uuid;

use crate::{
    ast::ast_tree::{
        MathPotatoAstTree,
        private::continuation_node::storage::get_id::ContinuationNodeStorageApiGetId,
        public::find_next_feasible_continuation_node_and_set_as_actual_continuation_node::FindNextFeasibleContinuationNode,
    },
    parser::parser_error::ParseError,
};

impl FindNextFeasibleContinuationNode for MathPotatoAstTree {
    fn find_next_feasible_continuation_node_and_set_as_actual_continuation_node(
        &self,
    ) -> Result<Uuid, ParseError> {
        Ok(self.continuation_node.get_id())
    }
}
