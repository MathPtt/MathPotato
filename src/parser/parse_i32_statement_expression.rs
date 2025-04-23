use crate::types::{
    i32_ast_node::I32AstNode, integer_value_expression_ast_node::IntegerValueExpressionAstNode,
    math_potato_ast_node_types_enum::MathPotatoAstNodeType,
    math_potato_ast_tree::MathPotatoAstTree, potato_ast_node::PotatoAstNode,
    potato_token::PotatoToken, potato_token_types::PotatoTokenTypes,
};

use super::parser_error::MathPotatoParserError;

/// Parses integer, i32, statement expression based on the tokens provided by the lexer.
///
/// # Parameters
///
/// * `i` - start character of processing
/// * `tokens` - PotatoTokens
/// * `nodes` - Abstract Syntax Tree
///
/// # Returns
///
/// Abstract Syntax Tree, `VariableValueExpressionNode` representing the processed tokens.
///
/// # Examples
///
/// The method processes whatever is between `=` and `;`;
/// ```
/// Integer asd = 5;
/// ```
pub fn parse_i32_statement_expression(
    i: usize,
    tokens: Vec<PotatoToken>,
    mut ast: MathPotatoAstTree,
) -> Result<MathPotatoAstTree, MathPotatoParserError> {
    match tokens.get(i).cloned().ok_or_else(|| error_message(i)) {
        Err(e) => panic!("{}", e),

        Ok(token) => {
            let continuation_node_details = ast.get_continuation_node_id_and_type().unwrap();
            match token.token_type {
                PotatoTokenTypes::LiteralIntegerValue => match continuation_node_details.1 {
                    MathPotatoAstNodeType::I32AstNode => {
                        // this is a syntax error, since two number type cannot follow each other
                        Err(MathPotatoParserError::new(String::from(
                            "Syntax error! Two number type cannot follow each other!",
                        )))
                    }
                    MathPotatoAstNodeType::None => {
                        // this is the case when we right after the `=` sign and the expression
                        // tree is empty
                        ast.put_i32_ast_node(I32AstNode::new_with_value(parse_literal_to_i32(
                            &token,
                        )));
                        parse_i32_statement_expression(i + 1, tokens, ast)
                    }
                },
                PotatoTokenTypes::SignAssignment => {
                    parse_i32_statement_expression(i + 1, tokens, ast)
                }
                PotatoTokenTypes::OperationDivision => {
                    parse_i32_statement_expression(i + 1, tokens, ast)
                }
                PotatoTokenTypes::OperationAddition => {
                    parse_i32_statement_expression(i + 1, tokens, ast)
                }
                PotatoTokenTypes::KeywordI32 => parse_i32_statement_expression(i + 1, tokens, ast),
                PotatoTokenTypes::SignCloseParentheses => {
                    parse_i32_statement_expression(i + 1, tokens, ast)
                }
                PotatoTokenTypes::LiteralValueVariableIdentifier => {
                    parse_i32_statement_expression(i + 1, tokens, ast)
                }
                PotatoTokenTypes::SignOpenParentheses => {
                    parse_i32_statement_expression(i + 1, tokens, ast)
                }
                PotatoTokenTypes::SignSemicolon => Ok(ast),
                PotatoTokenTypes::None => todo!(),
            }
        }
    }
}

fn error_message(i: usize) -> String {
    format!("There is no character at {}", i)
}

fn parse_literal_to_i32(t: &PotatoToken) -> i32 {
    let result: Result<i32, _> = t.literal_value.parse();
    match result {
        Ok(v) => v,
        Err(e) => {
            panic!(
                "Error while parsing literal value to i32. Error message is {}",
                e
            );
        }
    }
}
