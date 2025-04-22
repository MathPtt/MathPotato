use std::collections::HashMap;

use uuid::Uuid;

use super::i32_ast_node::I32AstNode;

/// Defines that every typed AST tree has to have a size method.
pub trait TypedAstTreeGetSize {
    fn size(self) -> usize;
}

/// Defines that ever typed AST tree has to have a method returning the list of keys of the inner
/// HashMap.
pub trait TypedAstTreeGetKeys {
    fn keys(self) -> Vec<Uuid>;
}
