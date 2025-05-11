use crate::ast::ast_tree::MathPotatoAstTree;

pub mod add_root_node_id_and_type;
pub mod get_root_node_id;
pub mod get_root_node_type;
pub mod node_api_get_infix_by_id;
pub mod update_root_node_id_and_type;

/// Root Node Api
///
/// The purpose of this Api is to provide intuitive methods to manage the Root Node in the Abstract
/// Syntax Tree.
///
/// # The concept of Root Node
///
/// The parser builds the Abstract Syntax Tree and the interpreter walks it. The Interpreter needs
/// a point to start: the Root Node.
///
/// # Workflows related to Root Node
///
/// The Parser is responsible to maintain which node is the actual Root Node. So that, the Parser
/// adds and modifies the Root Node value. And the Parser requests actual information about the
/// Root Node.
pub trait RootNodeApi {}
impl RootNodeApi for MathPotatoAstTree {}
