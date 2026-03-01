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
        ("!5",       false),
        ("!0",       false),
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
    assert_null(eval("if (false) { 10 }"));
    assert_null(eval("if (1 > 2) { 10 }"));
}


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


#[test]
fn test_eval_array_literal() {
    let result = eval("[1, 2, 3]");
    let Object::Array(arr) = result else {
        panic!("Expected Array, got: {:?}", result);
    };
    assert_eq!(arr.elements.len(), 3);
    assert_integer(arr.elements[0].clone(), 1);
    assert_integer(arr.elements[1].clone(), 2);
    assert_integer(arr.elements[2].clone(), 3);
}

#[test]
fn test_eval_empty_array() {
    let result = eval("[]");
    let Object::Array(arr) = result else {
        panic!("Expected Array, got: {:?}", result);
    };
    assert_eq!(arr.elements.len(), 0);
}


#[test]
fn test_eval_array_index() {
    let tests = vec![
        ("[1, 2, 3][0]", 1i64),
        ("[1, 2, 3][1]", 2),
        ("[1, 2, 3][2]", 3),
        ("let a = [1, 2, 3]; a[0]", 1),
        ("let a = [1, 2, 3]; a[1 + 1]", 3),
        ("[1, 2, 3][0 + 1]", 2),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_array_index_out_of_bounds() {
    assert_null(eval("[1, 2, 3][99]"));
    assert_null(eval("[1, 2, 3][-1]"));
    assert_null(eval("[][0]"));
}

#[test]
fn test_eval_index_unsupported_type() {
    assert_error(eval("1[0]"), "index operator not supported: INTEGER");
    assert_error(eval("true[0]"), "index operator not supported: BOOLEAN");
}


#[test]
fn test_builtin_len_string() {
    let tests = vec![
        (r#"len("")"#,          0i64),
        (r#"len("hello")"#,     5),
        (r#"len("hello world")"#, 11),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_builtin_len_array() {
    let tests = vec![
        ("len([])",         0i64),
        ("len([1, 2, 3])",  3),
        ("len([1])",        1),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_builtin_len_wrong_args() {
    assert_error(eval("len()"),         "wrong number of arguments. got=0, want=1");
    assert_error(eval("len(1, 2)"),     "wrong number of arguments. got=2, want=1");
    assert_error(eval("len(1)"),        "argument to `len` not supported, got INTEGER");
    assert_error(eval("len(true)"),     "argument to `len` not supported, got BOOLEAN");
}


#[test]
fn test_builtin_first() {
    assert_integer(eval("first([1, 2, 3])"), 1);
    assert_integer(eval("first([42])"),      42);
    assert_null(eval("first([])"));
}

#[test]
fn test_builtin_first_wrong_args() {
    assert_error(eval("first()"),       "wrong number of arguments. got=0, want=1");
    assert_error(eval("first(1, 2)"),   "wrong number of arguments. got=2, want=1");
    assert_error(eval("first(1)"),      "argument to `first` not supported, got INTEGER");
}


#[test]
fn test_builtin_last() {
    assert_integer(eval("last([1, 2, 3])"), 3);
    assert_integer(eval("last([42])"),      42);
    assert_null(eval("last([])"));
}

#[test]
fn test_builtin_last_wrong_args() {
    assert_error(eval("last()"),        "wrong number of arguments. got=0, want=1");
    assert_error(eval("last(1, 2)"),    "wrong number of arguments. got=2, want=1");
    assert_error(eval("last(1)"),       "argument to `last` not supported, got INTEGER");
}


#[test]
fn test_builtin_tail() {
    let result = eval("tail([1, 2, 3])");
    let Object::Array(arr) = result else {
        panic!("Expected Array, got: {:?}", result);
    };
    assert_eq!(arr.elements.len(), 2);
    assert_integer(arr.elements[0].clone(), 2);
    assert_integer(arr.elements[1].clone(), 3);
}

#[test]
fn test_builtin_tail_single_element() {
    let result = eval("tail([1])");
    let Object::Array(arr) = result else {
        panic!("Expected Array, got: {:?}", result);
    };
    assert_eq!(arr.elements.len(), 0);
}

#[test]
fn test_builtin_tail_empty() {
    assert_null(eval("tail([])"));
}

#[test]
fn test_builtin_tail_wrong_args() {
    assert_error(eval("tail()"),        "wrong number of arguments. got=0, want=1");
    assert_error(eval("tail(1, 2)"),    "wrong number of arguments. got=2, want=1");
    assert_error(eval("tail(1)"),       "argument to `first` not supported, got INTEGER");
}


#[test]
fn test_builtin_push() {
    let result = eval("push([1, 2], 3)");
    let Object::Array(arr) = result else {
        panic!("Expected Array, got: {:?}", result);
    };
    assert_eq!(arr.elements.len(), 3);
    assert_integer(arr.elements[2].clone(), 3);
}

#[test]
fn test_builtin_push_onto_empty() {
    let result = eval("push([], 1)");
    let Object::Array(arr) = result else {
        panic!("Expected Array, got: {:?}", result);
    };
    assert_eq!(arr.elements.len(), 1);
    assert_integer(arr.elements[0].clone(), 1);
}

#[test]
fn test_builtin_push_does_not_mutate() {
    // push should return a new array, original should be unchanged
    let result = eval("let a = [1, 2]; let b = push(a, 3); len(a)");
    assert_integer(result, 2);
}

#[test]
fn test_builtin_push_wrong_args() {
    assert_error(eval("push()"),        "wrong number of arguments. got=0, want=1");
    assert_error(eval("push([])"),      "wrong number of arguments. got=1, want=1");
    assert_error(eval("push(1, 2)"),    "argument to `first` not supported, got INTEGER");
}


#[test]
fn test_builtin_map_with_closures() {
    let input = "
let map = fun(arr, f) {
    let iter = fun(arr, accumulated) {
        if len(arr) == 0 {
            accumulated
        } else {
            iter(tail(arr), push(accumulated, f(first(arr))));
        }
    };
    iter(arr, []);
};
let double = fun(x) { 2 * x };
let result = map([1, 2, 3], double);
result[2]
";
    assert_integer(eval(input), 6);
}


#[test]
fn test_eval_hash_literal() {
    let input = r#"{"one": 1, "two": 2, "three": 3}"#;
    let result = eval(input);
    let Object::Hash(hash) = result else {
        panic!("Expected Hash, got: {:?}", result);
    };
    assert_eq!(hash.pairs.len(), 3);
}

#[test]
fn test_eval_hash_integer_keys() {
    let input = r#"{1: "one", 2: "two"}"#;
    let result = eval(input);
    let Object::Hash(hash) = result else {
        panic!("Expected Hash, got: {:?}", result);
    };
    assert_eq!(hash.pairs.len(), 2);
}

#[test]
fn test_eval_hash_boolean_keys() {
    let input = "{true: 1, false: 2}";
    let result = eval(input);
    let Object::Hash(hash) = result else {
        panic!("Expected Hash, got: {:?}", result);
    };
    assert_eq!(hash.pairs.len(), 2);
}

#[test]
fn test_eval_empty_hash() {
    let result = eval("{}");
    let Object::Hash(hash) = result else {
        panic!("Expected Hash, got: {:?}", result);
    };
    assert_eq!(hash.pairs.len(), 0);
}

#[test]
fn test_eval_hash_index_string_key() {
    let tests = vec![
        (r#"{"foo": 5}["foo"]"#,           5i64),
        (r#"{"foo": 5, "bar": 10}["bar"]"#, 10),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_hash_index_integer_key() {
    let tests = vec![
        ("{1: 10, 2: 20}[1]", 10i64),
        ("{1: 10, 2: 20}[2]", 20),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_hash_index_boolean_key() {
    let tests = vec![
        ("{true: 1, false: 2}[true]",  1i64),
        ("{true: 1, false: 2}[false]", 2),
    ];
    for (input, expected) in tests {
        assert_integer(eval(input), expected);
    }
}

#[test]
fn test_eval_hash_index_missing_key() {
    assert_null(eval(r#"{"foo": 5}["bar"]"#));
    assert_null(eval("{1: 5}[2]"));
    assert_null(eval("{true: 5}[false]"));
}

#[test]
fn test_eval_hash_index_with_expression_key() {
    assert_integer(eval(r#"let key = "foo"; {"foo": 99}[key]"#), 99);
    assert_integer(eval("{1 + 1: 42}[2]"), 42);
}

#[test]
fn test_eval_hash_unusable_key() {
    assert_error(
        eval("{[1, 2]: 5}"),
        "unusable as hash key: ARRAY",
    );
}

#[test]
fn test_eval_hash_index_unusable_key() {
    assert_error(
        eval(r#"{"foo": 5}[[1]]"#),
        "unusable as hash key: ARRAY",
    );
}

// ── len ──────────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_len() {
    assert_integer(eval("[1, 2, 3].len()"), 3);
    assert_integer(eval("[].len()"), 0);
    assert_integer(eval("[42].len()"), 1);
}

#[test]
fn test_array_method_len_wrong_args() {
    assert_error(eval("[1, 2].len(1)"), "len() takes 0 arguments, got 1");
}

// ── first ─────────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_first() {
    assert_integer(eval("[1, 2, 3].first()"), 1);
    assert_integer(eval("[42].first()"), 42);
    assert_null(eval("[].first()"));
}

#[test]
fn test_array_method_first_wrong_args() {
    assert_error(eval("[1].first(1)"), "first() takes 0 arguments, got 1");
}

// ── last ──────────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_last() {
    assert_integer(eval("[1, 2, 3].last()"), 3);
    assert_integer(eval("[42].last()"), 42);
    assert_null(eval("[].last()"));
}

#[test]
fn test_array_method_last_wrong_args() {
    assert_error(eval("[1].last(1)"), "last() takes 0 arguments, got 1");
}

// ── push ──────────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_push() {
    let result = eval("[1, 2].push(3)");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 3);
    assert_integer(arr.elements[2].clone(), 3);
}

#[test]
fn test_array_method_push_onto_empty() {
    let result = eval("[].push(1)");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 1);
    assert_integer(arr.elements[0].clone(), 1);
}

#[test]
fn test_array_method_push_does_not_mutate() {
    assert_integer(eval("let a = [1, 2]; a.push(3); a.len()"), 2);
}

#[test]
fn test_array_method_push_wrong_args() {
    assert_error(eval("[1].push()"), "push() takes 1 argument, got 0");
    assert_error(eval("[1].push(1, 2)"), "push() takes 1 argument, got 2");
}

// ── tail ──────────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_tail() {
    let result = eval("[1, 2, 3].tail()");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 2);
    assert_integer(arr.elements[0].clone(), 2);
    assert_integer(arr.elements[1].clone(), 3);
}

#[test]
fn test_array_method_tail_single_element() {
    let result = eval("[1].tail()");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 0);
}

#[test]
fn test_array_method_tail_empty() {
    assert_null(eval("[].tail()"));
}

#[test]
fn test_array_method_tail_wrong_args() {
    assert_error(eval("[1].tail(1)"), "tail() takes 0 arguments, got 1");
}

// ── pop ───────────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_pop() {
    let result = eval("[1, 2, 3].pop()");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 2);
    assert_integer(arr.elements[0].clone(), 1);
    assert_integer(arr.elements[1].clone(), 2);
}

#[test]
fn test_array_method_pop_single_element() {
    let result = eval("[1].pop()");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 0);
}

#[test]
fn test_array_method_pop_empty() {
    assert_null(eval("[].pop()"));
}

#[test]
fn test_array_method_pop_does_not_mutate() {
    assert_integer(eval("let a = [1, 2, 3]; a.pop(); a.len()"), 3);
}

#[test]
fn test_array_method_pop_wrong_args() {
    assert_error(eval("[1].pop(1)"), "pop() takes 0 arguments, got 1");
}

// ── filter ────────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_filter() {
    let result = eval("[1, 2, 3, 4].filter(fun(x) { x > 2 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 2);
    assert_integer(arr.elements[0].clone(), 3);
    assert_integer(arr.elements[1].clone(), 4);
}

#[test]
fn test_array_method_filter_none_match() {
    let result = eval("[1, 2, 3].filter(fun(x) { x > 99 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 0);
}

#[test]
fn test_array_method_filter_all_match() {
    let result = eval("[1, 2, 3].filter(fun(x) { x > 0 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 3);
}

#[test]
fn test_array_method_filter_empty() {
    let result = eval("[].filter(fun(x) { x > 0 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 0);
}

#[test]
fn test_array_method_filter_wrong_args() {
    assert_error(eval("[1].filter()"), "filter() takes 1 argument, got 0");
    assert_error(eval("[1].filter(fun(x){x}, fun(x){x})"), "filter() takes 1 argument, got 2");
}

// ── filterNot ─────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_filter_not() {
    let result = eval("[1, 2, 3, 4].filterNot(fun(x) { x > 2 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 2);
    assert_integer(arr.elements[0].clone(), 1);
    assert_integer(arr.elements[1].clone(), 2);
}

#[test]
fn test_array_method_filter_not_empty() {
    let result = eval("[].filterNot(fun(x) { x > 0 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 0);
}

// ── map ───────────────────────────────────────────────────────────────────────

#[test]
fn test_array_method_map() {
    let result = eval("[1, 2, 3].map(fun(x) { x * 2 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 3);
    assert_integer(arr.elements[0].clone(), 2);
    assert_integer(arr.elements[1].clone(), 4);
    assert_integer(arr.elements[2].clone(), 6);
}

#[test]
fn test_array_method_map_empty() {
    let result = eval("[].map(fun(x) { x * 2 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 0);
}

#[test]
fn test_array_method_map_to_strings() {
    let result = eval(r#"[1, 2, 3].map(fun(x) { "item" })"#);
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 3);
    assert_string(arr.elements[0].clone(), "item");
}

#[test]
fn test_array_method_map_chained_with_filter() {
    let result = eval("[1, 2, 3, 4].filter(fun(x) { x > 2 }).map(fun(x) { x * 10 })");
    let Object::Array(arr) = result else { panic!("Expected Array") };
    assert_eq!(arr.elements.len(), 2);
    assert_integer(arr.elements[0].clone(), 30);
    assert_integer(arr.elements[1].clone(), 40);
}

#[test]
fn test_array_method_map_wrong_args() {
    assert_error(eval("[1].map()"), "map() takes 1 argument, got 0");
    assert_error(eval("[1].map(fun(x){x}, fun(x){x})"), "map() takes 1 argument, got 2");
}