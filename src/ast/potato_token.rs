use super::potato_token_types::PotatoTokenTypes;

/// Represents a single character in the parsed source code.
/// Every single character, keyword and literal value gets its own Token.
#[derive(Clone, Debug)]
pub struct PotatoToken {
    /// The token type
    pub token_type: PotatoTokenTypes,
    /// The literal value of the token.
    /// It plays a crucial role when variable names are determined.
    /// The variable name will be placed here during lexing.
    pub literal_value: String,
}
impl PartialEq for PotatoToken {
    fn eq(&self, other: &PotatoToken) -> bool {
        if self.token_type == other.token_type && self.literal_value == other.literal_value {
            return true;
        }

        false
    }
}
