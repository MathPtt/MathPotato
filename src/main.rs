//! # MathPotato Programming Language
//!
//! This programming language is about mathematics, you know the calculus, numerical
//! analysis and stuff. Because why not!?

use lexer::lexer::lexing;
use parser::parse;
use types::potato_token::PotatoToken;

pub mod lexer;
pub mod parser;
pub mod types;

fn main() {
    let tokens: Vec<PotatoToken> = lexing("dummy code");
    let _parsed = parse(tokens);
}
