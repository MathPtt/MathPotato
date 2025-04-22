use parser_error::MathPotatoParserError;

use crate::types::{
    math_potato_ast_tree::MathPotatoAstTree, potato_token::PotatoToken,
    potato_token_types::PotatoTokenTypes,
};

pub mod parse_i32_statement;
pub mod parse_i32_statement_expression;
pub mod parser_error;

pub fn parse(tokens: Vec<PotatoToken>) -> Result<MathPotatoAstTree, MathPotatoParserError> {
    let mut math_potato_ast_tree = MathPotatoAstTree::new();
    for (i, t) in tokens.iter().enumerate() {
        if t.token_type == PotatoTokenTypes::KeywordI32 {
            match parse_i32_statement::parse_i32_statement(i, &tokens) {
                Ok(r) => {
                    let Ok(_) = math_potato_ast_tree.merge(r.1.get_nodes()) else {
                        panic!("Adding nodes to AST has failed")
                    };
                }
                Err(err) => {
                    panic!("Parser error: {:#?}", err);
                }
            }
        }
    }
    Ok(math_potato_ast_tree)
}
