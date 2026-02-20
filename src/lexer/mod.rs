use std::error::Error;
use std::fs;
use super::token::{TokenType};

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
    tokens: Vec<TokenType>,
}

impl Lexer {

    pub fn reset(&mut self) {
        self.position = 0;
        self.read_position = 0;
        self.tokens.clear();
        self.read_char();
    }
    pub fn new(filepath: String) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read_to_string(filepath)?;

        let mut l = Self {
            input: Vec::from(file_content.as_bytes()),
            position: 0,
            read_position: 0,
            ch: 0,
            tokens: Vec::new(),
        };
        l.read_char();
        Ok(l)
    }

    pub fn from_str(input: &str) -> Self {
        let mut l = Self {
            input: input.as_bytes().to_vec(),
            position: 0,
            read_position: 0,
            ch: 0,
            tokens: Vec::new(),
        };
        l.read_char();
        l
    }

    pub fn get_tokens(&self) -> Vec<TokenType> {
        self.tokens.clone()
    }

    pub fn print_file(&mut self) {
        println!("------- Start of File ---------");
        while self.position < self.input.len() && self.ch != 0 {
            print!("{}", self.ch as char);
            self.read_char();
        }
        println!(" ");
        println!("------- End of File ------------");
        self.reset();
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }


    fn peek_token(&self) -> Option<u8> {
        if self.read_position < self.input.len() {
            Some(self.input[self.read_position])
        } else {
            None
        }
    }

    fn read_identifier(&mut self) -> Vec<u8> {
        let mut ident = Vec::new();
        while (self.ch as char).is_alphabetic() || self.ch == b'_' {
            ident.push(self.ch);
            self.read_char();
        }
        ident
    }

    fn read_number(&mut self) -> Vec<u8> {
        let mut number = Vec::new();
        while (self.ch as char).is_numeric() || self.ch == b'_' {
            number.push(self.ch);
            self.read_char();
        }
        number
    }

    fn read_string(&mut self) -> Vec<u8> {
        self.read_char(); // skip opening "
        let mut string = Vec::new();
        while self.ch != b'"' && self.ch != 0 {
            string.push(self.ch);
            self.read_char();
        }
        string
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\r' || self.ch == b'\n' {
            self.read_char()
        }
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("Token with literal: {}", token.get_literal());
        }
        if self.tokens.contains(&TokenType::ILLEGAL) {
            println!(" ");
            println!("Illegal Token! Program will panic");
        } else {
            println!(" ");
            println!("Program got lexed correctly!");
        }
    }

    pub fn convert_to_tokens(&mut self) {
        self.skip_whitespaces();

        while self.ch != 0 {
            let next_token = match self.ch {
                b'"' => {
                    let string = String::from_utf8(self.read_string()).unwrap();
                    TokenType::STRING(string)
                }
                b';' => TokenType::SEMICOLON,
                b':' => TokenType::COLON,
                b'(' => TokenType::LPAREN,
                b')' => TokenType::RPAREN,
                b'{' => TokenType::LBRACE,
                b'}' => TokenType::RBRACE,
                b'[' => TokenType::LBRACKET,
                b']' => TokenType::RBRACKET,
                b'.' => TokenType::DOT,
                b'+' => TokenType::PLUS,
                b'-' => TokenType::MINUS,
                b'>' => TokenType::GT,
                b'<' => TokenType::LT,
                b'*' => TokenType::MUL,
                b'/' => TokenType::DIV,
                b',' => TokenType::COMMA,
                b'=' => match self.peek_token() {
                    Some(b'=') => { self.read_char(); TokenType::EQ },
                    Some(b' ') | Some(b'\n') | Some(b'\t') | Some(b'\r') => TokenType::ASSIGN,
                    Some(b'0'..=b'9') => TokenType::ASSIGN,
                    None => TokenType::ASSIGN,
                    Some(_) => TokenType::ILLEGAL,
                }
                b'!' => match self.peek_token() {
                    Some(b'=') => { self.read_char(); TokenType::NEQ },
                    Some(b' ') | Some(b'\n') | Some(b'\t') | Some(b'\r') => TokenType::BANG,
                    None => TokenType::BANG,
                    Some(_) => TokenType::ILLEGAL,
                }

                _ => {
                    if (self.ch as char).is_alphabetic() {
                        let ident = String::from_utf8(self.read_identifier()).unwrap();
                        self.tokens.push(TokenType::IDENT(ident));
                        self.skip_whitespaces();
                        continue; // skip the read_char at the bottom
                    } else if (self.ch as char).is_numeric() {
                        let num = String::from_utf8(self.read_number()).unwrap();
                        self.tokens.push(TokenType::INT(num));
                        self.skip_whitespaces();
                        continue;
                    } else {
                        self.tokens.push(TokenType::EOF);
                        continue;
                    }
                }
            };
            self.tokens.push(next_token);
            self.read_char();
            self.skip_whitespaces();
        }
    }
}