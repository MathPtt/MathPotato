use parser_error::ParseError;

use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::global::enums::potato_token::PotatoToken;
use crate::ast::ast_tree::global::enums::potato_token_types::PotatoTokenTypes;

pub mod parse_i32_statement;
pub mod parse_i32_statement_expression;
pub mod parser_error;

pub fn parse(tokens: Vec<PotatoToken>) -> Result<MathPotatoAstTree, ParseError> {
    let mut math_potato_ast_tree = MathPotatoAstTree::new();
    for (i, t) in tokens.iter().enumerate() {
        if t.token_type == PotatoTokenTypes::KeywordI32 {
            match parse_i32_statement::parse_i32_statement(i, &tokens) {
                Ok(r) => {
                    math_potato_ast_tree = r.1
                    // math_potato_ast_tree.clone().merge(r.1.clone()).unwrap();
                }
                Err(err) => {
                    panic!("Parser error: {:#?}", err);
                }
            }
        }
    }
    Ok(math_potato_ast_tree)
}
