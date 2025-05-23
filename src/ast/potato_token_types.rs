use core::fmt;

#[derive(PartialEq, Clone, Eq, Debug, Default)]
pub enum PotatoTokenTypes {
    SignOpenParentheses,
    SignCloseParentheses,
    SignSemicolon,
    OperationAddition,
    OperationDivision,
    KeywordI32,
    SignAssignment,
    LiteralValueVariableIdentifier,
    LiteralIntegerValue,
    #[default]
    None,
}
impl fmt::Display for PotatoTokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PotatoTokenTypes::SignOpenParentheses => {
                write!(f, "PotatoTokenTypes::SignOpenParentheses")
            }
            PotatoTokenTypes::SignCloseParentheses => {
                write!(f, "PotatoTokenTypes::SignCloseParentheses")
            }
            PotatoTokenTypes::SignSemicolon => {
                write!(f, "PotatoTokenTypes::SignSemicolon")
            }
            PotatoTokenTypes::OperationAddition => write!(f, "PotatoTokenTypes::OperationAddition"),
            PotatoTokenTypes::OperationDivision => write!(f, "PotatoTokenTypes::OperationDivision"),
            PotatoTokenTypes::KeywordI32 => write!(f, "PotatoTokenTypes::KeywordI32"),
            PotatoTokenTypes::SignAssignment => write!(f, "PotatoTokenTypes::SignAssignment"),
            PotatoTokenTypes::LiteralValueVariableIdentifier => {
                write!(f, "PotatoTokenTypes::LiteralValueVariableIdentifier")
            }
            PotatoTokenTypes::LiteralIntegerValue => {
                write!(f, "PotatoTokenTypes::LiteralIntegerValue")
            }
            PotatoTokenTypes::None => {
                write!(f, "PotatoTokenTypes::None")
            }
        }
    }
}
