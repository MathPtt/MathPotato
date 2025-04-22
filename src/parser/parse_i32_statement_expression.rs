use crate::types::{
    ast_tree::PotatoAstTree, integer_value_expression_ast_node::IntegerValueExpressionAstNode,
    potato_ast_node::PotatoAstNode, potato_token::PotatoToken,
    potato_token_types::PotatoTokenTypes,
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
    mut ast: PotatoAstTree,
) -> Result<PotatoAstTree, MathPotatoParserError> {
    match tokens.get(i).cloned().ok_or_else(|| error_message(i)) {
        Err(e) => panic!("{}", e),

        Ok(token) => {
            let last_added_node = ast.get_last_modified_clone().unwrap_or(PotatoAstNode::None);
            match token.token_type {
                    PotatoTokenTypes::LiteralIntegerValue => match last_added_node {
                        // happens when the integer value is right after the `=` sign,
                        // like `= 1 + 2` and we process `1`
                        PotatoAstNode::None => {
                        let integer_value_expression_node = PotatoAstNode::IntegerValueExpressionAstNode(
                            IntegerValueExpressionAstNode::new_with_value(parse_literal_to_i32(&token)));
                        let Ok(_) = ast.add_node(integer_value_expression_node) else {
                            panic!("Adding node to AST tree has failed")
                        };
                        match parse_i32_statement_expression(i + 1, tokens, ast) {
                                Ok(r) => Ok(r),
                                Err(err) => {
                                    panic!("Error: {:#?}", err);
                                }
                            }
                        },
                        PotatoAstNode::IntegerValueExpressionAstNode(_) =>
                            Err(MathPotatoParserError::new(
                                String::from("A value, like Integer value, cannot follow directly another value, like Integer."))),
                        PotatoAstNode::IntegerStatementAstNode(_) =>
                            Err(MathPotatoParserError::new(
                                String::from("Syntax error. The `Integer` string is present between `Integer` and `;`, and this is not allowed"))),
                    },
                    PotatoTokenTypes::SignSemicolon => Ok(ast),
                    _ => panic!("Case of {} is not covered.", token.token_type),
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
#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::{
        lexer::lexer::lexing,
        types::{
            ast_tree::PotatoAstTree,
            integer_value_expression_ast_node::IntegerValueExpressionAstNode,
            potato_ast_node::{PotatoAstNode, PotatoAstNodeKind},
        },
    };

    use super::parse_i32_statement_expression;

    #[test]
    fn single_integer_case() {
        let input_tokens = lexing("1;");
        let result_ast = parse_i32_statement_expression(0, input_tokens, PotatoAstTree::new());
        let mut expected_ast = PotatoAstTree::new();
        let node_1_guid = Uuid::new_v4();
        let node_1 = PotatoAstNode::IntegerValueExpressionAstNode(
            IntegerValueExpressionAstNode::new_with_guid_and_value(node_1_guid, 1),
        );
        expected_ast.add_node(node_1).expect("Failed to add node");
        match result_ast {
            Err(error) => panic!("{:#?}", error),
            Ok(result_ast_ok) => {
                assert_eq!(result_ast_ok.len(), 1);
                let expected_node = expected_ast
                    .get(
                        node_1_guid,
                        PotatoAstNodeKind::IntegerValueExpressionAstNode,
                    )
                    .unwrap_or_else(|| {
                        panic!(
                            "{}",
                            format!("No elem in expected result with id: {}", node_1_guid)
                        )
                    });
                let result_node = result_ast_ok
                    .get(
                        node_1_guid,
                        PotatoAstNodeKind::IntegerValueExpressionAstNode,
                    )
                    .unwrap_or_else(|| {
                        panic!(
                            "{}",
                            format!("No elem in test result with id: {}", node_1_guid)
                        )
                    });
            }
        }
    }

    #[test]
    fn integer_addition() {
        let input = lexing("1 + 2;");
    }
}
