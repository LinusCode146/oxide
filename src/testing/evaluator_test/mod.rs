use crate::evaluator::eval_program;
use crate::environment::Environment;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

fn eval(input: &str) -> Object {
    let mut l = Lexer::from_str(input);
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();
    let mut env = Environment::new();
    eval_program(&program, &mut env)
}

fn assert_integer(obj: Object, expected: i64) {
    let Object::Integer(val) = obj else {
        panic!("Expected Integer, got: {:?}", obj);
    };
    assert_eq!(val, expected);
}

fn assert_boolean(obj: Object, expected: bool) {
    let Object::Boolean(val) = obj else {
        panic!("Expected Boolean, got: {:?}", obj);
    };
    assert_eq!(val, expected);
}

fn assert_null(obj: Object) {
    assert!(matches!(obj, Object::Null), "Expected Null, got: {:?}", obj);
}

fn assert_string(obj: Object, expected: &str) {
    let Object::StringObj(val) = obj else {
        panic!("Expected StringObj, got: {:?}", obj);
    };
    assert_eq!(val, expected);
}

fn assert_error(obj: Object, expected_msg: &str) {
    let Object::Error(msg) = obj else {
        panic!("Expected Error, got: {:?}", obj);
    };
    assert_eq!(msg, expected_msg);
}

// ─── Integer Arithmetic ───────────────────────────────────────────────────────

