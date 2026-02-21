use crate::ast::{Boolean, ExpressionStatement, Identifier, IntegerLiteral, Node, PrefixExpression};
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_let_statements() {
    let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();

    assert_eq!(program.statements.len(), 3);

    let expected_identifiers = vec!["x", "y", "foobar"];

    for (i, expected) in expected_identifiers.iter().enumerate() {
        let stmt = &program.statements[i];
        test_let_statement(stmt, expected);
    }
}

fn test_let_statement(stmt: &Box<dyn crate::ast::Statement>, expected_name: &str) {
    let let_stmt = stmt
        .as_any()
        .downcast_ref::<crate::ast::LetStatement>()
        .expect("Expected a LetStatement");

    assert_eq!(
        let_stmt.name.as_ref().expect("name should be Some").value,
        expected_name
    );

    assert_eq!(let_stmt.token_literal(), "LET");
}


#[test]
fn test_return_statements() {
    let input = "return 5;
return 10;
return 993322;";

    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();

    assert_eq!(program.statements.len(), 3);

    for (_, stmt) in program.statements.iter().enumerate() {
        assert_eq!(stmt.token_literal(), String::from("RETURN"))
    }

}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();

    assert_eq!(program.statements.len(), 1);

    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected ExpressionStatement");

    let ident = stmt.expression
        .as_ref()
        .expect("Expected expression")
        .as_any()
        .downcast_ref::<Identifier>()
        .expect("Expected Identifier");

    assert_eq!(ident.value, "foobar");
    assert_eq!(ident.token_literal(), "foobar");
}

#[test]
fn test_integer_expression() {
    let input = "5;";

    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();

    assert_eq!(program.statements.len(), 1);

    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected ExpressionStatement");

    let integer = stmt.expression
        .as_ref()
        .expect("Expected expression")
        .as_any()
        .downcast_ref::<IntegerLiteral>()
        .expect("Expected Identifier");

    assert_eq!(integer.value, 5);
    assert_eq!(integer.token_literal(), "5");
}

#[test]
fn test_prefix_expressions() {
    let tests = vec![
        ("!5;", "!", 5i64),
        ("-15;", "-", 15i64),
    ];

    for (input, expected_operator, expected_value) in tests {
        let mut l = Lexer::from_str(input);
        l.convert_to_tokens();
        let mut parser = Parser::new(l);
        let program = parser.parse_program();
        parser.check_parser_errors();

        assert_eq!(program.statements.len(), 1, "input: {}", input);

        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected ExpressionStatement");

        let prefix = stmt.expression
            .as_ref()
            .expect("Expected expression")
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .expect("Expected PrefixExpression");

        assert_eq!(prefix.operator, expected_operator, "input: {}", input);

        let right = prefix.right
            .as_ref()
            .expect("Expected right expression")
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .expect("Expected IntegerLiteral");

        assert_eq!(right.value, expected_value, "input: {}", input);
    }
}


#[test]
fn test_infix_expressions() {
    let tests = vec![
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
    ];

    for (input, expected) in tests {
        let mut l = Lexer::from_str(input);
        l.convert_to_tokens();
        let mut parser = Parser::new(l);
        let program = parser.parse_program();
        parser.check_parser_errors();

        assert_eq!(program.string().trim(), expected, "input: {}", input);
    }
}

#[test]
fn test_bool_expression() {
    let input = "false;";

    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();

    assert_eq!(program.statements.len(), 1);

    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected ExpressionStatement");

    let boolean = stmt.expression
        .as_ref()
        .expect("Expected expression")
        .as_any()
        .downcast_ref::<Boolean>()
        .expect("Expected Identifier");

    assert_eq!(boolean.value, false);
    assert_eq!(boolean.token_literal(), "FALSE");
}
