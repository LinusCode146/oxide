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

pub struct IntegerLiteral {
    pub token: TokenType,
    pub value: i64,
}

pub struct ReturnStatement {
    pub token: TokenType,
    pub return_value: Option<Box<dyn Expression>>,
}

pub struct ExpressionStatement {
    pub token: TokenType,
    pub expression: Option<Box<dyn Expression>>
}

pub struct PrefixExpression {
    pub token: TokenType,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>
}

pub struct InfixExpression {
    pub token: TokenType,
    pub left: Option<Box<dyn Expression>>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>
}

pub struct Boolean {
    pub token: TokenType,
    pub value: bool
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

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.as_ref().map(|r| r.string()).unwrap_or_default())
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        self.token.get_literal()
    }
}
impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        format!("({} {} {})", self.left.as_ref().map(|r| r.string()).unwrap_or_default(), self.operator, self.right.as_ref().map(|r| r.string()).unwrap_or_default())
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

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn string(&self) -> String {
        self.value.clone().to_string()
    }
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

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

impl Expression for Boolean {
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