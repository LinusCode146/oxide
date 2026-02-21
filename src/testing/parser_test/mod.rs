use crate::ast::{ExpressionStatement, Identifier, Node};
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
