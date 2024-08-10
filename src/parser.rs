use crate::lexer::{self, Tokenizer};

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

struct Parser {
    current_token: lexer::Token,
}

impl Parser {

    pub fn get_next_token(&mut self) {
        self.current_token = lexer::Tokenizer::gettok()
}


