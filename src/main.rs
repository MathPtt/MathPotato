//! # MathPotato Programming Language
//!
//! This programming language is about mathematics, you know the calculus, numerical
//! analysis and stuff. Because why not!?

use ast::potato_token::PotatoToken;
use lexer::lexer::lexing;
use parser::parse;

pub mod ast;
pub mod lexer;
pub mod parser;

fn main() {
    let tokens: Vec<PotatoToken> = lexing("dummy code");
    let _parsed = parse(tokens);
}
