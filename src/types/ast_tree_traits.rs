use uuid::Uuid;

/// Defines that every typed AST tree has to have a size method.
pub trait TypedAstTreeLen {
    fn len(&self) -> usize;
}

/// Defines that ever typed AST tree has to have a method returning the list of keys of the inner
/// HashMap.
pub trait TypedAstTreeGetKeys {
    fn keys(self) -> Vec<Uuid>;
}
