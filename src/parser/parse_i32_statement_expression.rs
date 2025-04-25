use core::panic;

use uuid::Uuid;

use crate::{
    lexer::lexer::lexing,
    types::{
        i32_ast_node::I32AstNode, infix_operation_ast_node::InfixOperationAstNode,
        math_potato_ast_node_types_enum::AstNodeType, math_potato_ast_tree::MathPotatoAstTree,
        potato_token::PotatoToken, potato_token_types::PotatoTokenTypes,
    },
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
            match ast.get_continuation_node_id_and_type() {
                // this means that we are processing the first character!
                None => {
                    match token.token_type {
                        PotatoTokenTypes::LiteralIntegerValue => {
                            // this is the case when we right after the `=` sign and the expression
                            // tree is empty
                            ast.put_i32_ast_node(I32AstNode::new_with_value(parse_literal_to_i32(
                                &token,
                            )))
                            .unwrap_or_else(|_| panic!("Cannot add i32 ast node to AST tree"));
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::SignAssignment => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::OperationDivision => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::OperationAddition => {
                            panic!(
                                "Syntax error! The first character of an expression cannot be +!"
                            )
                        }
                        PotatoTokenTypes::KeywordI32 => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
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
                Some(cont_node) => {
                    // once we matched the actual token type
                    // we are going to match on continuation node too.

                    match token.token_type {
                        PotatoTokenTypes::LiteralIntegerValue => {
                            match cont_node.1 {
                                AstNodeType::I32AstNode => {
                                    // this is a syntax error, since two number type cannot follow each other
                                    Err(MathPotatoParserError::new(String::from(
                                        "Syntax error! Two number type cannot follow each other!",
                                    )))
                                }
                                AstNodeType::InfixOperationAstNode => {
                                    // this happens when we are right after the `+` sign in the
                                    // following example:
                                    // `i32 variable_name = 1 + 3;`
                                    panic!("Deal with it later.")
                                }
                                AstNodeType::None => {
                                    panic!("we have a cont node, but the type is none")
                                }
                            }
                        }
                        PotatoTokenTypes::SignAssignment => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::SignCloseParentheses => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::SignOpenParentheses => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::SignSemicolon => Ok(ast),
                        PotatoTokenTypes::OperationDivision => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::OperationAddition => {
                            match cont_node.1 {
                                AstNodeType::I32AstNode => {
                                    // this means we are at the `+` in the following example
                                    // `i32 variable_name = 1 + 2`
                                    // we are going to create a infix operation node and
                                    // the continuation node will be a child to it
                                    // and the new node will be the parent of the continuation node
                                    match ast.get_i32_node(cont_node.1) {
                                        None => panic!(
                                            "There is no i32 continuation node with {}",
                                            cont_node.0
                                        ),
                                        Some(n) => {
                                            let inode = InfixOperationAstNode::new_with_type_and_left_child_node(
                                                AstNodeType::InfixOperationAstNode,
                                                cont_node.1,
                                                cont_node.0
                                            );
                                        }
                                    }
                                }
                                AstNodeType::InfixOperationAstNode => {
                                    panic!("Syntax error! Infix operation node cannot follow another one.")
                                }
                                AstNodeType::None => {
                                    panic!("The continuation node type is None.")
                                }
                            }

                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::KeywordI32 => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::LiteralValueVariableIdentifier => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::None => todo!(),
                    }
                }
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

#[test]
fn single_value_then_return_single_node_in_ast() {
    // arrange
    let input = String::from("3;");
    let lexed_input = lexing(&input);
    let input_ast = MathPotatoAstTree::new();

    // action
    match parse_i32_statement_expression(0, lexed_input, input_ast) {
        // assert
        Err(r) => {
            panic!("There is no result! {:#?}", r)
        }
        Ok(r) => {
            assert_eq!(r.i32_tree_len(), 1);
            let continuation_node_id = r
                .get_continuation_node_id_and_type()
                .unwrap_or_else(|| panic!("There is no continuation node!"));
            let cont_node = r
                .get_i32_node(continuation_node_id.0)
                .unwrap_or_else(|| panic!("There is no continuation node by id"));
            println!("Continuation node: {}", continuation_node_id.0);
            println!("Requested node: {:#?}", cont_node.clone());
            assert_eq!(cont_node.value, 3);
            assert_eq!(cont_node.child_uuid, Uuid::nil());
            assert_eq!(cont_node.parent_uuid, Uuid::nil());
            assert_eq!(cont_node.parent_type, AstNodeType::None);
            assert_eq!(cont_node.child_type, AstNodeType::None);
        }
    }
}
