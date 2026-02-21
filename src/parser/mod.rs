use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::token::TokenType;
use crate::ast::{Boolean, Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program, ReturnStatement, Statement};
use crate::parser::Precedence::{EQUALS, LESSGREATER, LOWEST, PRODUCT, SUM};

#[derive(PartialOrd, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}


type PrefixParseFn = fn(&mut Parser) -> Box<dyn Expression>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Box<dyn Expression>;
pub struct Parser {
    pos: usize,
    peek_pos: usize,
    tokens: Vec<TokenType>,
    cur_token: TokenType,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
    precedences: HashMap<TokenType, Precedence>
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let tokens = l.get_tokens();
        let cur_token = tokens.get(0).cloned().unwrap_or(TokenType::ILLEGAL);
        let mut p =  Parser {
            pos: 0,
            peek_pos: 0,
            tokens,
            cur_token,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
            precedences: HashMap::from([
                (TokenType::EQ, EQUALS),
                (TokenType::NEQ, EQUALS),
                (TokenType::LT, LESSGREATER),
                (TokenType::GT, LESSGREATER),
                (TokenType::PLUS, SUM),
                (TokenType::MINUS, SUM),
                (TokenType::DIV, PRODUCT),
                (TokenType::MUL, PRODUCT),
            ])
        };
        p.next_token();

        p.register_prefix(TokenType::IDENT(String::new()), Parser::parse_identifier);
        p.register_prefix(TokenType::INT(String::new()), Parser::parse_integer_literal);
        p.register_prefix(TokenType::BANG, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::TRUE, Parser::parse_boolean);
        p.register_prefix(TokenType::FALSE, Parser::parse_boolean);

        p.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
        p.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
        p.register_infix(TokenType::DIV, Parser::parse_infix_expression);
        p.register_infix(TokenType::MUL, Parser::parse_infix_expression);
        p.register_infix(TokenType::EQ, Parser::parse_infix_expression);
        p.register_infix(TokenType::NEQ, Parser::parse_infix_expression);
        p.register_infix(TokenType::LT, Parser::parse_infix_expression);
        p.register_infix(TokenType::GT, Parser::parse_infix_expression);

        p
    }

    pub fn peek_precedence(&self) -> Precedence {
        self.precedences.get(&self.tokens[self.peek_pos])
            .copied()
            .unwrap_or(LOWEST)
    }

    pub fn cur_precedence(&self) -> Precedence {
        self.precedences.get(&self.cur_token)
            .copied()
            .unwrap_or(LOWEST)
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn register_prefix(&mut self, token_type: TokenType, fct: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, fct);
    }

    pub fn register_infix(&mut self, token_type: TokenType, fct: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, fct);
    }

    pub fn peek_error(&mut self, t: &TokenType) {
        let msg = format!("Expected next token to be {} but got {} instead!", t.get_literal(), self.tokens[self.peek_pos].get_literal());
        self.errors.push(msg)
    }

    pub fn check_parser_errors(&self) {
        if self.errors().len() == 0 {
            return
        }

        print!("Parser has encountered {} errors!", self.errors().len());
        for error in &self.errors {
            println!("{}", error)
        }
        panic!("Parser Error Checks found errors!")
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    pub fn no_prefix_parse_fn_error(&mut self, token_type: &TokenType) {
        self.errors.push(format!("No PrefixParseFn for {:?} found!", token_type))
    }

    fn next_token(&mut self) {
        self.cur_token = self.tokens
            .get(self.peek_pos)
            .cloned()
            .unwrap_or(TokenType::ILLEGAL);
        self.pos = self.peek_pos;
        self.peek_pos += 1;
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        token_type == self.cur_token
    }

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        match self.tokens.get(self.peek_pos) {
            Some(tok) => std::mem::discriminant(tok) == std::mem::discriminant(token_type),
            None => false,
        }
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            true
        }else{
            self.peek_error(&token_type);
            false
        }
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token {
            TokenType::LET => self.parse_let_statement().map(|s| Box::new(s) as Box<dyn Statement>),
            TokenType::RETURN => self.parse_return_statement().map(|s| Box::new(s) as Box<dyn Statement>),
            _ => self.parse_expression_statement().map(|s| Box::new(s) as Box<dyn Statement>)
        }
    }

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST);

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(ExpressionStatement { token, expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        if matches!(self.cur_token, TokenType::SEMICOLON | TokenType::EOF | TokenType::ILLEGAL) {
            return None;
        }

        let prefix = self.prefix_parse_fns.iter().find(|(k, _)| {
            std::mem::discriminant(*k) == std::mem::discriminant(&self.cur_token)
        }).map(|(_, v)| *v);

        let mut left = match prefix {
            None => {
                self.no_prefix_parse_fn_error(&self.cur_token.clone());
                return None;
            }
            Some(prefix_fn) => prefix_fn(self),
        };

        while !self.peek_token_is(&TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            let infix = self.infix_parse_fns.iter().find(|(k, _)| {
                std::mem::discriminant(*k) == std::mem::discriminant(&self.tokens[self.peek_pos])
            }).map(|(_, v)| *v);

            match infix {
                None => return Some(left),
                Some(infix_fn) => {
                    self.next_token();
                    left = infix_fn(self, left);
                }
            }
        }

        Some(left)
    }

    fn parse_identifier(p: &mut Parser) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: p.cur_token.clone(),
            value: p.cur_token.get_literal()
        })
    }

    fn parse_integer_literal(p: &mut Parser) -> Box<dyn Expression> {
        let token = p.cur_token.clone();
        let num: i64 = p.cur_token.get_literal().parse().expect("REASON");

        Box::new(
            IntegerLiteral{ token, value: num }
        )
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let token = self.cur_token.clone();

        self.next_token();

        let return_value = self.parse_expression(LOWEST);

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(ReturnStatement { token, return_value })
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::IDENT(String::new())) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.get_literal(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        self.next_token(); // move past '='

        let value = self.parse_expression(LOWEST);

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(LetStatement { token, name: Some(name), value })
    }

    pub fn parse_prefix_expression(p: &mut Parser) -> Box<dyn Expression> {
        let token = p.cur_token.clone();
        let operator = p.cur_token.get_literal();

        p.next_token();

        let right = p.parse_expression(Precedence::PREFIX);

        Box::new(
            PrefixExpression {
                token,
                operator,
                right
            }
        )
    }

    pub fn parse_infix_expression(p: &mut Parser, left: Box<dyn Expression>) -> Box<dyn Expression> {
        let token = p.cur_token.clone();
        let operator = p.cur_token.get_literal();

        let pcd = p.cur_precedence();
        p.next_token();
        let right = p.parse_expression(pcd);

        Box::new(InfixExpression {
            token,
            left: Some(left),
            operator,
            right,
        })
    }

    pub fn parse_boolean(p: &mut Parser) -> Box<dyn Expression> {
        Box::new(
            Boolean {
                token: p.cur_token.clone(),
                value: p.cur_token_is(TokenType::TRUE)
            }
        )
    }
}
