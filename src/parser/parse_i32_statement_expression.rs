use core::panic;
use std::any::type_name;

use crate::ast::ast_tree::{global::enums::{potato_token::PotatoToken, potato_token_types::PotatoTokenTypes}, public::{create_i32_node_with_value::{CreateI32NodeWithValue, I32ApiCreateNodeWithValue}, get_continuation_node_id_and_type::ContinuationNodeStorageApiGetIdAndType}, MathPotatoAstTree};

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
            // match ast.cont_node_api_get_cont_node_id_and_type() {
                // this means that we are processing the first character!
                None => {
                    match token.token_type {
                        PotatoTokenTypes::LiteralIntegerValue => {
                            // this is the case when we right after the `=` sign and the expression
                            // tree is empty
                            let recorded_node_id = ast.i32_api_create_node_with_value(parse_literal_to_i32(&token))
                                .unwrap_or_else(|e|panic!("Creating new {:#?} node failed. Details: {:#?}", AstNodeType::I32AstNode, e));
                            let _ = ast.root_node_api_update_root_node_id_and_type(
                                recorded_node_id, 
                                ast.node_catalog_api_get_node_type_by_id(recorded_node_id)?
                            ).unwrap_or_else(|e|panic!("Updating root node id and type failed for node type: {:#?} with id: {}. Details: {:#?}", 
                                    AstNodeType::I32AstNode,
                                    recorded_node_id,
                                    e
                                ));
                            let _ = ast.continuation_node_api_update_node_id_and_type(
                                recorded_node_id,
                                AstNodeType::I32AstNode,
                            );
                            // println!("=== first char LiteralIntegerValue: {:#?}", ast);
                            parse_i32_statement_expression(i + 1, tokens, ast)
                        }
                        _ => {
                            panic!("The first character right after the `=` was {:#?} than the expected: {:#?}.", token, PotatoTokenTypes::LiteralIntegerValue);
                        }
                    }
                }
                Some(cont_node_details) => {
                    // once we matched the actual token type
                    // we are going to match on continuation node too.

                    match token.token_type {
                        PotatoTokenTypes::LiteralIntegerValue => {
                            match cont_node_details.get_type() {
                                AstNodeType::I32AstNode => {
                                    panic!("Syntax error! Two {} cannot follow each other! Token: {}, cont node: {}",
                                        PotatoTokenTypes::LiteralIntegerValue, token, cont_node_details);
                                }
                                AstNodeType::InfixOperationAstNode => {
                                    // this happens when we are right after the `+` sign in the
                                    // following example:
                                    // `i32 variable_name = 1 + 3;`
                                    ast.cont_node_api_check_if_left_empty_right_occupied(cont_node_details.get_id()?)
                                        .unwrap_or_else(|e|panic!("Continuation node, type: {}, consistency check failed. It has its left side empty, but right side occupied.",
                                        cont_node_details.get_type()));

                                    let created_i32_node_id = ast.i32_api_create_node_with_value(parse_literal_to_i32(&token))
                                        .unwrap_or_else(|e|panic!("Error happened while creating an {} node with value: {}. Details: {}",
                                        type_name::<I32AstNode>(), token, e));

                                    ast.infix_api_add_i32_node_to_the_right(created_i32_node_id)
                                        .unwrap_or_else(|e|panic!("Error happened while adding an {} type node with id: {} to an {} type node with id: {}. Details: {}", 
                                            AstNodeType::I32AstNode,
                                            created_i32_node_id,
                                            AstNodeType::InfixOperationAstNode,
                                            cont_node_details.get_id(),
                                            e
                                        ));
                                    cont_node
                                        .add_i32node_to_the_right(i32node_recorded.id)
                                        .unwrap_or_else(|e| panic!("{:#?}", e));
                                    ast.update_infix_node_by_id(cont_node.clone()).unwrap_or_else(|e|panic!("Error happened while persisting updated InfixOperationAstNode node. {:#?}", e));
                                    ast.continuation_node_api_update_node_id_and_type(
                                        i32node_recorded.id,
                                        AstNodeType::I32AstNode,
                                    )
                                    .unwrap_or_else(|e| {
                                        panic!("Updating continuation node. {:#?}", e)
                                    });

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
                            // here we assume that we are not the first character after the `=`, so
                            // we have to determine where are we, meaning investigating the AST and
                            // based on the result act.

                            // We check if the conditions are met to execute this part
                            match cont_node_details.1 {
                                AstNodeType::InfixOperationAstNode => {
                                    panic!("Cannot be an InfixAstNode right after another one. Actual node type by token: {:#?}; AST tree {:#?}", token, ast);
                                }
                                AstNodeType::None => {
                                    panic!(
                                        "Continuation node type is None: {:#?}, token: {:#?}",
                                        cont_node_details.1, token
                                    );
                                }
                                _ => {}
                            }
                            let parent_infix_node =
                                find_the_parent_infix_node(&ast, cont_node_details.clone())
                                .unwrap_or_else(|e|panic!("Error while looking for parent infix node. Node details: {:#?}, error details: {:#?}", cont_node_details.clone(), e));
                            let removed_left_node_id = parent_infix_node.clone()
                                .remove_left_node_and_return_id()
                                .unwrap_or_else(|e| panic!("Error happened while removing left if of infix node: {:#?}. Details: {:#?}", parent_infix_node, e));
                            let multiplication_infix_node = InfixAstNode::new_with_type_and_parent_node(
                                InfixOperationTypeEnum::Multiplication, 
                                &token.literal_value, 
                                parent_infix_node.clone().id)
                            .unwrap_or_else(|e|
                                    panic!("Error happened while creating infix node with type: {:#?}, parent id: {} and literal value: {}. Details: {:#?}", 
                                        InfixOperationTypeEnum::Multiplication, parent_infix_node.clone().id, token.literal_value, e));

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
                                        InfixAstNode::new_with_type_and_left_child_node(
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
                                    cont_node.parent_id = recorded_infix_node.id;
                                    let _ = ast.update_i32_node(
                                                        cont_node_details.0,
                                                        cont_node,
                                                    ).unwrap_or_else(|r|
                                            panic!("Updating the I32AstNode with the added InfixNode details failed. Details: {:#?}", r));
                                    let _ = ast
                                    .update_root_node_id_and_type(recorded_infix_node.id, AstNodeType::InfixOperationAstNode)
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

/// Finds the parent InfixNode based on the provided continuation node details.
///
/// # Remarks
/// In the process of building the AST and considering the operation precedence we need to walk
/// through the tree of InfixNodes and find the place where the actual node need to be placed. The
/// first step in this process is finding the parent Infix node.
fn find_the_parent_infix_node(
    ast: &MathPotatoAstTree,
    cont_node_details: (uuid::Uuid, AstNodeType),
) -> Result<InfixAstNode, ParseError> {
    match cont_node_details.1 {
        AstNodeType::I32AstNode => {
            let cont_node = ast
                .get_i32_node_by_id(cont_node_details.0)
                .unwrap_or_else(|e| {
                    panic!(
                        "There is no target node with {:#?} id. Error details: {:#?}",
                        cont_node_details.0, e
                    )
                });
            match cont_node.parent_type {
                AstNodeType::I32AstNode => Err(ParseError::new(format!(
                    "Structural error. A {:#?} node type cannot be parent of {:#?} node type.",
                    AstNodeType::I32AstNode,
                    AstNodeType::I32AstNode
                ))),
                AstNodeType::None => {
                    // this means that there is no parent for continuation node
                    // meaning, this is the case when we process the first infix node after the `=`
                    // sign
                    Err(ParseError::new(format!(
                        "Structural error. A {:#?} node type cannot be parent of {:#?} node type.",
                        AstNodeType::I32AstNode,
                        AstNodeType::I32AstNode
                    )))
                }
                AstNodeType::InfixOperationAstNode => {
                    match ast.get_infix_node_by_id(cont_node.parent_id) {
                        None => Err(ParseError::new(format!(
                            "There is no node in the AST with id: {}",
                            cont_node.parent_id
                        ))),
                        Some(res) => Ok(res),
                    }
                }
            }
        }
        AstNodeType::InfixOperationAstNode => {
            panic!(
                "Syntax error! Cannot be an {:#?} node before an Infix node type.",
                AstNodeType::InfixOperationAstNode,
            )
        }
        AstNodeType::None => {
            panic!(
                "Syntax error! Cannot be an {:#?} node before an Infix node type.",
                AstNodeType::None,
            )
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

    use crate::ast::ast_tree::ast_public_apis::cont_node_api::get_node_id_and_type::ContNodeApiGetIdAndType;
    use crate::ast::ast_tree::ast_public_apis::i32_api::i32_api_get_node_by_id::I32ApiGetNodeById;
    use crate::ast::ast_tree::ast_public_apis::i32_api::i32_api_node_count::I32ApiNodeCount;
    use crate::ast::ast_tree::ast_public_apis::infix_api::get_by_id::InfixApiGetNodeById;
    use crate::ast::ast_tree::ast_public_apis::infix_api::get_node_count::InfixApiGetNodeCount;
    use crate::ast::ast_tree::ast_public_apis::root_node::get_root_node_id::RootNodeApiGetRootNodeId;
    use crate::ast::ast_tree::ast_public_apis::root_node::get_root_node_type::RootNodeApiGetRootNodeType;
    use crate::ast::ast_tree::ast_public_apis::root_node::node_api_get_infix_by_id::RootNodeApiGetInfixNodeById;
    use crate::ast::ast_tree::ast_nodes::infix_ast_node::get_left_node_id_and_type::InfixAstNodeGetLeftNodeIdAndType;
    use crate::ast::ast_tree::ast_nodes::infix_ast_node::get_operation_type::InfixAstNodeGetOperationType;
    use crate::ast::ast_tree::ast_nodes::infix_ast_node::get_right_node_id_and_type::InfixAstNodeGetRightNodeIdAndType;
    use crate::ast::ast_tree::ast_nodes::infix_ast_node::is_left_occupied::InfixAstNodeIsLeftOccupied;
    use crate::ast::ast_tree::ast_nodes::infix_ast_node::is_right_occupied::InfixAstNodeIsRightOccupied;
    use crate::ast::ast_tree::MathPotatoAstTree;
    use crate::ast::infix_operation_type_enum::InfixOperationTypeEnum;
    use crate::{
        ast::ast_node_types_enum::AstNodeType, lexer::lexer::lexing,
        parser::parse_i32_statement_expression::parse_i32_statement_expression,
    };

    #[test]
    fn addition_and_multiplication_precedence_case() {
        // case: `2 + 3 * 4;`
        // arrange
        let input = String::from("1 + 2 * 3;");
        let lexed_input = lexing(&input);
        // lexed_input.iter().for_each(|i| println!("{:#?}", i));
        let input_ast = MathPotatoAstTree::new();

        // action
        let result = parse_i32_statement_expression(0, lexed_input, input_ast)
            .unwrap_or_else(|e| panic!("Parsing has failed! {:#?}", e));

        assert_eq!(
            result.get_i32_node_count(),
            2,
            "expected i32 node count was: {}, actual result: {}",
            result.get_i32_node_count(),
            2
        );
        assert_eq!(
            result.get_infix_node_count(),
            3,
            "expected infix node count was: {}, actual result: {}",
            result.get_infix_node_count(),
            2
        );
    }

    #[test]
    fn value_infix_and_value() {
        // case: `3 + 3;`
        // arrange
        let input = String::from("1 + 2;");
        let lexed_input = lexing(&input);
        let input_ast = MathPotatoAstTree::new();
        // println!("{:#?}", lexed_input);

        // action
        let result = parse_i32_statement_expression(0, lexed_input, input_ast)
            .unwrap_or_else(|e| panic!("There is no result: {:#?}", e));

        // assert
        assert_eq!(
            result.get_i32_node_count(),
            2,
            "sout: {}, expected: {}",
            result.get_i32_node_count(),
            2
        );
        assert_eq!(
            result.get_infix_node_count(),
            1,
            "sout: {}, expected: {}",
            result.get_infix_node_count(),
            1
        );

        // continuation node checks
        let continuation_node_id_and_type = result
            .cont_node_api_get_cont_node_id_and_type()
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
            .get_root_node_infix_by_id(root_node_id)
            .unwrap_or_else(|e| panic!("{:#?}", e));
        assert!(
            root_node.is_left_occupied(),
            "The left node of the root InfixOperationAstNode must be occupied at this point.",
        );
        assert!(
            root_node.is_right_occupied(),
            "The left node of the root InfixOperationAstNode must be occupied at this point.",
        );
        let left_children_id_and_type = root_node
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
            left_children.parent_id, root_node.id,
            "The parent id must be equal to root node id."
        );
        assert_eq!(
            left_children.parent_type,
            result.get_root_node_type(),
            "The parent type must be equal to root node type."
        );

        let right_children_id_and_type = root_node
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
            right_children.parent_id, root_node.id,
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
        assert_eq!(result.get_i32_node_count(), 1);
        assert_eq!(result.get_infix_node_count(), 1);

        // continuation node checks
        let continuation_node_id_and_type = result
            .cont_node_api_get_cont_node_id_and_type()
            .unwrap_or_else(|| panic!("There is no continuation node!"));
        assert_eq!(
            continuation_node_id_and_type.1,
            AstNodeType::InfixOperationAstNode
        );
        let cont_node = result
            .get_infix_node_by_id(continuation_node_id_and_type.0)
            .unwrap_or_else(|| panic!("There is no continuation node by id"));
        assert_eq!(cont_node.id, continuation_node_id_and_type.0);
        assert_eq!(
            cont_node.get_operation_type(),
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
            root_node.get_operation_type(),
            InfixOperationTypeEnum::Addition
        );
        let root_node_right_child_id_and_type = root_node
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
        assert_eq!(result.get_i32_node_count(), 1);
        let continuation_node_id = result
            .cont_node_api_get_cont_node_id_and_type()
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
