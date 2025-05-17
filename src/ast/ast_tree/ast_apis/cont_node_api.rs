use crate::ast::ast_tree::MathPotatoAstTree;

pub mod get_node_id_and_type;
pub mod update_node_id_and_type;
/// The Continuation Node Api
///
/// This zero method trait is only to store all the documentation related to Continuation Node
/// problem space. All other Continuation Node related traits implement this trait,
/// so there is traceability in the documentation.
///
/// # Continuation Node
///
/// The parser goes through the tokens recursively. Every single call has to know where to continue
/// the processing. The Continuation Node is the single source of truth in this case.
pub trait ContNodeApi {}
impl ContNodeApi for MathPotatoAstTree {}
