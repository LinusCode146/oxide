use crate::ast::{Expression, Node, Statement, Program};
use crate::lexer::Lexer;
use crate::parser::Parser;

fn parse(input: &str) -> Program {
    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();
    program
}


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

    assert_eq!(if_expr.consequence.statements.len(), 1);
    let Statement::Expression(cons_stmt) = &if_expr.consequence.statements[0] else {
        panic!("Expected ExpressionStatement in consequence");
    };
    let Expression::Identifier(cons_ident) = &cons_stmt.expression else {
        panic!("Expected Identifier in consequence");
    };
    assert_eq!(cons_ident.value, "x");

    assert!(if_expr.alternative.is_none());
}


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

    let Expression::Infix(condition) = if_expr.condition.as_ref() else {
        panic!("Expected InfixExpression as condition");
    };
    assert_eq!(condition.operator, "<");

    assert_eq!(if_expr.consequence.statements.len(), 1);
    let Statement::Expression(cons_stmt) = &if_expr.consequence.statements[0] else {
        panic!("Expected ExpressionStatement in consequence");
    };
    let Expression::Identifier(cons_ident) = &cons_stmt.expression else {
        panic!("Expected Identifier in consequence");
    };
    assert_eq!(cons_ident.value, "x");

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


#[test]
fn test_function_literal() {
    let program = parse("fun(x, y) { x + y; }");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::FunctionLiteral(func) = &stmt.expression else {
        panic!("Expected FunctionLiteral");
    };

    assert_eq!(func.parameters.len(), 2);
    assert_eq!(func.parameters[0].value, "x");
    assert_eq!(func.parameters[1].value, "y");

    assert_eq!(func.body.statements.len(), 1);
    let Statement::Expression(body_stmt) = &func.body.statements[0] else {
        panic!("Expected ExpressionStatement in body");
    };
    let Expression::Infix(infix) = &body_stmt.expression else {
        panic!("Expected InfixExpression in body");
    };
    assert_eq!(infix.operator, "+");

    let Expression::Identifier(left) = infix.left.as_ref() else {
        panic!("Expected Identifier on left of body infix");
    };
    assert_eq!(left.value, "x");

    let Expression::Identifier(right) = infix.right.as_ref() else {
        panic!("Expected Identifier on right of body infix");
    };
    assert_eq!(right.value, "y");
}


#[test]
fn test_function_literal_parameters() {
    let tests = vec![
        ("fun() {}",        vec![]),
        ("fun(x) {}",       vec!["x"]),
        ("fun(x, y, z) {}", vec!["x", "y", "z"]),
    ];

    for (input, expected_params) in tests {
        let program = parse(input);

        let Statement::Expression(stmt) = &program.statements[0] else {
            panic!("Expected ExpressionStatement for input: {}", input);
        };
        let Expression::FunctionLiteral(func) = &stmt.expression else {
            panic!("Expected FunctionLiteral for input: {}", input);
        };

        assert_eq!(func.parameters.len(), expected_params.len(), "input: {}", input);
        for (param, expected) in func.parameters.iter().zip(expected_params.iter()) {
            assert_eq!(param.value, *expected, "input: {}", input);
        }
    }
}


#[test]
fn test_function_literal_string_representation() {
    let tests = vec![
        ("fun(x, y) { x + y; }", "fun(x, y) { (x + y) }"),
        ("fun() { 5; }",         "fun() { 5 }"),
    ];

    for (input, expected) in tests {
        let program = parse(input);
        assert_eq!(program.string().trim(), expected, "input: {}", input);
    }
}


#[test]
fn test_call_expression() {
    let program = parse("add(1, 2 * 3, 4 + 5);");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::Call(call) = &stmt.expression else {
        panic!("Expected CallExpression");
    };

    let Expression::Identifier(func) = call.function.as_ref() else {
        panic!("Expected Identifier as function");
    };
    assert_eq!(func.value, "add");

    assert_eq!(call.arguments.len(), 3);

    let Expression::IntegerLiteral(arg0) = call.arguments[0].as_ref() else {
        panic!("Expected IntegerLiteral as first argument");
    };
    assert_eq!(arg0.value, 1);

    let Expression::Infix(arg1) = call.arguments[1].as_ref() else {
        panic!("Expected InfixExpression as second argument");
    };
    assert_eq!(arg1.operator, "*");

    let Expression::Infix(arg2) = call.arguments[2].as_ref() else {
        panic!("Expected InfixExpression as third argument");
    };
    assert_eq!(arg2.operator, "+");
}


#[test]
fn test_call_expression_no_arguments() {
    let program = parse("foo();");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::Call(call) = &stmt.expression else {
        panic!("Expected CallExpression");
    };

    let Expression::Identifier(func) = call.function.as_ref() else {
        panic!("Expected Identifier as function");
    };
    assert_eq!(func.value, "foo");
    assert_eq!(call.arguments.len(), 0);
}


#[test]
fn test_call_expression_with_function_literal() {
    let program = parse("fun(x, y) { x + y; }(2, 3);");
    assert_eq!(program.statements.len(), 1);

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!("Expected ExpressionStatement");
    };
    let Expression::Call(call) = &stmt.expression else {
        panic!("Expected CallExpression");
    };

    let Expression::FunctionLiteral(func) = call.function.as_ref() else {
        panic!("Expected FunctionLiteral as function");
    };
    assert_eq!(func.parameters.len(), 2);
    assert_eq!(call.arguments.len(), 2);
}


#[test]
fn test_call_expression_string_representation() {
    let tests = vec![
        ("add(1, 2 * 3, 4 + 5);", "add(1, (2 * 3), (4 + 5))"),
        ("foo();",                 "foo()"),
        ("foo(a, b);",             "foo(a, b)"),
    ];

    for (input, expected) in tests {
        let program = parse(input);
        assert_eq!(program.string().trim(), expected, "input: {}", input);
    }
}