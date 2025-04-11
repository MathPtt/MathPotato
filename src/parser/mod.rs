use parser_error::PotatoParserError;

use crate::types::{
    ast_tree::PotatoAstTree, potato_token::PotatoToken, potato_token_types::PotatoTokenTypes,
};

pub mod parse_integer_statement;
pub mod parse_integer_statement_expression;
pub mod parser_error;

pub fn parse(tokens: Vec<PotatoToken>) -> Result<PotatoAstTree, PotatoParserError> {
    let mut potato_ast_tree = PotatoAstTree::new();
    for (i, t) in tokens.iter().enumerate() {
        if t.token_type == PotatoTokenTypes::KeywordInteger {
            match parse_integer_statement::parse_integer_statement(i, &tokens) {
                Ok(r) => {
                    let Ok(_) = potato_ast_tree.merge(r.1.get_nodes()) else {
                        panic!("Adding nodes to AST has failed")
                    };
                }
                Err(err) => {
                    panic!("Parser error: {:#?}", err);
                }
            }
        }
    }
    Ok(potato_ast_tree)
}
