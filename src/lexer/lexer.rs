use crate::types::{
    patterns::{
        KEYWORD_I32, SIGN_ADDITION, SIGN_ASSIGNMENT, SIGN_CLOSE_PARENTHESES, SIGN_DIVISION,
        SIGN_OPEN_PARENTHESES, SIGN_SEMICOLON, WHITESPACE,
    },
    potato_token::PotatoToken,
    potato_token_types::PotatoTokenTypes,
};

pub fn lexing(s: &str) -> Vec<PotatoToken> {
    if s.is_empty() {
        return vec![];
    }
    let mut tokens: Vec<PotatoToken> = vec![];

    let mut actual = String::new();
    for c in s.chars() {
        // whitespace marks where we stop collecting the characters to be tokenized
        // semicolon shows the end of the line
        if is_token_whitespace(&c) || is_token_semicolon(&c) {
            tokens.push(tokenize(&actual));
            actual = String::new();
            // when we send the actual to be tokenized and the triggering character is ;
            // we need to tokenize that too
            if is_token_semicolon(&c) {
                tokens.push(tokenize(&c.to_string()));
            }
        } else {
            actual.push(c);
        }
    }

    tokens
}

fn tokenize(s: &str) -> PotatoToken {
    match s {
        SIGN_OPEN_PARENTHESES => PotatoToken {
            token_type: PotatoTokenTypes::SignOpenParentheses,
            literal_value: s.to_string(),
        },
        SIGN_CLOSE_PARENTHESES => PotatoToken {
            token_type: PotatoTokenTypes::SignCloseParentheses,
            literal_value: s.to_string(),
        },
        SIGN_SEMICOLON => PotatoToken {
            token_type: PotatoTokenTypes::SignSemicolon,
            literal_value: s.to_string(),
        },
        SIGN_ASSIGNMENT => PotatoToken {
            token_type: PotatoTokenTypes::SignAssignment,
            literal_value: s.to_string(),
        },
        SIGN_ADDITION => PotatoToken {
            token_type: PotatoTokenTypes::OperationAddition,
            literal_value: s.to_string(),
        },
        SIGN_DIVISION => PotatoToken {
            token_type: PotatoTokenTypes::OperationDivision,
            literal_value: s.to_string(),
        },
        KEYWORD_I32 => PotatoToken {
            token_type: PotatoTokenTypes::KeywordI32,
            literal_value: s.to_string(),
        },
        _ => {
            if is_integer_value(s) {
                PotatoToken {
                    token_type: PotatoTokenTypes::LiteralIntegerValue,
                    literal_value: s.to_string(),
                }
            } else {
                PotatoToken {
                    token_type: PotatoTokenTypes::LiteralValueVariableIdentifier,
                    literal_value: s.to_string(),
                }
            }
        }
    }
}

fn is_token_whitespace(c: &char) -> bool {
    c.to_string().eq(WHITESPACE)
}
fn is_token_semicolon(c: &char) -> bool {
    c.to_string().eq(SIGN_SEMICOLON)
}
fn is_integer_value(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use crate::types::{potato_token::PotatoToken, potato_token_types::PotatoTokenTypes};

    use super::*;

    #[test]
    fn lexing_test_strings() {
        struct CharToTokensData {
            input: String,
            expected: Vec<PotatoToken>,
        }
        let test_data = vec![CharToTokensData {
            input: String::from("3 + 3;"),
            expected: vec![
                PotatoToken {
                    token_type: PotatoTokenTypes::LiteralIntegerValue,
                    literal_value: String::from("3"),
                },
                PotatoToken {
                    token_type: PotatoTokenTypes::OperationAddition,
                    literal_value: String::from("+"),
                },
                PotatoToken {
                    token_type: PotatoTokenTypes::LiteralIntegerValue,
                    literal_value: String::from("3"),
                },
                PotatoToken {
                    token_type: PotatoTokenTypes::SignSemicolon,
                    literal_value: String::from(";"),
                },
            ],
        }];
        for d in test_data {
            let result = lexing(&d.input);
            assert_ne!(result.len(), 0);
            for (i, r) in d.expected.iter().enumerate() {
                assert_eq!(r, &result[i]);
            }
        }
    }

    #[test]
    fn lexing_characters_test() {
        struct CharToTokensData {
            input: String,
            expected: Vec<PotatoToken>,
        }
        let test_data = vec![CharToTokensData {
            input: String::from("i32 asd = 6;"),
            expected: vec![
                PotatoToken {
                    token_type: PotatoTokenTypes::KeywordI32,
                    literal_value: String::from("i32"),
                },
                PotatoToken {
                    token_type: PotatoTokenTypes::LiteralValueVariableIdentifier,
                    literal_value: String::from("asd"),
                },
                PotatoToken {
                    token_type: PotatoTokenTypes::SignAssignment,
                    literal_value: String::from("="),
                },
                PotatoToken {
                    token_type: PotatoTokenTypes::LiteralIntegerValue,
                    literal_value: String::from("6"),
                },
                PotatoToken {
                    token_type: PotatoTokenTypes::SignSemicolon,
                    literal_value: String::from(";"),
                },
            ],
        }];
        for d in test_data {
            let result = lexing(&d.input);
            assert_ne!(result.len(), 0);
            for (i, r) in d.expected.iter().enumerate() {
                assert_eq!(r, &result[i]);
            }
        }
    }

    #[test]
    fn tokenize_characters_test() {
        struct CharToTokenData {
            input: String,
            expected: PotatoToken,
        }
        let test_data = vec![
            CharToTokenData {
                input: String::from("("),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::SignOpenParentheses,
                    literal_value: String::from("("),
                },
            },
            CharToTokenData {
                input: String::from(")"),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::SignCloseParentheses,
                    literal_value: String::from(")"),
                },
            },
            CharToTokenData {
                input: String::from(";"),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::SignSemicolon,
                    literal_value: String::from(";"),
                },
            },
            CharToTokenData {
                input: String::from("="),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::SignAssignment,
                    literal_value: String::from("="),
                },
            },
            CharToTokenData {
                input: String::from("+"),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::OperationAddition,
                    literal_value: String::from("+"),
                },
            },
            CharToTokenData {
                input: String::from("/"),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::OperationDivision,
                    literal_value: String::from("/"),
                },
            },
            CharToTokenData {
                input: String::from("i32"),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::KeywordI32,
                    literal_value: String::from("i32"),
                },
            },
            CharToTokenData {
                input: String::from("6"),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::LiteralIntegerValue,
                    literal_value: String::from("6"),
                },
            },
            CharToTokenData {
                input: String::from("66"),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::LiteralIntegerValue,
                    literal_value: String::from("66"),
                },
            },
            CharToTokenData {
                input: String::from("a66"),
                expected: PotatoToken {
                    token_type: PotatoTokenTypes::LiteralValueVariableIdentifier,
                    literal_value: String::from("a66"),
                },
            },
        ];

        for t in test_data {
            let result = tokenize(&t.input);
            assert_eq!(result.token_type, t.expected.token_type);
            assert_eq!(result.literal_value, t.expected.literal_value);
        }
    }
}
