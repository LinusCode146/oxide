use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
#[derive(Eq, Hash)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(String),
    STRING(String),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    MUL,
    DIV,
    BANG,
    LT,
    GT,
    EQ,
    NEQ,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COLON,
    DOT,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN
}

impl TokenType {
    pub fn get_literal(&self) -> String {
        match self {
            TokenType::ILLEGAL => String::from("ILLEGAL"),
            TokenType::EOF => String::from("EOF"),
            TokenType::IDENT(name) => format!("{}", name),
            TokenType::INT(num) => format!("{}", num),
            TokenType::STRING(name) => format!("{}", name),

            TokenType::ASSIGN => String::from("="),
            TokenType::PLUS => String::from("+"),
            TokenType::MINUS => String::from("-"),
            TokenType::MUL => String::from("*"),
            TokenType::DIV => String::from("/"),
            TokenType::BANG => String::from("!"),
            TokenType::LT => String::from("<"),
            TokenType::GT => String::from(">"),
            TokenType::EQ => String::from("=="),
            TokenType::NEQ => String::from("!="),
            TokenType::COMMA => String::from(","),
            TokenType::SEMICOLON => String::from(";"),
            TokenType::LPAREN => String::from("("),
            TokenType::RPAREN => String::from(")"),
            TokenType::LBRACE => String::from("{"),
            TokenType::RBRACE => String::from("}"),
            TokenType::LBRACKET => String::from("["),
            TokenType::RBRACKET => String::from("]"),
            TokenType::COLON => String::from(":"),
            TokenType::DOT => String::from("."),

            TokenType::FUNCTION => String::from("fun"),
            TokenType::LET => String::from("let"),
            TokenType::TRUE => String::from("true"),
            TokenType::FALSE => String::from("false"),
            TokenType::IF => String::from("if"),
            TokenType::ELSE => String::from("else"),
            TokenType::RETURN => String::from("return"),
        }
    }
}

pub fn lookup_keyword(ident: &str) -> TokenType {
    let keywords = HashMap::from([
        ("let", TokenType::LET),
        ("fn", TokenType::FUNCTION),
        ("true", TokenType::TRUE),
        ("false", TokenType::FALSE),
        ("if", TokenType::IF),
        ("else", TokenType::ELSE),
        ("return", TokenType::RETURN),
    ]);
    keywords.get(ident).cloned().unwrap_or(TokenType::IDENT(ident.to_string()))
}