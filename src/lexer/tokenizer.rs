use std::io::{self, Read};

#[derive(PartialEq, Debug)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    Other(String),
}

#[derive(PartialEq, Debug)]
pub struct Tokenizer {
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn tokenize(string: String) -> Token {
        let mut token = String::new();
        let mut string_iter = string.chars().into_iter().peekable();
        while let Some(c) = string_iter.peek() {
            if c.is_whitespace() {
                string_iter.next();
            } else if c.is_alphabetic() {
                while let Some(token_char) = string_iter.next() {
                    if token_char.is_alphanumeric() {
                        token = String::from(token + &token_char.to_string());
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
                while let Some(token_char) = string_iter.next() {
                    if token.contains('.') && token_char == '.' {
                        panic!("Multiple decimals")
                    }
                    if token_char.is_numeric() || token_char == '.' {
                        token = String::from(token + &token_char.to_string());
                    } else {
                        break;
                    }
                    return Token::Number(
                        token
                            .parse::<f64>()
                            .expect("Failed to parse numeric value."),
                    );
                }
            } else if *c == '#' {
                while let Some(comment_char) = string_iter.next() {
                    if !"\r\n".contains(comment_char) {
                    } else {
                        break;
                    }
                }
            } else {
                return Token::Other(c.to_string());
            }
        }
        return Token::Eof;
    }
    pub fn gettok() -> Token {
        let mut string = String::new();
        io::stdin()
            .read_to_string(&mut string)
            .expect("Failed to read from stdin");
        Self::tokenize(string)
    }
}

#[cfg(test)]
mod test {
    use super::{Token, Tokenizer};

    #[test]
    fn eof() {
        let expected = Token::Eof;
        let actual = Tokenizer::tokenize("".to_string());
        assert_eq!(expected, actual)
    }

    #[test]
    fn fails() {
        assert_eq!(1, 0)
    }
}
