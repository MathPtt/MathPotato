/// This enum represents the possible values in the language.
///
/// The language manages the types in a way where all of the types have their own HashMap to be
/// stored. The reason for this is easier type management and not dealing with how to manage
/// commonalities (the same fields across types like, variable name for variable types) and
/// differences like (integer or string type values where there is difference between types). In
/// OOO language this would be done using interfaces, but Rust is a different animal.
///
/// Every AST node is aware of what is its parent and children in the tree. But these can be
/// different types, as it happens when variable declaration (name) and variable value node (the
/// value of the variable) happens. The nodes know the Uuid value of the parent and what is the
/// type of it. The type is marked with this AstNodeType. This way the interpreter will know in
/// which HashMap it should do a search for the child or the parent.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MathPotatoAstNodeType {
    I32AstNode,
    None,
}
