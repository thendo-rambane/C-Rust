use std::collections;

use crate::lexer::{self};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
struct Prototype {
    name: String,
    args: Vec<String>,
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

impl Prototype {
    pub fn get_name(self) -> String {
        self.name
    }
}

#[derive(Debug, PartialEq)]
struct Function {
    prototype: Prototype,
    body: Expression,
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
    pub fn get_next_token(&mut self) -> lexer::Token {
        self.current_token = self.tokenizer.tokenize();
        return self.current_token.clone();
    }

    pub fn parse_number_expression(&mut self) -> Option<Expression> {
        match self.current_token {
            lexer::Token::Number(value) => {
                self.get_next_token();
                Some(Expression::NumberExpression(value))
            }
            _ => None,
        }
    }

    pub fn operator(token: String) -> Option<Operator> {
        match token.as_str() {
            "+" => Some(Operator::Plus),
            "-" => Some(Operator::Minus),
            "/" => Some(Operator::Divide),
            "*" => Some(Operator::Multiply),
            _ => None,
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
    ) -> Option<Expression> {
        loop {
            let token_precedence = self.get_token_precedence();
            if token_precedence < expression_precedence {
                return Some(lhs);
            }
            let binary_operation = Self::operator(match self.current_token.clone() {
                lexer::Token::Other(token) => token,
                _ => "".to_string(),
            })
            .expect("Expected an Operator");
            self.get_next_token();

            let mut rhs = match self.parse_primary() {
                Some(primary_expression) => primary_expression,
                _ => return None,
            };

            let next_token_precedence = self.get_token_precedence();
            if token_precedence < next_token_precedence {
                rhs = match self.parse_binary_op_rhs(token_precedence + 1, rhs) {
                    Some(primary_expression) => primary_expression,
                    _ => return None,
                }
            }
            return Some(Expression::BinaryExpression {
                operator: binary_operation,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
    }

    pub fn parse_identifier_expression(&mut self) -> Option<Expression> {
        match self.current_token.clone() {
            lexer::Token::Identifier(identifier) => {
                self.get_next_token();
                if let lexer::Token::Other(open_paren) = &self.current_token {
                    if open_paren != "(" {
                        let string = &identifier.clone();
                        return Some(Expression::VariableExpression(string.to_string()));
                    }
                    self.get_next_token();

                    let mut args: Vec<Box<Expression>> = Vec::new();
                    loop {
                        let arg = self.parse_expression();
                        match arg {
                            Some(parsed_arg) => args.push(Box::new(parsed_arg)),
                            _ => return None,
                        }
                        if let lexer::Token::Other(token) = &self.current_token {
                            if token == ")" {
                                break;
                            }
                            if token != "," {
                                return None;
                            }
                        }
                        self.get_next_token();
                    }
                    self.get_next_token();
                    Some(Expression::CallExpression {
                        callee: identifier.to_owned(),
                        args,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_prototype(&mut self) -> Option<Prototype> {
        match self.current_token.clone() {
            lexer::Token::Identifier(function_name) => {
                self.get_next_token();
                if let lexer::Token::Other(token) = self.current_token.clone() {
                    if token != "(" {
                        return None;
                    };
                    let mut argument_names: Vec<String> = Vec::new();
                    while let lexer::Token::Identifier(arg_identifier) = self.get_next_token() {
                        argument_names.push(arg_identifier);
                    }
                    if let lexer::Token::Other(end_token) = self.current_token.clone() {
                        if end_token != ")" {
                            return None;
                        }
                        self.get_next_token();
                    }
                    return Some(Prototype {
                        name: function_name,
                        args: argument_names,
                    });
                }
                return None;
            }
            _ => return None,
        }
    }

    fn parse_definition(&mut self) -> Option<Function> {
        self.get_next_token();
        if let Some(prototype) = self.parse_prototype() {
            if let Some(body) = self.parse_expression() {
                return Some(Function { prototype, body });
            }
        }
        None
    }

    fn parse_top_level_expression(&mut self) -> Option<Function> {
        if let Some(body) = self.parse_expression() {
            let prototype = Prototype {
                name: "__anon_expr".into(),
                args: vec![],
            };
            Some(Function { prototype, body })
        } else {
            None
        }
    }

    fn parse_extern(&mut self) -> Option<Prototype> {
        self.get_next_token();
        return self.parse_prototype();
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        match self.current_token.clone() {
            lexer::Token::Identifier(_) => self.parse_identifier_expression(),
            lexer::Token::Number(_) => self.parse_number_expression(),
            lexer::Token::Other(token) => {
                if token == "(" {
                    self.parse_parenthesis_expression()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        let lhs = match self.parse_primary() {
            Some(expression) => expression,
            _ => return None,
        };
        return self.parse_binary_op_rhs(1, lhs);
    }

    pub fn parse_parenthesis_expression(&mut self) -> Option<Expression> {
        match &self.current_token {
            lexer::Token::Other(_) => {
                self.get_next_token();
                let value = self.parse_expression();
                match value {
                    Some(expression) => {
                        if let lexer::Token::Other(close_paren) = &self.current_token {
                            if close_paren != ")" {
                                return None;
                            }
                            self.get_next_token();
                        }
                        Some(expression)
                    }
                    _ => return None,
                }
            }
            _ => None,
        }
    }
}
