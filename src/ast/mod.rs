use crate::token::TokenType;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
    Assign(AssignStatement)
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s)        => s.token_literal(),
            Statement::Return(s)     => s.token_literal(),
            Statement::Expression(s) => s.token_literal(),
            Statement::Block(s)      => s.token_literal(),
            Statement::Assign(a) => a.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            Statement::Let(s)        => s.string(),
            Statement::Return(s)     => s.string(),
            Statement::Expression(s) => s.string(),
            Statement::Block(s)      => s.string(),
            Statement::Assign(s)      => s.string(),
        }
    }
}
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
    FunctionLiteral(FunctionLiteral),
    Call(CallExpression),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    Index(IndexExpression),
    HashLiteral(HashLiteral),
    WhileLoop(WhileExpression)
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
            Expression::FunctionLiteral(e) => e.token_literal(),
            Expression::Call(e) => e.token_literal(),
            Expression::StringLiteral(s) => s.token_literal(),
            Expression::ArrayLiteral(a) => a.token_literal(),
            Expression::Index(i  ) => i.token_literal(),
            Expression::HashLiteral(i  ) => i.token_literal(),
            Expression::WhileLoop(w ) => w.token_literal(),
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
            Expression::FunctionLiteral(e) => e.string(),
            Expression::Call(e) => e.string(),
            Expression::StringLiteral(s) => s.string(),
            Expression::ArrayLiteral(e) => e.string(),
            Expression::Index(i  ) => i.string(),
            Expression::HashLiteral(i  ) => i.string(),
            Expression::WhileLoop(i  ) => i.string(),
        }
    }
}

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
#[derive(Clone, Debug)]
#[derive(PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct WhileExpression {
    pub token: TokenType,
    pub condition: Box<Expression>,
    pub body: BlockStatement,
}

impl Node for WhileExpression {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        format!("while {} {{ {} }}", self.condition.string(), self.body.string())
    }
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
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
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct ExpressionStatement {
    pub token: TokenType,
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.expression.string() }
}
#[derive(Clone, Debug)]
#[derive(PartialEq)]
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
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct Identifier {
    pub token: TokenType,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.value.clone() }
}
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct IntegerLiteral {
    pub token: TokenType,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.value.to_string() }
}
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct BooleanLiteral {
    pub token: TokenType,
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.token.get_literal() }
}
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct FunctionLiteral {
    pub token: TokenType,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        let params = self.parameters
            .iter()
            .map(|p| p.string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("{}({}) {{ {} }}", self.token.get_literal(), params, self.body.string())
    }
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct StringLiteral {
    pub token: TokenType,
    pub value: String,
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String { self.value.clone() }
}



#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct IndexExpression {
    pub token: TokenType,
    pub left: Box<Expression>,
    pub index: Box<Expression>
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct CallExpression {
    pub token: TokenType,
    pub function: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

impl Node for IndexExpression {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        format!("({}[{}])", self.left.string(), self.index.string())
    }
}

impl Node for CallExpression {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        let args = self.arguments
            .iter()
            .map(|a| a.string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("{}({})", self.function.string(), args)
    }
}
#[derive(Clone, Debug)]
#[derive(PartialEq)]
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

#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct HashLiteral {
    pub token: TokenType,
    pub pairs: Vec<(Expression, Expression)>,
}

impl Node for HashLiteral {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        let pairs = self.pairs
            .iter()
            .map(|(key, value)| format!("{}:{}", key.string(), value.string()))
            .collect::<Vec<_>>()
            .join(", ");

        format!("{{{}}}", pairs)
    }
}


#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct ArrayLiteral {
    pub token: TokenType,
    pub elements: Vec<Box<Expression>>
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        let elements = self.elements
            .iter()
            .map(|e| e.string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{}]", elements)
    }
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
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
#[derive(Clone, Debug)]
#[derive(PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct AssignStatement {
    pub token: TokenType,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for AssignStatement {
    fn token_literal(&self) -> String { self.token.get_literal() }
    fn string(&self) -> String {
        format!("{} = {};", self.name.string(), self.value.string())
    }
}