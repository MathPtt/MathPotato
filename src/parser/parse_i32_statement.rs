use crate::types::{
    ast_tree::MathPotatoAstTree, integer_statement_ast_node::IntegerStatementAstNode,
    potato_token::PotatoToken, potato_token_types::PotatoTokenTypes,
};

use super::parser_error::ParserError;

/// Parses an integer variable assignment.
///
/// # Example for Integer variable assignment
///
/// ```
/// Integer variable_name = 2;
/// Integer variable_name = 2 + 3;
/// Integer variable_name = (2 + 3) * 4;
/// ```
/// # Arguments
///
/// * `i` - the token position where the method starts processing the tokens
/// * `tokens` - the tokens representing the source code
/// * `ast_treez` - the AST tree
///
/// # Returns
///
/// * (usize, IntegerVariableAssignmentNode) tuple where:
/// * usize value represents the continuation position (after the `;` closing the variable
///     assignment)
/// * IntegerVariableAssignmentNode represents the Abstract Syntax Tree
pub fn parse_i32_statement(
    i: usize,
    tokens: &[PotatoToken],
) -> Result<(usize, MathPotatoAstTree), ParserError> {
    let mut integer_statement_ast_node = IntegerStatementAstNode::default();
    // The first token must be "Integer"
    match tokens
        .get(i)
        .cloned()
        .ok_or_else(|| create_error_message(i))
    {
        Ok(token) => match token.token_type {
            PotatoTokenTypes::KeywordI32 => {}
            _ => panic!(
                "The keyword is invalid, it should be {}",
                PotatoTokenTypes::KeywordI32
            ),
        },
        Err(e) => panic!("{}", e),
    };
    // the second token must be literal variable identifier
    match tokens
        .get(i + 1)
        .cloned()
        .ok_or_else(|| create_error_message(i + 1))
    {
        Ok(token) => match token.token_type {
            PotatoTokenTypes::LiteralValueVariableIdentifier => {
                integer_statement_ast_node.variable_name = token.literal_value;
            }
            _ => panic!(
                "The token is not type of: {}",
                PotatoTokenTypes::LiteralValueVariableIdentifier
            ),
        },

        Err(e) => panic!("{}", e),
    };
    // the third token must be "="
    match tokens
        .get(i + 2)
        .cloned()
        .ok_or_else(|| create_error_message(i + 2))
    {
        Ok(token) => match token.token_type {
            PotatoTokenTypes::SignAssignment => {}
            _ => panic!(
                "The token is not type of: {}",
                PotatoTokenTypes::SignAssignment
            ),
        },

        Err(e) => panic!("{}", e),
    };
    // here starts the assignment expression
    match tokens
        .get(i + 3)
        .cloned()
        .ok_or_else(|| create_error_message(i + 3))
    {
        Ok(_token) => {}

        Err(e) => panic!("{}", e),
    };

    todo!()
}

fn create_error_message(i: usize) -> String {
    format!("There is no character at the {} position.", i)
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_number() {}
}
