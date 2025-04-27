use core::panic;

use uuid::Uuid;

use crate::{
    lexer::lexer::lexing,
    types::{
        ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree, i32_ast_node::I32AstNode,
        infix_operation_ast_node::InfixOperationAstNode, infix_operation_enum::InfixOperationType,
        potato_token::PotatoToken, potato_token_types::PotatoTokenTypes,
    },
};

use super::parser_error::ParseError;

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
) -> Result<MathPotatoAstTree, ParseError> {
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
                            let recorded_node = ast
                                .put_i32_ast_node(I32AstNode::new_with_value(parse_literal_to_i32(
                                    &token,
                                )))
                                .unwrap_or_else(|e| {
                                    panic!("Adding I32AstNode has failed. Details: {:#?}", e)
                                });

                            let _ = ast
                                .add_root_node(recorded_node.0, AstNodeType::I32AstNode)
                                .unwrap_or_else(|r| {
                                    panic!(
                                        "Adding root node details to AST failed. Details: {:#?}",
                                        r
                                    )
                                });
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
                Some(cont_node_details) => {
                    // once we matched the actual token type
                    // we are going to match on continuation node too.

                    match token.token_type {
                        PotatoTokenTypes::LiteralIntegerValue => {
                            match cont_node_details.1 {
                                AstNodeType::I32AstNode => {
                                    // this is a syntax error, since two number type cannot follow each other
                                    Err(ParseError::new(String::from(
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
                        PotatoTokenTypes::OperationDivision => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::OperationAddition => {
                            match cont_node_details.1 {
                                AstNodeType::I32AstNode => {
                                    // this means we are at the `+` in the following example
                                    // `i32 variable_name = 1 + 2`
                                    // we are going to create a infix operation node and
                                    // the continuation node will be a child to it
                                    // and the new node will be the parent of the continuation node
                                    let mut cont_node = ast
                                        .get_i32_node(cont_node_details.0)
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "There is no i32 continuation node with {}",
                                                cont_node_details.0
                                            )
                                        });

                                    let infix_node =
                                        InfixOperationAstNode::new_with_type_and_left_child_node(
                                            InfixOperationType::Addition,
                                            cont_node_details.1,
                                            cont_node_details.0,
                                        );
                                    let recorded_infix_node =
                                        ast.put_infix_node(infix_node).unwrap_or_else(|err| {
                                            panic!(
                                                "Error while adding InfixNode to AST. Error: {:#?}",
                                                err
                                            )
                                        });
                                    cont_node.parent_type = AstNodeType::InfixOperationAstNode;
                                    cont_node.parent_uuid = recorded_infix_node.0;
                                    let _ = ast.overwrite_i32_node(
                                                        cont_node_details.0,
                                                        cont_node,
                                                    ).unwrap_or_else(|r|
                                            panic!("Updating the I32AstNode with the added InfixNode details failed. Details: {:#?}", r));
                                    let _ = ast
                                    .update_root_node_id_and_type(recorded_infix_node.0, AstNodeType::InfixOperationAstNode)
                                        .unwrap_or_else(|e|panic!("Error happened while updated root node id and type. Error: {:#?}", e));

                                    parse_i32_statement_expression(i + 1, tokens, ast)
                                }
                                AstNodeType::InfixOperationAstNode => {
                                    panic!("Syntax error! Infix operation node cannot follow another one.")
                                }
                                AstNodeType::None => {
                                    panic!("The continuation node type is None.")
                                }
                            }
                        }
                        PotatoTokenTypes::KeywordI32 => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::LiteralValueVariableIdentifier => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::SignSemicolon => Ok(ast),
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
fn value_and_infixoperation() {
    // arrange
    let input = String::from("3 +;");
    let lexed_input = lexing(&input);
    let input_ast = MathPotatoAstTree::new();

    // action
    let result = parse_i32_statement_expression(0, lexed_input, input_ast)
        .unwrap_or_else(|e| panic!("There is no result: {:#?}", e));

    // assert
    assert_eq!(result.i32_tree_len(), 1);
    assert_eq!(result.infix_tree_len(), 1);

    // continuation node checks
    let continuation_node_id_and_type = result
        .get_continuation_node_id_and_type()
        .unwrap_or_else(|| panic!("There is no continuation node!"));
    assert_eq!(
        continuation_node_id_and_type.1,
        AstNodeType::InfixOperationAstNode
    );
    let cont_node = result
        .get_infix_node(continuation_node_id_and_type.0)
        .unwrap_or_else(|| panic!("There is no continuation node by id"));
    assert_eq!(cont_node.0, continuation_node_id_and_type.0);
    assert_eq!(
        cont_node.1.get_operation_type(),
        InfixOperationType::Addition
    );

    // root node checks
    let root_node_id = result
        .get_root_node_id()
        .unwrap_or_else(|| panic!("There is no root node id!"));
    assert_eq!(
        result.get_root_node_type(),
        AstNodeType::InfixOperationAstNode
    );
    let root_node = result
        .get_infix_node(root_node_id)
        .unwrap_or_else(|| panic!("There is no root node!"));
    assert_eq!(
        root_node.1.get_operation_type(),
        InfixOperationType::Addition
    );
    let root_node_right_child_id_and_type = root_node
        .1
        .get_left_node_id_and_type()
        .unwrap_or_else(|| panic!("There is no children for root node"));
    assert_eq!(root_node_right_child_id_and_type.1, AstNodeType::I32AstNode);

    // child node
    let child_node = result
        .get_i32_node(root_node_right_child_id_and_type.0)
        .unwrap_or_else(|| {
            panic!("There is no node under the provided root node's child item node.")
        });
    assert_eq!(child_node.value, 3);
    assert_eq!(child_node.parent_type, AstNodeType::InfixOperationAstNode);
    assert_eq!(child_node.parent_uuid, root_node_id);
    assert_eq!(child_node.child_uuid, Uuid::nil());
    assert_eq!(child_node.child_type, AstNodeType::None);
}

#[test]
fn value_only() {
    // arrange
    let input = String::from("3;");
    let lexed_input = lexing(&input);
    let input_ast = MathPotatoAstTree::new();

    // action
    let result = parse_i32_statement_expression(0, lexed_input, input_ast)
        .unwrap_or_else(|r| panic!("There is no result! {:#?}", r));
    // assert
    assert_eq!(result.i32_tree_len(), 1);
    let continuation_node_id = result
        .get_continuation_node_id_and_type()
        .unwrap_or_else(|| panic!("There is no continuation node!"));
    let cont_node = result
        .get_i32_node(continuation_node_id.0)
        .unwrap_or_else(|| panic!("There is no continuation node by id"));
    let root_node_id = result
        .get_root_node_id()
        .unwrap_or_else(|| panic!("There is no root node in AST."));
    assert_eq!(root_node_id, continuation_node_id.0);
    assert_eq!(result.get_root_node_type(), AstNodeType::I32AstNode);
    assert_eq!(cont_node.value, 3);
    assert_eq!(cont_node.child_uuid, Uuid::nil());
    assert_eq!(cont_node.parent_uuid, Uuid::nil());
    assert_eq!(cont_node.parent_type, AstNodeType::None);
    assert_eq!(cont_node.child_type, AstNodeType::None);
}
