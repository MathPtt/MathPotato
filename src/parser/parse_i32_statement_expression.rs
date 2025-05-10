use core::panic;

use crate::ast::{
    ast_node_types_enum::AstNodeType,
    ast_tree::{
        cont_node_api_get_id_and_type::ContNodeApiGetIdAndType,
        i32_api_get_node_by_id::I32ApiGetNodeById, i32_api_put_node::I32ApiPutNode,
        i32_api_update_node::I32ApiUpdateNode, root_node_api::RootNodeApi, MathPotatoAstTree,
    },
    i32_node::I32AstNode,
    infix_operation_type_enum::InfixOperationTypeEnum,
    internal::infix_operation_ast_node::InfixAstNodeInternal,
    potato_token::PotatoToken,
    potato_token_types::PotatoTokenTypes,
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
/// i32 asd = 5;
/// i32 foo = 4 + 4;
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
                                .add_root_node_id_and_type(
                                    recorded_node.id,
                                    AstNodeType::I32AstNode,
                                )
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
                        PotatoTokenTypes::OperationMultiplication => {
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
                                    let mut cont_node = ast
                                        .get_infix_node_by_id(cont_node_details.0)
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "No InfixOperationAstNode with id {:#?}",
                                                cont_node_details.0
                                            )
                                        });
                                    cont_node
                                        .1
                                        .check_if_left_empty_right_occupied()
                                        .unwrap_or_else(|e| panic!("{:#?}", e));
                                    let i32node = I32AstNode::new_value_parent_id_and_type(
                                        parse_literal_to_i32(&token),
                                        AstNodeType::InfixOperationAstNode,
                                        cont_node.0,
                                    );
                                    let i32node_recorded = ast
                                        .put_i32_ast_node(i32node)
                                        .unwrap_or_else(|e| panic!("{:#?}", e));
                                    cont_node
                                        .1
                                        .add_i32node_to_the_right(i32node_recorded.id)
                                        .unwrap_or_else(|e| panic!("{:#?}", e));
                                    ast.update_infix_node_by_id(cont_node.0, cont_node.1).unwrap_or_else(|e|panic!("Error happened while persisting updated InfixOperationAstNode node. {:#?}", e));
                                    parse_i32_statement_expression(i + 1, tokens, ast)
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
                        PotatoTokenTypes::OperationMultiplication => {
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        PotatoTokenTypes::OperationMultiplication => {
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
                                        .get_i32_node_by_id(cont_node_details.0)
                                        .unwrap_or_else(|e| {
                                            panic!(
                                                "There is no i32 continuation node with {}. Error: {:#?}",
                                                cont_node_details.0,
                                                e
                                            )
                                        });

                                    let infix_node =
                                        InfixAstNodeInternal::new_with_type_and_left_child_node(
                                            InfixOperationTypeEnum::Addition,
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
                                    cont_node.parent_id = recorded_infix_node.0;
                                    let _ = ast.update_i32_node(
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

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::ast::ast_tree::cont_node_api_get_id_and_type::ContNodeApiGetIdAndType;
    use crate::ast::ast_tree::i32_api_get_node_by_id::I32ApiGetNodeById;
    use crate::ast::ast_tree::i32_api_node_count::I32ApiNodeCount;
    use crate::ast::infix_operation_type_enum::InfixOperationTypeEnum;
    use crate::{
        ast::{
            ast_node_types_enum::AstNodeType,
            ast_tree::{root_node_api::RootNodeApi, MathPotatoAstTree},
        },
        lexer::lexer::lexing,
        parser::parse_i32_statement_expression::parse_i32_statement_expression,
    };

    #[test]
    fn addition_and_multiplication_precedence_case() {
        // case: `2 + 3 * 4;`
        // arrange
        let input = String::from("1 + 2;");
        let lexed_input = lexing(&input);
        let input_ast = MathPotatoAstTree::new();
    }

    #[test]
    fn value_infix_and_value() {
        // case: `3 + 3;`
        // arrange
        let input = String::from("1 + 2;");
        let lexed_input = lexing(&input);
        let input_ast = MathPotatoAstTree::new();

        // action
        let result = parse_i32_statement_expression(0, lexed_input, input_ast)
            .unwrap_or_else(|e| panic!("There is no result: {:#?}", e));

        // assert
        assert_eq!(
            result.i32_node_count(),
            2,
            "sout: {}, expected: {}",
            result.i32_node_count(),
            2
        );
        assert_eq!(
            result.infix_tree_len(),
            1,
            "sout: {}, expected: {}",
            result.infix_tree_len(),
            1
        );

        // continuation node checks
        let continuation_node_id_and_type = result
            .get_continuation_node_id_and_type()
            .unwrap_or_else(|| panic!("There is no continuation node!"));
        assert_eq!(
            continuation_node_id_and_type.1,
            AstNodeType::I32AstNode,
            "The continuation node has to be an I32AstNode"
        );
        let _ = result
            .get_i32_node_by_id(continuation_node_id_and_type.0)
            .unwrap_or_else(|e| panic!("There is no i32 continuation node by id. Error: {:#?}", e));

        // root node checks
        let root_node_id = result
            .get_root_node_id()
            .unwrap_or_else(|| panic!("There is no root node id!"));
        assert_eq!(
            result.get_root_node_type(),
            AstNodeType::InfixOperationAstNode,
            "Root node must be: {:#?}, but it was {:#?}",
            AstNodeType::InfixOperationAstNode,
            result.get_root_node_type()
        );
        let root_node = result
            .get_root_node_infix(root_node_id)
            .unwrap_or_else(|e| panic!("{:#?}", e));
        assert!(
            root_node.1.is_left_occupied(),
            "The left node of the root InfixOperationAstNode must be occupied at this point.",
        );
        assert!(
            root_node.1.is_right_occupied(),
            "The left node of the root InfixOperationAstNode must be occupied at this point.",
        );
        let left_children_id_and_type = root_node
            .1
            .get_left_node_id_and_type()
            .unwrap_or_else(|| panic!("There is no left children."));
        assert_eq!(
            left_children_id_and_type.1,
            AstNodeType::I32AstNode,
            "Children node has to be {:#?}",
            AstNodeType::I32AstNode
        );
        let left_children = result
            .get_i32_node_by_id(left_children_id_and_type.0)
            .unwrap_or_else(|e| panic!("No left children. Error: {:#?}", e));
        assert_eq!(left_children.value, 1, "The left children value must be 1");
        assert_eq!(
            left_children.parent_id, root_node.0,
            "The parent id must be equal to root node id."
        );
        assert_eq!(
            left_children.parent_type,
            result.get_root_node_type(),
            "The parent type must be equal to root node type."
        );

        let right_children_id_and_type = root_node
            .1
            .get_right_node_id_and_type()
            .unwrap_or_else(|| panic!("There is no left children."));
        assert_eq!(
            right_children_id_and_type.1,
            AstNodeType::I32AstNode,
            "Children node has to be {:#?}",
            AstNodeType::I32AstNode
        );
        let right_children = result
            .get_i32_node_by_id(right_children_id_and_type.0)
            .unwrap_or_else(|e| panic!("No right children. Error: {:#?}", e));
        assert_eq!(
            right_children.value, 2,
            "The right children value must be 2"
        );
        assert_eq!(
            right_children.parent_id, root_node.0,
            "The parent id must be equal to root node id."
        );
        assert_eq!(
            right_children.parent_type,
            result.get_root_node_type(),
            "The parent type must be equal to root node type."
        );
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
        assert_eq!(result.i32_node_count(), 1);
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
            .get_infix_node_by_id(continuation_node_id_and_type.0)
            .unwrap_or_else(|| panic!("There is no continuation node by id"));
        assert_eq!(cont_node.0, continuation_node_id_and_type.0);
        assert_eq!(
            cont_node.1.get_operation_type(),
            InfixOperationTypeEnum::Addition
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
            .get_infix_node_by_id(root_node_id)
            .unwrap_or_else(|| panic!("There is no root node!"));
        assert_eq!(
            root_node.1.get_operation_type(),
            InfixOperationTypeEnum::Addition
        );
        let root_node_right_child_id_and_type = root_node
            .1
            .get_left_node_id_and_type()
            .unwrap_or_else(|| panic!("There is no children for root node"));
        assert_eq!(root_node_right_child_id_and_type.1, AstNodeType::I32AstNode);

        // child node
        let child_node = result
            .get_i32_node_by_id(root_node_right_child_id_and_type.0)
            .unwrap_or_else(|e| {
                panic!(
                    "There is no node under the provided root node's child item node. Error: {:#?}",
                    e
                )
            });
        assert_eq!(child_node.value, 3);
        assert_eq!(child_node.parent_type, AstNodeType::InfixOperationAstNode);
        assert_eq!(child_node.parent_id, root_node_id);
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
        assert_eq!(result.i32_node_count(), 1);
        let continuation_node_id = result
            .get_continuation_node_id_and_type()
            .unwrap_or_else(|| panic!("There is no continuation node!"));
        let cont_node = result
            .get_i32_node_by_id(continuation_node_id.0)
            .unwrap_or_else(|e| panic!("There is no continuation node by id. Error: {:#?}", e));
        let root_node_id = result
            .get_root_node_id()
            .unwrap_or_else(|| panic!("There is no root node in AST."));
        assert_eq!(root_node_id, continuation_node_id.0);
        assert_eq!(result.get_root_node_type(), AstNodeType::I32AstNode);
        assert_eq!(cont_node.value, 3);
        assert_eq!(cont_node.parent_id, Uuid::nil());
        assert_eq!(cont_node.parent_type, AstNodeType::None);
    }
}
