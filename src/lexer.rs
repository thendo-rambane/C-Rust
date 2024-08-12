use core::str;
use std::{
    io::{self, Read},
    iter,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    source: Box<iter::Peekable<str::Chars<'a>>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            source: Box::new(string.chars().peekable()),
        }
    }
    pub fn tokenize(&mut self) -> Token {
        let mut token = String::new();
        while let Some(c) = self.source.peek() {
            if c.is_whitespace() {
                self.source.next();
            } else if c.is_alphabetic() {
                while let Some(token_char) = self.source.peek() {
                    if token_char.is_alphanumeric() {
                        token = String::from(token + &token_char.to_string());
                        self.source.next();
                    } else {
                        break;
                    }
                }
                return match token.as_str() {
                    "def" => Token::Def,
                    "extern" => Token::Extern,
                    _ => Token::Identifier(token.clone()),
                };
            } else if c.is_numeric() {
                while let Some(token_char) = self.source.peek() {
                    if token.contains('.') && *token_char == '.' {
                        panic!("Multiple decimals")
                    }
                    if token_char.is_numeric() || *token_char == '.' {
                        token = String::from(token + &token_char.to_string());
                        self.source.next();
                    } else {
                        break;
                    }
                }
                return Token::Number(
                    token
                        .parse::<f64>()
                        .expect("Failed to parse numeric value."),
                );
            } else if *c == '#' {
                while let Some(comment_char) = self.source.peek() {
                    if !"\r\n".contains(*comment_char) {
                        self.source.next();
                    } else {
                        break;
                    }
                }
            } else {
                let char = c.to_string();
                self.source.next();
                return Token::Other(char);
            }
        }
        return Token::Eof;
    }
    pub fn gettok() -> Self {
        let mut string = String::new();
        io::stdin()
            .read_to_string(&mut string)
            .expect("Failed to read from stdin");
        let program = Box::leak(string.into_boxed_str());
        Self::new(program)
    }
}

#[cfg(test)]
mod test {
    use super::{Token, Tokenizer};

    #[test]
    fn eof() {
        let mut tokenizer = Tokenizer::new("");
        let expected = Token::Eof;
        let actual = tokenizer.tokenize();
        assert_eq!(expected, actual)
    }

    #[test]
    fn numeric() {
        let mut tokenizer = Tokenizer::new("1.45");
        let expected = Token::Number(1.45.into());
        let actual = tokenizer.tokenize();
        assert_eq!(expected, actual)
    }

    #[test]
    fn identifier() {
        let mut tokenizer = Tokenizer::new("ident");
        let expected = Token::Identifier(String::from("ident"));
        let actual = tokenizer.tokenize();
        assert_eq!(actual, expected)
    }

    #[test]
    fn reserved_define() {
        let mut tokenizer = Tokenizer::new("def");
        let expected = Token::Def;
        let actual = tokenizer.tokenize();
        assert_eq!(actual, expected)
    }

    #[test]
    fn reserved_extern() {
        let mut tokenizer = Tokenizer::new("extern");
        let expected = Token::Extern;
        let actual = tokenizer.tokenize();
        assert_eq!(actual, expected)
    }
    #[test]
    fn other() {
        let mut tokenizer = Tokenizer::new("{}");
        let expected = Token::Other(String::from("{"));
        let actual = tokenizer.tokenize();
        assert_eq!(actual, expected);
        let expected = Token::Other(String::from("}"));
        let actual = tokenizer.tokenize();
        assert_eq!(actual, expected)
    }

    #[test]
    fn multi_tokens() {
        let mut tokenizer = Tokenizer::new("{} test");
        let expected = Token::Other(String::from("{"));
        let actual = tokenizer.tokenize();
        assert_eq!(actual, expected);
        let expected = Token::Other(String::from("}"));
        let actual = tokenizer.tokenize();
        assert_eq!(actual, expected);
        let expected = Token::Identifier(String::from("test"));
        let actual = tokenizer.tokenize();
        assert_eq!(actual, expected);
    }
    #[test]
    fn multi_tokens_no_whitespace() {
        let mut tokenizer = Tokenizer::new("x+y");
        let expected_x = Token::Identifier(String::from("x"));
        let actual_x = tokenizer.tokenize();
        assert_eq!(actual_x, expected_x);
        let expected_plus = Token::Other(String::from("+"));
        let actual_plus = tokenizer.tokenize();
        assert_eq!(actual_plus, expected_plus);
        let expected_y = Token::Identifier(String::from("y"));
        let actual_y = tokenizer.tokenize();
        assert_eq!(actual_y, expected_y);
    }
}