#[test]
fn test_eval_integer_literal() {
    let tests = vec![
        ("5",   5i64),
        ("10",  10),
        ("999", 999),
        ("0",   0),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_integer_arithmetic() {
    let tests = vec![
        ("5 + 5",         10),
        ("5 - 5",         0),
        ("5 * 5",         25),
        ("10 / 2",        5),
        ("2 + 3 * 4",     14),
        ("(2 + 3) * 4",   20),
        ("10 / 2 + 3",    8),
        ("100 / 10 / 2",  5),
        ("-5 + 10",       5),
        ("-10 + -10",     -20),
        ("50 / 2 * 2 + 10", 60),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_prefix_minus() {
    let tests = vec![
        ("-5",    -5i64),
        ("-10",   -10),
        ("--5",   5),
        ("-0",    0),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

// ─── Boolean Expressions ─────────────────────────────────────────────────────

#[test]
fn test_eval_boolean_literal() {
    assert_boolean(eval("true"),  true);
    assert_boolean(eval("false"), false);
}

#[test]
fn test_eval_bang_operator() {
    let tests = vec![
        ("!true",    false),
        ("!false",   true),
        ("!!true",   true),
        ("!!false",  false),
        ("!5",       false),  // truthy integer
        ("!0",       false),  // 0 is still truthy in Monkey
    ];
    for (input, expected) in tests {
        assert_boolean(eval(input), expected);
    }
}

#[test]
fn test_eval_integer_comparisons() {
    let tests = vec![
        ("1 < 2",    true),
        ("1 > 2",    false),
        ("1 < 1",    false),
        ("1 > 1",    false),
        ("1 == 1",   true),
        ("1 != 1",   false),
        ("1 == 2",   false),
        ("1 != 2",   true),
        ("10 > 9",   true),
        ("10 < 9",   false),
    ];
    for (input, expected) in tests {
        assert_boolean(eval(input), expected);
    }
}

#[test]
fn test_eval_boolean_comparisons() {
    let tests = vec![
        ("true == true",    true),
        ("false == false",  true),
        ("true == false",   false),
        ("true != false",   true),
        ("false != true",   true),
        ("(1 < 2) == true", true),
        ("(1 > 2) == true", false),
        ("(1 < 2) == false", false),
        ("(1 > 2) == false", true),
    ];
    for (input, expected) in tests {
        assert_boolean(eval(input), expected);
    }
}

// ─── If / Else ────────────────────────────────────────────────────────────────

#[test]
fn test_eval_if_expression_truthy() {
    let tests = vec![
        ("if (true) { 10 }",              10i64),
        ("if (1) { 10 }",                 10),
        ("if (1 < 2) { 10 }",             10),
        ("if (1 < 2) { 10 } else { 20 }", 10),
        ("if (1 > 2) { 10 } else { 20 }", 20),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_if_expression_null_branch() {
    // no else branch + falsy condition → Null
    assert_null(eval("if (false) { 10 }"));
    assert_null(eval("if (1 > 2) { 10 }"));
}

// ─── Return Statements ────────────────────────────────────────────────────────

#[test]
fn test_eval_return_statement() {
    let tests = vec![
        ("return 10;",                  10i64),
        ("return 10; 9;",              10),
        ("return 2 * 5; 9;",           10),
        ("9; return 2 * 5; 9;",        10),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_return_stops_execution() {
    let input = "
if (10 > 1) {
    if (10 > 1) {
        return 10;
    }
    return 1;
}
";
    assert_integer(eval(input), 10);
}

// ─── Let Statements & Identifiers ────────────────────────────────────────────

#[test]
fn test_eval_let_statement() {
    let tests = vec![
        ("let x = 5; x",              5i64),
        ("let x = 5 * 5; x",          25),
        ("let x = 5; let y = x; y",   5),
        ("let x = 5; let y = x; let z = x + y + 5; z", 15),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_undefined_identifier() {
    assert_error(eval("foobar"), "identifier not found: foobar");
}

// ─── Strings ─────────────────────────────────────────────────────────────────

#[test]
fn test_eval_string_literal() {
    assert_string(eval(r#""hello world""#), "hello world");
    assert_string(eval(r#""""#), "");
}

#[test]
fn test_eval_string_concatenation() {
    assert_string(eval(r#""hello" + " " + "world""#), "hello world");
    assert_string(eval(r#""foo" + "bar""#), "foobar");
}

#[test]
fn test_eval_string_wrong_operator() {
    assert_error(eval(r#""a" - "b""#), "unknown operator: STRING - STRING");
    assert_error(eval(r#""a" * "b""#), "unknown operator: STRING * STRING");
}

// ─── Functions ────────────────────────────────────────────────────────────────

#[test]
fn test_eval_function_literal() {
    let input = "fun(x) { x + 2; }";
    let result = eval(input);
    let Object::Function(func) = result else {
        panic!("Expected Function, got: {:?}", result);
    };
    assert_eq!(func.parameters.len(), 1);
    assert_eq!(func.parameters[0].value, "x");
}

#[test]
fn test_eval_function_call() {
    let tests = vec![
        ("let identity = fun(x) { x; }; identity(5);",          5i64),
        ("let identity = fun(x) { return x; }; identity(5);",   5),
        ("let double = fun(x) { x * 2; }; double(5);",          10),
        ("let add = fun(x, y) { x + y; }; add(5, 5);",          10),
        ("let add = fun(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fun(x) { x; }(5)",                                     5),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_closures() {
    let input = "
let newAdder = fun(x) {
    fun(y) { x + y; };
};
let addTwo = newAdder(2);
addTwo(3);
";
    assert_integer(eval(input), 5);
}

#[test]
fn test_eval_closure_captures_environment() {
    let input = "
let x = 10;
let addX = fun(y) { x + y; };
addX(5);
";
    assert_integer(eval(input), 15);
}

#[test]
fn test_eval_recursive_function() {
    let input = "
let factorial = fun(n) {
    if (n < 2) { return 1; }
    return n * factorial(n - 1);
};
factorial(5);
";
    assert_integer(eval(input), 120);
}

#[test]
fn test_eval_wrong_number_of_arguments() {
    let input = "let f = fun(x, y) { x + y; }; f(1);";
    assert_error(eval(input), "wrong number of arguments: expected 2, got 1");
}

#[test]
fn test_eval_call_non_function() {
    assert_error(eval("let x = 5; x(1);"), "not a function: INTEGER");
}

// ─── Error Handling ───────────────────────────────────────────────────────────

#[test]
fn test_eval_type_mismatch_errors() {
    let tests = vec![
        ("5 + true",        "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;",    "type mismatch: INTEGER + BOOLEAN"),
        ("true + false",    "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
    ];
    for (input, expected_msg) in tests {
        assert_error(eval(input), expected_msg);
    }
}

#[test]
fn test_eval_errors_stop_execution() {
    // error in an if branch should still propagate and stop further evaluation
    let input = "
if (10 > 1) {
    true + false;
    return 99;
}
";
    assert_error(eval(input), "unknown operator: BOOLEAN + BOOLEAN");
}

#[test]
fn test_eval_unknown_prefix_operator() {
    // '-' on a boolean
    assert_error(eval("-true"), "unknown operator: -BOOLEAN");
}

#[test]
fn test_eval_division_by_zero() {
    assert_error(eval("10 / 0"), "division by zero");
}

// ─── Builtins ─────────────────────────────────────────────────────────────────

#[test]
fn test_log_returns_null() {
    assert_null(eval(r#"log("hello")"#));
}

#[test]
fn test_log_multiple_args_returns_null() {
    assert_null(eval(r#"log("a", "b", 42)"#));
}

#[test]
fn test_log_no_args_returns_null() {
    assert_null(eval("log()"));
}

#[test]
fn test_log_is_accessible_as_identifier() {
    let result = eval("log");
    assert!(matches!(result, Object::Builtin(_)), "Expected Builtin, got: {:?}", result);
}