use std::collections;

use crate::lexer::{self, Token};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
struct Prototype {
    name: String,
    args: Vec<String>,
}

impl Prototype {
    pub fn get_name(self) -> String {
        self.name
    }
}

#[derive(Debug, PartialEq)]
struct Funcion {
    proto: Box<Prototype>,
    body: Box<Expression>,
}

#[derive(Debug, Clone)]
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

    pub fn operator(token: String) -> Result<Operator, String> {
        match token.as_str() {
            "+" => Ok(Operator::Plus),
            "-" => Ok(Operator::Minus),
            "/" => Ok(Operator::Divide),
            "*" => Ok(Operator::Multiply),
            _ => Err("Not operator".into()),
        }
    }

    pub fn get_token_precedence(&self) -> u32 {
        let token_precedence = collections::HashMap::from([
            (Operator::Plus, 20),
            (Operator::Minus, 20),
            (Operator::Divide, 30),
            (Operator::Multiply, 30),
        ]);
        if let lexer::Token::Other(token) = &self.current_token {
            let operator = Self::operator(token.into()).expect("Expected an operator");
            if let Some(precedent) = token_precedence.get(&operator) {
                return *precedent;
            }
        }
        return 0u32;
    }

    pub fn parse_binary_op_rhs(
        &mut self,
        expression_precedence: u32,
        lhs: Expression,
    ) -> Result<Expression, String> {
        loop {
            let token_precedence = self.get_token_precedence();
            if token_precedence < expression_precedence {
                return Ok(lhs);
            }
            let binary_operation = Self::operator(match self.current_token.clone() {
                lexer::Token::Other(token) => token,
                _ => "".to_string(),
            })
            .expect("Expected an Operator");
            self.get_next_token();

            let mut rhs = match self.parse_primary() {
                Ok(primary_expression) => primary_expression,
                Err(error) => return Err(format!("Binary Operation Error From {}", error)),
            };

            let next_token_precedence = self.get_token_precedence();
            if token_precedence < next_token_precedence {
                rhs = match self.parse_binary_op_rhs(token_precedence + 1, rhs) {
                    Ok(primary_expression) => primary_expression,
                    Err(error) => return Err(format!("Binary Operation Error From {}", error)),
                }
            }
            return Ok(Expression::BinaryExpression {
                operator: binary_operation,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
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
        let error = Err("unkown token expected expression".to_string());
        match self.current_token.clone() {
            lexer::Token::Identifier(_) => self.parse_identifier_expression(),
            lexer::Token::Number(_) => self.parse_number_expression(),
            lexer::Token::Other(token) => {
                if token == "(" {
                    self.parse_parenthesis_expression()
                } else {
                    error
                }
            }
            _ => error,
        }
    }

    fn parse_bin_op(&mut self) -> Result<Expression, String> {
        unimplemented!("TODO:")
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        let lhs = match self.parse_primary() {
            Ok(expression) => expression,
            Err(error) => return Err(format!("Failed to parse expression: {}", error)),
        };
        return self.parse_binary_op_rhs(1, lhs);
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
