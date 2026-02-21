use std::any::Any;
use crate::token::TokenType;

pub trait Node {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub(crate) statements: Vec<Box<dyn Statement>>
}

pub struct LetStatement {
    pub token: TokenType,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}

pub struct Identifier {
    pub token: TokenType,
    pub value: String
}

pub struct ReturnStatement {
    pub token: TokenType,
    pub return_value: Option<Box<dyn Expression>>,
}

pub struct ExpressionStatement {
    pub token: TokenType,
    pub expression: Option<Box<dyn Expression>>
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        self.statements.iter().map(|s| s.string()).collect()
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        if let Some(name) = &self.name {
            out.push_str(&name.string());
        }
        out.push_str(" = ");
        if let Some(value) = &self.value {
            out.push_str(&value.string());
        }
        out.push_str(";");
        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        if let Some(value) = &self.return_value {
            out.push_str(&value.string());
        }
        out.push_str(";");
        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        if let Some(expr) = &self.expression {
            expr.string()
        } else {
            "".to_string()
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}