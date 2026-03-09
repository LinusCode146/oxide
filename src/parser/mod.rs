use crate::lexer::Lexer;
use crate::token::TokenType;
use crate::ast::{ArrayLiteral, AssignStatement, BlockStatement, BooleanLiteral, CallExpression, Expression, ExpressionStatement, FunctionLiteral, HashLiteral, Identifier, IfExpression, IndexExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program, ReturnStatement, Statement, StringLiteral, WhileExpression};
use crate::parser::Precedence::Lowest;

#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

fn token_precedence(t: &TokenType) -> Precedence {
    match t {
        TokenType::EQ | TokenType::NEQ              => Precedence::Equals,
        TokenType::LT | TokenType::GT               => Precedence::LessGreater,
        TokenType::PLUS | TokenType::MINUS          => Precedence::Sum,
        TokenType::MUL | TokenType::DIV             => Precedence::Product,
        TokenType::LPAREN                           => Precedence::Call,
        TokenType::LBRACKET | TokenType::DOT                      => Precedence::Index,
        _                                           => Lowest,
    }
}


#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    fn new(msg: impl Into<String>) -> Self {
        ParseError { message: msg.into() }
    }
}


pub struct Parser {
    pos: usize,
    peek_pos: usize,
    tokens: Vec<TokenType>,
    cur_token: TokenType,
    pub errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let tokens = l.get_tokens();
        let cur_token = tokens.get(0).cloned().unwrap_or(TokenType::ILLEGAL);
        let mut p = Parser {
            pos: 0,
            peek_pos: 0,
            tokens,
            cur_token,
            errors: vec![],
        };
        p.next_token();
        p
    }


    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token != TokenType::EOF {
            match self.parse_statement() {
                Ok(stmt)  => program.statements.push(stmt),
                Err(e)    => self.errors.push(e),
            }
            self.next_token();
        }

        program
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

    pub fn check_parser_errors(&self) {
        if self.errors.is_empty() { return; }
        eprintln!("Parser has encountered {} error(s):", self.errors.len());
        for e in &self.errors {
            eprintln!("  {}", e.message);
        }
        panic!("Parser errors found!");
    }


    fn next_token(&mut self) {
        self.cur_token = self.tokens
            .get(self.peek_pos)
            .cloned()
            .unwrap_or(TokenType::ILLEGAL);
        self.pos = self.peek_pos;
        self.peek_pos += 1;
    }

    fn cur_token_is(&self, t: &TokenType) -> bool {
        std::mem::discriminant(&self.cur_token) == std::mem::discriminant(t)
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        match self.tokens.get(self.peek_pos) {
            Some(tok) => std::mem::discriminant(tok) == std::mem::discriminant(t),
            None      => false,
        }
    }

    fn expect_peek(&mut self, t: TokenType) -> Result<(), ParseError> {
        if self.peek_token_is(&t) {
            self.next_token();
            Ok(())
        } else {
            Err(ParseError::new(format!(
                "Expected next token to be {} but got {} instead!",
                t.get_literal(),
                self.tokens.get(self.peek_pos)
                    .map(|t| t.get_literal())
                    .unwrap_or_default()
            )))
        }
    }

    fn peek_precedence(&self) -> Precedence {
        self.tokens.get(self.peek_pos)
            .map(token_precedence)
            .unwrap_or(Lowest)
    }

    fn cur_precedence(&self) -> Precedence {
        token_precedence(&self.cur_token)
    }


    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.cur_token {
            TokenType::LET    => self.parse_let_statement().map(Statement::Let),
            TokenType::RETURN => self.parse_return_statement().map(Statement::Return),
            TokenType::FUNCTION if self.peek_token_is(&TokenType::IDENT(String::new())) => {
                self.parse_named_function_statement().map(Statement::Let)
            }
            TokenType::IDENT(_) if self.peek_token_is(&TokenType::ASSIGN) => {
                self.parse_assign_statement().map(Statement::Assign)
            }
            _ => self.parse_expression_statement().map(Statement::Expression),
        }
    }

    fn parse_assign_statement(&mut self) -> Result<AssignStatement, ParseError> {
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.get_literal(),
        };
        self.next_token(); // move past ident to `=`
        self.next_token(); // move past `=` to value

        let value = self.parse_expression(Lowest)?;

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(AssignStatement { token: name.token.clone(), name, value })
    }

    fn parse_named_function_statement(&mut self) -> Result<LetStatement, ParseError> {
        let fun_token = self.cur_token.clone();
        self.next_token();

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.get_literal(),
        };

        self.expect_peek(TokenType::LPAREN)?;
        let parameters = self.parse_function_parameters()?;
        self.expect_peek(TokenType::LBRACE)?;
        let body = self.parse_block_statement();

        let value = Expression::FunctionLiteral(FunctionLiteral {
            token: fun_token.clone(),
            parameters,
            body,
        });

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(LetStatement { token: fun_token, name, value })
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, ParseError> {
        let token = self.cur_token.clone();

        self.expect_peek(TokenType::IDENT(String::new()))?;
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.get_literal(),
        };

        self.expect_peek(TokenType::ASSIGN)?;
        self.next_token(); // move past '='

        let value = self.parse_expression(Lowest)?;

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(LetStatement { token, name, value })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParseError> {
        let token = self.cur_token.clone();
        self.next_token();

        let return_value = self.parse_expression(Lowest)?;

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(ReturnStatement { token, return_value })
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, ParseError> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Lowest)?;

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(ExpressionStatement { token, expression })
    }


    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        if matches!(self.cur_token, TokenType::SEMICOLON | TokenType::EOF | TokenType::ILLEGAL) {
            return Err(ParseError::new(format!(
                "Unexpected token '{}'", self.cur_token.get_literal()
            )));
        }

        let mut left = self.parse_prefix()?;

        while !self.peek_token_is(&TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            // only advance if there actually is an infix handler
            if !self.has_infix_handler() {
                return Ok(left);
            }
            self.next_token();
            left = self.parse_infix(left)?;
        }

        Ok(left)
    }

    fn has_infix_handler(&self) -> bool {
        matches!(
            self.tokens.get(self.peek_pos),
            Some(TokenType::PLUS  | TokenType::MINUS | TokenType::MUL  |
                 TokenType::DIV   | TokenType::EQ    | TokenType::NEQ  |
                 TokenType::LT    | TokenType::GT    | TokenType::LPAREN |
                TokenType::LBRACKET | TokenType::DOT )
        )
    }

    fn parse_string_literal(&self) -> Result<Expression, ParseError> {
        match &self.cur_token {
            TokenType::STRING(s) => Ok(Expression::StringLiteral(StringLiteral {
                token: self.cur_token.clone(),
                value: s.clone(),
            })),
            _ => Err(ParseError::new("Expected string literal")),
        }
    }


    fn parse_prefix(&mut self) -> Result<Expression, ParseError> {
        match &self.cur_token {
            TokenType::IDENT(_)  => Ok(self.parse_identifier()),
            TokenType::INT(_)    => self.parse_integer_literal(),
            TokenType::TRUE
            | TokenType::FALSE   => Ok(self.parse_boolean()),
            TokenType::BANG
            | TokenType::MINUS   => self.parse_prefix_expression(),
            TokenType::LPAREN    => self.parse_grouped_expression(),
            TokenType::IF        => self.parse_if_expression(),
            TokenType::WHILE        => self.parse_while_expression(),
            TokenType::FUNCTION  => self.parse_function_literal(),
            TokenType::STRING(_) => self.parse_string_literal(),
            TokenType::LBRACKET  => self.parse_array_literal(),
            TokenType::LBRACE  => self.parse_hash_literal(),
            other => Err(ParseError::new(format!(
                "No prefix parse function for '{:?}' found!", other
            ))),
        }
    }

    fn parse_hash_literal(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let mut pairs = Vec::new();

        while !self.peek_token_is(&TokenType::RBRACE) {
            self.next_token();
            let key = self.parse_expression(Lowest)?;
            self.expect_peek(TokenType::COLON)?;
            self.next_token();
            let value = self.parse_expression(Lowest)?;

            pairs.push((key, value));

            if !self.peek_token_is(&TokenType::RBRACE) {
                self.expect_peek(TokenType::COMMA)?;
            }
        }

        self.expect_peek(TokenType::RBRACE)?;
        Ok(Expression::HashLiteral(HashLiteral { token, pairs }))
    }

    fn parse_array_literal(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let elements = self.parse_expression_list(TokenType::RBRACKET)?;
        Ok(
            Expression::ArrayLiteral(ArrayLiteral {
                token, elements
            })
        )
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Result<Vec<Box<Expression>>, ParseError> {
        let mut list = Vec::new();

        if self.peek_token_is(&end) {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        list.push(Box::new(self.parse_expression(Lowest)?));

        while self.peek_token_is(&TokenType::COMMA) {
            self.next_token();
            self.next_token();
            list.push(Box::new(self.parse_expression(Lowest)?));
        }

        self.expect_peek(end)?;

        Ok(list)
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.get_literal(),
        })
    }



    fn parse_integer_literal(&self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let value = token.get_literal().parse::<i64>().map_err(|_| {
            ParseError::new(format!("Could not parse '{}' as i64", token.get_literal()))
        })?;
        Ok(Expression::IntegerLiteral(IntegerLiteral { token, value }))
    }

    fn parse_boolean(&self) -> Expression {
        Expression::Boolean(BooleanLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token_is(&TokenType::TRUE),
        })
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let token    = self.cur_token.clone();
        let operator = self.cur_token.get_literal();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression::Prefix(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_token();
        let expr = self.parse_expression(Lowest)?;
        self.expect_peek(TokenType::RPAREN)?;
        Ok(expr)
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();

        self.expect_peek(TokenType::LPAREN)?;
        let parameters = self.parse_function_parameters()?;
        self.expect_peek(TokenType::LBRACE)?;
        let body = self.parse_block_statement();

        Ok(Expression::FunctionLiteral(FunctionLiteral {
            token,
            parameters,
            body
        }))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, ParseError> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(&TokenType::RPAREN) {
            self.next_token();
            return Ok(identifiers);
        }

        self.next_token();
        identifiers.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.get_literal(),
        });

        while self.peek_token_is(&TokenType::COMMA) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.get_literal(),
            });
        }

        self.expect_peek(TokenType::RPAREN)?;

        Ok(identifiers)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();

        self.next_token();
        let condition = self.parse_expression(Lowest)?;
        self.expect_peek(TokenType::LBRACE)?;
        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token_is(&TokenType::ELSE) {
            self.next_token();
            self.expect_peek(TokenType::LBRACE)?;
            Some(self.parse_block_statement())
        } else {
            None
        };

        Ok(Expression::If(IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    fn parse_while_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();

        self.next_token();
        let condition = self.parse_expression(Lowest)?;
        self.expect_peek(TokenType::LBRACE)?;
        let body = self.parse_block_statement();


        Ok(Expression::WhileLoop(WhileExpression {
            token,
            condition: Box::new(condition),
            body,
        }))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.cur_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        while !self.cur_token_is(&TokenType::RBRACE) && !self.cur_token_is(&TokenType::EOF) {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e)   => self.errors.push(e),
            }
            self.next_token();
        }

        BlockStatement { token, statements }
    }

    fn parse_infix(&mut self, left: Expression) -> Result<Expression, ParseError> {
        match &self.cur_token {
            TokenType::PLUS  | TokenType::MINUS | TokenType::MUL  |
            TokenType::DIV   | TokenType::EQ    | TokenType::NEQ  |
            TokenType::LT    | TokenType::GT   => self.parse_infix_expression(left),
            TokenType::LBRACKET => self.parse_index_expression(left),
            TokenType::DOT => self.parse_dot_expression(left),
            TokenType::LPAREN => self.parse_call_expression(left),
                other => Err(ParseError::new(format!(
                "No infix parse function for '{:?}' found!", other
            ))),
        }
    }

    fn parse_dot_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone(); // the '.' token
        self.next_token(); // move to method name

        let method_name = self.cur_token.get_literal();
        let string_token = TokenType::STRING(method_name.clone());

        Ok(Expression::Index(IndexExpression {
            token,
            left: Box::new(left),
            index: Box::new(Expression::StringLiteral(StringLiteral {
                token: string_token,
                value: method_name,
            })),
        }))
    }

    fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();

        self.next_token();
        let index = self.parse_expression(Lowest)?;
        self.expect_peek(TokenType::RBRACKET)?;
        Ok(Expression::Index(IndexExpression {
            token, left: Box::new(left), index: Box::new(index)
        }))
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let arguments = self.parse_expression_list(TokenType::RPAREN)?;

        Ok(Expression::Call( CallExpression {
            token,
            function: Box::new(function),
            arguments,
        } ))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let token    = self.cur_token.clone();
        let operator = self.cur_token.get_literal();
        let prec     = self.cur_precedence();
        self.next_token();
        let right    = self.parse_expression(prec)?;
        Ok(Expression::Infix(InfixExpression {
            token,
            left:  Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }
}