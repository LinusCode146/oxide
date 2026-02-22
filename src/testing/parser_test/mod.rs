use crate::ast::{Expression, Node, Statement};
use crate::lexer::Lexer;
use crate::parser::Parser;

fn parse(input: &str) -> crate::ast::Program {
    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();
    program
}

// ── Let statements ────────────────────────────────────────────────────────────

#[test]
fn test_let_statements() {
    let program = parse("
let x = 5;
let y = 10;
let foobar = 838383;
");
    assert_eq!(program.statements.len(), 3);

    let expected = ["x", "y", "foobar"];
    for (stmt, name) in program.statements.iter().zip(expected.iter()) {
        let Statement::Let(let_stmt) = stmt else {
            panic!("Expected LetStatement, got something else");
        };
        assert_eq!(let_stmt.token_literal(), "let");
        assert_eq!(let_stmt.name.value, *name);
    }
}

// ── Return statements ─────────────────────────────────────────────────────────

#[test]
fn test_return_statements() {
    let program = parse("return 5;
return 10;
return 993322;");

    assert_eq!(program.statements.len(), 3);
    for stmt in &program.statements {
        assert!(
            matches!(stmt, Statement::Return(_)),
            "Expected ReturnStatement"
        );
        assert_eq!(stmt.token_literal(), "return");
    }
}

// ── Identifier expression ─────────────────────────────────────────────────────

#[test]
fn test_identifier_expression() {
    let program = parse("foobar;");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::Identifier(ident) = &stmt.expression else {
        panic!("Expected Identifier");
    };

    assert_eq!(ident.value, "foobar");
    assert_eq!(ident.token_literal(), "foobar");
}

// ── Integer literal ───────────────────────────────────────────────────────────

#[test]
fn test_integer_expression() {
    let program = parse("5;");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::IntegerLiteral(int) = &stmt.expression else {
        panic!("Expected IntegerLiteral");
    };

    assert_eq!(int.value, 5);
    assert_eq!(int.token_literal(), "5");
}

// ── Prefix expressions ────────────────────────────────────────────────────────

#[test]
fn test_prefix_expressions() {
    let tests = vec![
        ("!5;",   "!", 5i64),
        ("-15;", "-", 15i64),
    ];

    for (input, expected_op, expected_val) in tests {
        let program = parse(input);
        assert_eq!(program.statements.len(), 1, "input: {}", input);

        let Statement::Expression(stmt) = &program.statements[0] else {
            panic!("Expected ExpressionStatement for input: {}", input);
        };
        let Expression::Prefix(prefix) = &stmt.expression else {
            panic!("Expected PrefixExpression for input: {}", input);
        };

        assert_eq!(prefix.operator, expected_op, "input: {}", input);

        let Expression::IntegerLiteral(int) = prefix.right.as_ref() else {
            panic!("Expected IntegerLiteral on right for input: {}", input);
        };
        assert_eq!(int.value, expected_val, "input: {}", input);
    }
}

// ── Infix expressions ─────────────────────────────────────────────────────────

#[test]
fn test_infix_expressions() {
    let tests = vec![
        ("a + b / c",                               "(a + (b / c))"),
        ("a + b * c + d / e - f",                   "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5",                           "(3 + 4)((-5) * 5)"),
        ("5 > 4 == true",                           "((5 > 4) == true)"),
        ("5 < 4 != 3 > 4",                          "((5 < 4) != (3 > 4))"),
        ("3 + 4 * 5 == 3 * 1 + 4 * 5",             "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
    ];

    for (input, expected) in tests {
        let program = parse(input);
        assert_eq!(program.string().trim(), expected, "input: {}", input);
    }
}

// ── Grouped expressions ───────────────────────────────────────────────────────

#[test]
fn test_grouped_expressions() {
    let tests = vec![
        ("(5 + 5) * 2",        "((5 + 5) * 2)"),
        ("!(true == true)",    "(!(true == true))"),
        ("-(5 + 5)",           "(-(5 + 5))"),
    ];

    for (input, expected) in tests {
        let program = parse(input);
        assert_eq!(program.string().trim(), expected, "input: {}", input);
    }
}

// ── Boolean expression ────────────────────────────────────────────────────────

#[test]
fn test_bool_expression() {
    let program = parse("true;");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::Boolean(boolean) = &stmt.expression else {
        panic!("Expected BooleanLiteral");
    };

    assert_eq!(boolean.value, true);
    assert_eq!(boolean.token_literal(), "true");
}

// ── If expression — no else ───────────────────────────────────────────────────

#[test]
fn test_if_expression() {
    let program = parse("if (x < y) { x }");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::If(if_expr) = &stmt.expression else {
        panic!("Expected IfExpression");
    };

    // condition: x < y
    let Expression::Infix(condition) = if_expr.condition.as_ref() else {
        panic!("Expected InfixExpression as condition");
    };
    assert_eq!(condition.operator, "<");

    let Expression::Identifier(left) = condition.left.as_ref() else {
        panic!("Expected Identifier on left of condition");
    };
    assert_eq!(left.value, "x");

    let Expression::Identifier(right) = condition.right.as_ref() else {
        panic!("Expected Identifier on right of condition");
    };
    assert_eq!(right.value, "y");

    // consequence: { x }
    assert_eq!(if_expr.consequence.statements.len(), 1);
    let Statement::Expression(cons_stmt) = &if_expr.consequence.statements[0] else {
        panic!("Expected ExpressionStatement in consequence");
    };
    let Expression::Identifier(cons_ident) = &cons_stmt.expression else {
        panic!("Expected Identifier in consequence");
    };
    assert_eq!(cons_ident.value, "x");

    // no alternative
    assert!(if_expr.alternative.is_none());
}

// ── If-else expression ────────────────────────────────────────────────────────

#[test]
fn test_if_else_expression() {
    let program = parse("if (x < y) { x } else { y }");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::If(if_expr) = &stmt.expression else {
        panic!("Expected IfExpression");
    };

    // condition
    let Expression::Infix(condition) = if_expr.condition.as_ref() else {
        panic!("Expected InfixExpression as condition");
    };
    assert_eq!(condition.operator, "<");

    // consequence: { x }
    assert_eq!(if_expr.consequence.statements.len(), 1);
    let Statement::Expression(cons_stmt) = &if_expr.consequence.statements[0] else {
        panic!("Expected ExpressionStatement in consequence");
    };
    let Expression::Identifier(cons_ident) = &cons_stmt.expression else {
        panic!("Expected Identifier in consequence");
    };
    assert_eq!(cons_ident.value, "x");

    // alternative: { y }
    let alt = if_expr.alternative.as_ref().expect("Expected alternative block");
    assert_eq!(alt.statements.len(), 1);
    let Statement::Expression(alt_stmt) = &alt.statements[0] else {
        panic!("Expected ExpressionStatement in alternative");
    };
    let Expression::Identifier(alt_ident) = &alt_stmt.expression else {
        panic!("Expected Identifier in alternative");
    };
    assert_eq!(alt_ident.value, "y");
}

// ── If expression string representation ──────────────────────────────────────

#[test]
fn test_if_expression_string_representation() {
    let tests = vec![
        ("if (x < y) { x }",              "if (x < y) { x }"),
        ("if (x < y) { x } else { y }",   "if (x < y) { x } else { y }"),
    ];

    for (input, expected) in tests {
        let program = parse(input);
        assert_eq!(program.string().trim(), expected, "input: {}", input);
    }
}