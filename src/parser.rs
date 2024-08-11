use std::string;

use crate::lexer::{self};

enum Operator {
    Plus,
}

enum Expression {
    NumberExpression(f64),
    VariableExpression(String),
    BinaryExpression {
        operator: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    CallExpression {
        callee: String,
        args: Vec<Box<Expression>>,
    },
    Null,
}

struct Prototype {
    name: String,
    args: Vec<String>,
}

impl Prototype {
    pub fn get_name(self) -> String {
        self.name
    }
}

struct Funcion {
    proto: Box<Prototype>,
    body: Box<Expression>,
}

struct Parser<'a> {
    current_token: lexer::Token,
    tokenizer: lexer::Tokenizer<'a>,
}

impl Parser<'_> {
    pub fn new(string: String) -> Self {
        let mut tokenizer = lexer::Tokenizer::new(Box::leak(string.into_boxed_str()));
        let current_token = tokenizer.tokenize();
        Self {
            current_token,
            tokenizer,
        }
    }
    pub fn get_next_token(&mut self) {
        self.current_token = self.tokenizer.tokenize();
    }

    pub fn parse_number_expression(&mut self) -> Result<Expression, String> {
        match self.current_token {
            lexer::Token::Number(value) => {
                self.get_next_token();
                Ok(Expression::NumberExpression(value))
            }
            _ => Err("Expected a 'Number'".into()),
        }
    }

    pub fn parse_identifier_expression(&mut self) -> Result<Expression, String> {
        match self.current_token.clone() {
            lexer::Token::Identifier(identifier) => {
                self.get_next_token();
                if let lexer::Token::Other(open_paren) = &self.current_token {
                    if open_paren != "(" {
                        let string = &identifier.clone();
                        return Ok(Expression::VariableExpression(string.to_string()));
                    }
                    self.get_next_token();

                    let mut args: Vec<Box<Expression>> = Vec::new();
                    loop {
                        let arg = self.parse_expression();
                        match arg {
                            Ok(parsed_arg) => args.push(Box::new(parsed_arg)),
                            Err(arg_error) => {
                                return Err(format!("Failed to parse args: {}", arg_error))
                            }
                        }
                        if let lexer::Token::Other(token) = &self.current_token {
                            if token == ")" {
                                break;
                            }
                            if token != "," {
                                return Err("Expected ')' or ',' in argument list".into());
                            }
                        }
                        self.get_next_token();
                    }
                    self.get_next_token();
                    Ok(Expression::CallExpression {
                        callee: identifier.to_owned(),
                        args,
                    })
                } else {
                    Err("Expected 'Other' token".into())
                }
            }
            _ => Err("Not identifier".into()),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.current_token.clone() {
            lexer::Token::Eof => todo!(),
            lexer::Token::Def => todo!(),
            lexer::Token::Extern => todo!(),
            lexer::Token::Identifier(_) => self.parse_identifier_expression(),
            lexer::Token::Number(_) => self.parse_number_expression(),
            lexer::Token::Other(token) => {
                if token == "(" {
                    self.parse_parenthesis_expression()
                } else {
                    Err("unkown token expected expression".to_string())
                }
            }
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        unimplemented!("TODO:")
    }

    pub fn parse_parenthesis_expression(&mut self) -> Result<Expression, String> {
        match &self.current_token {
            lexer::Token::Other(_) => {
                self.get_next_token();
                let value = self.parse_expression();
                match value {
                    Ok(expression) => {
                        if let lexer::Token::Other(close_paren) = &self.current_token {
                            if close_paren != ")" {
                                return Err("Expeced ')'".into());
                            }
                            self.get_next_token();
                        }
                        Ok(expression)
                    }
                    Err(error) => return Err(error),
                }
            }
            _ => Err("Not paren".into()),
        }
    }
}
