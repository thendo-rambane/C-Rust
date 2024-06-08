use std::iter;
use std::mem;
use std::str;

#[derive(PartialEq, Debug)]
pub enum LiteralType {
    Int(i32),
}

#[derive(PartialEq, Debug)]
pub enum IdentifierType {
    Var(Identifier),
}

#[derive(PartialEq, Debug)]
pub struct Identifier {
    value: LiteralType,
    label: String,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Literal(LiteralType),
    Identifier(IdentifierType),
}

#[derive(PartialEq, Debug)]
pub struct Tokenizer {
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn tokenize_int_literal(iter: &mut iter::Peekable<str::Chars>) -> Token {
        let mut token_buffer = String::with_capacity(mem::size_of::<i32>());
        while let Some(digit) = iter.next() {
            if digit.is_digit(10) {
                token_buffer.insert(token_buffer.len(), digit)
            }
        }

        if let Some(after_digit) = iter.peek() {
            if !after_digit.is_whitespace() {
                todo!("Handle error case");
            }
        }
        let token_value = token_buffer.parse::<i32>().unwrap();
        Token::Literal(LiteralType::Int(token_value))
    }
    pub fn parse_alpha_num(iter: &mut iter::Peekable<str::Chars>) -> String {
        let mut token_buffer = String::new();
        while let Some(c) = iter.next() {
            if c.is_ascii_alphanumeric() || c == '_' {
                token_buffer = String::from(token_buffer + &c.to_string());
            }
        }
        token_buffer
    }

    fn is_reserved(string: &str) -> bool {
        match string {
            "int" => true,
            _ => false,
        }
    }

    pub fn from_string(string: String) -> Tokenizer {
        let mut iter = string.chars().into_iter().peekable();

        let mut tokens: Vec<Token> = Vec::new();

        while let Some(character) = iter.peek() {
            if character.is_alphabetic() {
                let token_buffer = Tokenizer::parse_alpha_num(&mut iter);
                if Tokenizer::is_reserved(token_buffer.as_str()) {}
                let token = match token_buffer.as_str() {
                    "int" => {
                        if let Some(c) = iter.peek() {
                            if !c.is_whitespace() || *c != ';' {
                                todo!("Handle error case");
                            }
                            iter.next();
                        }
                    }
                    _ => {}
                };
            } else if character.is_digit(10) {
                let token = Tokenizer::tokenize_int_literal(&mut iter);
                tokens.push(token);
                break;
            }
        }

        Tokenizer { tokens }
    }
}

#[cfg(test)]
mod test {
    use super::{Identifier, IdentifierType, LiteralType, Token, Tokenizer};

    #[test]
    fn test_tokenize_int_literal() {
        let string_tokens = "1";
        let tokens = Tokenizer::from_string(string_tokens.to_string());
        let expected = Tokenizer {
            tokens: vec![Token::Literal(LiteralType::Int(1))],
        };
        assert_eq!(tokens, expected)
    }

    #[test]
    fn test_tokenize_itentifier() {
        let string_tokens = "int num;";
        let tokens = Tokenizer::from_string(string_tokens.to_string());
        let expected = Tokenizer {
            tokens: vec![Token::Identifier(IdentifierType::Var(Identifier {
                value: LiteralType::Int(0),
                label: String::from("num"),
            }))],
        };
        assert_eq!(tokens, expected)
    }
}
