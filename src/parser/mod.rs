use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::token::TokenType;
use crate::ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement};

type PrefixParseFn = fn() -> dyn Expression;
type InfixParseFn = fn(dyn Expression) -> dyn Expression;

pub struct Parser {
    pos: usize,
    peek_pos: usize,
    tokens: Vec<TokenType>,
    cur_token: TokenType,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>
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
        };
        p.next_token();
        p
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
        panic!("iuiuiu")
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token != TokenType::EOF {
            println!("loop cur_token: {}", self.cur_token.get_literal());
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
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
            _ => None
        }
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let token = self.cur_token.clone();

        self.next_token();

        while !self.cur_token_is(TokenType::SEMICOLON) {
            if self.cur_token_is(TokenType::EOF) { return None; }
            self.next_token();
        };

        Some(ReturnStatement { token, return_value: None})
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

        while !self.cur_token_is(TokenType::SEMICOLON) {
            if self.cur_token_is(TokenType::EOF) { return None; }
            self.next_token();
        }

        Some(LetStatement { token, name: Some(name), value: None })
    }
}