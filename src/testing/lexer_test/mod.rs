use crate::lexer::Lexer;
use crate::token::TokenType;

fn lex_string(input: &str) -> Vec<TokenType> {
    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    l.get_tokens().to_vec()
}

fn lex_file(filepath: &str) -> Vec<TokenType> {
    let mut l = Lexer::new(filepath.to_string()).unwrap();
    l.convert_to_tokens();
    l.get_tokens().to_vec()
}

#[test]
fn test_variable_assignment() {
    let tokens = lex_string("let five = 5;");
    assert_eq!(*tokens, vec![
        TokenType::IDENT("let".to_string()),
        TokenType::IDENT("five".to_string()),
        TokenType::ASSIGN,
        TokenType::INT("5".to_string()),
        TokenType::SEMICOLON,
    ]);
}

#[test]
fn test_function_call() {
    let tokens = lex_string("add(3, 4);");
    assert_eq!(*tokens, vec![
        TokenType::IDENT("add".to_string()),
        TokenType::LPAREN,
        TokenType::INT("3".to_string()),
        TokenType::COMMA,
        TokenType::INT("4".to_string()),
        TokenType::RPAREN,
        TokenType::SEMICOLON,
    ]);
}

#[test]
fn test_equality_operators() {
    let tokens = lex_string("five == 5\nx != y");
    assert_eq!(*tokens, vec![
        TokenType::IDENT("five".to_string()),
        TokenType::EQ,
        TokenType::INT("5".to_string()),
        TokenType::IDENT("x".to_string()),
        TokenType::NEQ,
        TokenType::IDENT("y".to_string()),
    ]);
}



#[test]
fn test_file1_string_and_semicolons() {
    let tokens = lex_file("src/testing/lexer_test/test1.oxide");
    assert!(tokens.contains(&TokenType::STRING("hello world".to_string())));
    assert!(tokens.contains(&TokenType::SEMICOLON));
    assert!(!tokens.contains(&TokenType::ILLEGAL));
}

#[test]
fn test_file2_arithmetic_expression() {
    let tokens = lex_file("src/testing/lexer_test/test2.oxide");
    assert!(tokens.contains(&TokenType::PLUS));
    assert!(tokens.contains(&TokenType::MINUS));
    assert!(tokens.contains(&TokenType::MUL));
    assert!(tokens.contains(&TokenType::DIV));
    assert!(!tokens.contains(&TokenType::ILLEGAL));
}

#[test]
fn test_file3_no_illegal_tokens() {
    let tokens = lex_file("src/testing/lexer_test/test3.oxide");
    assert!(!tokens.contains(&TokenType::ILLEGAL));
}

#[test]
fn test_file1_token_sequence() {
    let tokens = lex_file("src/testing/lexer_test/test1.oxide");
    // verify let binding sequence appears somewhere in the token stream
    let windows = tokens.windows(3);
    let has_let_binding = windows.into_iter().any(|w| matches!(
        w,
        [TokenType::IDENT(_), TokenType::ASSIGN, TokenType::INT(_)]
    ));
    assert!(has_let_binding, "Expected a let binding pattern in test1");
}

#[test]
fn test_file2_balanced_parens() {
    let tokens = lex_file("src/testing/lexer_test/test2.oxide");
    let lparen_count = tokens.iter().filter(|t| **t == TokenType::LPAREN).count();
    let rparen_count = tokens.iter().filter(|t| **t == TokenType::RPAREN).count();
    assert_eq!(lparen_count, rparen_count, "Parentheses should be balanced");
}

#[test]
fn test_file3_balanced_braces() {
    let tokens = lex_file("src/testing/lexer_test/test3.oxide");
    let lbrace_count = tokens.iter().filter(|t| **t == TokenType::LBRACE).count();
    let rbrace_count = tokens.iter().filter(|t| **t == TokenType::RBRACE).count();
    assert_eq!(lbrace_count, rbrace_count, "Braces should be balanced");
}

#[test]
fn test_file2_comparison_operators() {
    let tokens = lex_file("src/testing/lexer_test/test2.oxide");
    assert!(
        tokens.contains(&TokenType::EQ) || tokens.contains(&TokenType::NEQ)
            || tokens.contains(&TokenType::GT) || tokens.contains(&TokenType::LT),
        "Expected at least one comparison operator in test2"
    );
}