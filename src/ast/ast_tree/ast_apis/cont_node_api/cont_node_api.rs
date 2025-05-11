use crate::ast::ast_tree::MathPotatoAstTree;

/// The Continuation Node Api
///
/// # Note
///
/// This zero method trait is only to store all the documentation related to Continuation Node
/// problem space. All the related traits implement this trait, so there is traceability in the
/// documentation.
///
/// # Continuation Node
///
/// The parser goes through the tokens recursively. Every single call has to know where to continue
/// the processing. The Continuation Node is the single source of truth in this case.
pub trait ContNodeApi {}
impl ContNodeApi for MathPotatoAstTree {}
