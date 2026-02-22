use crate::token::TokenType;

// ── Node trait (nur noch für string/token_literal) ───────────────────────────

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

// ── Statement enum ────────────────────────────────────────────────────────────

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s)        => s.token_literal(),
            Statement::Return(s)     => s.token_literal(),
            Statement::Expression(s) => s.token_literal(),
            Statement::Block(s)      => s.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            Statement::Let(s)        => s.string(),
            Statement::Return(s)     => s.string(),
            Statement::Expression(s) => s.string(),
            Statement::Block(s)      => s.string(),
        }
    }
}

// ── Expression enum ───────────────────────────────────────────────────────────

pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(e)    => e.token_literal(),
            Expression::IntegerLiteral(e) => e.token_literal(),
            Expression::Boolean(e)       => e.token_literal(),
            Expression::Prefix(e)        => e.token_literal(),
            Expression::Infix(e)         => e.token_literal(),
            Expression::If(e)            => e.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            Expression::Identifier(e)    => e.string(),
            Expression::IntegerLiteral(e) => e.string(),
            Expression::Boolean(e)       => e.string(),
            Expression::Prefix(e)        => e.string(),
            Expression::Infix(e)         => e.string(),
            Expression::If(e)            => e.string(),
        }
    }
}

// ── Program ───────────────────────────────────────────────────────────────────

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements.first().map(|s| s.token_literal()).unwrap_or_default()
    }
    fn string(&self) -> String {
        self.statements.iter().map(|s| s.string()).collect()
    }
}

// ── Concrete statement types ──────────────────────────────────────────────────

pub struct LetStatement {
    pub token: TokenType,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        format!("{} {} = {};", self.token_literal(), self.name.string(), self.value.string())
    }
}

pub struct ReturnStatement {
    pub token: TokenType,
    pub return_value: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        format!("{} {};", self.token_literal(), self.return_value.string())
    }
}

pub struct ExpressionStatement {
    pub token: TokenType,
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.expression.string() }
}

pub struct BlockStatement {
    pub token: TokenType,
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.statements.first().map(|s| s.token_literal()).unwrap_or_default()
    }
    fn string(&self) -> String {
        self.statements.iter().map(|s| s.string()).collect()
    }
}

// ── Concrete expression types ─────────────────────────────────────────────────

pub struct Identifier {
    pub token: TokenType,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.value.clone() }
}

pub struct IntegerLiteral {
    pub token: TokenType,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.value.to_string() }
}

pub struct BooleanLiteral {
    pub token: TokenType,
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.token.get_literal() }
}

pub struct PrefixExpression {
    pub token: TokenType,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

pub struct InfixExpression {
    pub token: TokenType,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        format!("({} {} {})", self.left.string(), self.operator, self.right.string())
    }
}

pub struct IfExpression {
    pub token: TokenType,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        let mut out = format!("if {} {{ {} }}", self.condition.string(), self.consequence.string());
        if let Some(alt) = &self.alternative {
            out.push_str(&format!(" else {{ {} }}", alt.string()));
        }
        out
    }
}