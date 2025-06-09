use super::MathPotatoAstTree;

pub mod add_node_to_continuation_node_right;
pub mod check_continuation_node_consistency;
pub mod create_new_infix_node_with_left_child;
pub mod create_new_infix_node_with_parent_node;
pub mod create_or_update_root_node_id_and_type;
pub mod find_next_feasible_continuation_node_and_set_as_actual_continuation_node;
pub mod get_continuation_node_id_and_type;
pub mod get_infix_node_by_id;
pub mod get_infix_node_count;
pub mod remove_continuation_node_left_node_and_return_id;
pub mod update_continuation_node_id_and_type;

pub mod i32_node;
pub mod root_node;
/// Public Abstract Syntax Tree Api
///
/// This is the layer where the so-called business logic of the tree management
/// lives. The methods of this Api takes care all of the language specific
/// logic. Another specific thing of this Api is that when something fails it
/// tries to collect all the information to debug, but it doesn't panic out.
/// Panic is pushed to the logic of the consumer (parser and interpreter) of
/// this Api.
///
/// # Abstract Syntax Tree internals
/// ## Continuation Node
///
/// Continuation node plays a role during building the tree. The parser is set
/// of methods called recursively. Every single call has to know where the
/// previous call finished processing. This information is stored in the
/// `ContinuationNode`. Obviously the picture is not this simple, but
/// finding the real continuation point is the task of the parser.
///
/// See [`ContinuationNode`](ContinuationNode) for details.
pub trait AstApi {}
impl AstApi for MathPotatoAstTree {}
