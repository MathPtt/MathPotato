use core::fmt;

use derive_more::Display;

#[derive(PartialEq, Clone, Eq, Debug, Default, Display)]
#[display("PotatoTokenTypes({_0}", _0)]
pub enum PotatoTokenTypes {
    #[display("SignOpenParentheses")]
    SignOpenParentheses,

    #[display("SignCloseParentheses")]
    SignCloseParentheses,

    #[display("SignSemicolon")]
    SignSemicolon,

    #[display("OperationAddition")]
    OperationAddition,

    #[display("OperationDivision")]
    OperationDivision,

    #[display("OperationMultiplication")]
    OperationMultiplication,

    #[display("KeywordI32")]
    KeywordI32,

    #[display("SignAssignment")]
    SignAssignment,

    #[display("LiteralValueVariableIdentifier")]
    LiteralValueVariableIdentifier,

    #[display("LiteralIntegerValue")]
    LiteralIntegerValue,

    #[display("None")]
    #[default]
    None,
}
