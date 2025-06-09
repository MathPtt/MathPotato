//! # MathPotato Programming Language
//!
//! This programming language is about mathematics, you know the calculus,
//! numerical analysis and stuff. Because why not!?

use lexer::lexer::lexing;
use parser::parse;

use self::ast::ast_tree::global::enums::potato_token::PotatoToken;

pub mod ast;
pub mod lexer;
pub mod parser;

fn main() {
    let tokens: Vec<PotatoToken> = lexing("dummy code");
    let _parsed = parse(tokens);
}
