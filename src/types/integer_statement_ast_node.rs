use uuid::Uuid;

use super::{potato_token_types::PotatoTokenTypes, variable_state_enum::VariableState};

/// The `IntegerStatementAstNode` represents a code block where the result is an Integer, `i32`,
/// value and this value gets assigned to a literal, `String`, value.
///
/// # Example
///
/// In the below example the `Integer` marks that an integer value will be assigned to the `asd`
/// string literal value. The `a + b` operation is going to be executed and the resulting value
/// will be referrable through the code as `asd`.
///
/// ```
/// Integer asd = a + b;
/// ```
#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub struct IntegerStatementAstNode {
    pub guid: Uuid,
    pub variable_name: String,
    pub variable_value: i32,
    pub variable_state: VariableState,
    pub token_type: PotatoTokenTypes,
}
