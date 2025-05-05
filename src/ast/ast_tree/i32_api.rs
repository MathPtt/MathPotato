use super::MathPotatoAstTree;

/// The I32 Api
///
/// The I32 Api is dedicated to communicate with world outside of the AST.
///
/// # Note
///
/// This trait exist to store the Api level documentation. All I32 Api traits implements this
/// trait, so in the documentation there is a trace back to the top abstraction when it comes to
/// the I32 Api of the Abstract Syntax Tree.
///
/// # The i32 Api
///
/// The abstract syntax tree in this programming language has two specialities:
/// - the AST is not a tree, rather a HashMap for every type where the tree sturcture is
/// constructed using the id values of the nodes
/// - there is no generics in the AST. Everything is typed, and due to this we have the same
/// methods with different types, and this is the reason we have the id_and_type methods to decide
/// what type specific method should we call.
///
/// The second reason is behind the fact that we have a type specific Api, like this one: i32 Api.
/// Using this Api it is possible to manage the nodes represent i32 type things.
///
/// The i32 type is equvalent to Rust's i32 type. The interpreter will do i32 things when sees
/// these types.
pub trait I32Api {}
impl I32Api for MathPotatoAstTree {}
