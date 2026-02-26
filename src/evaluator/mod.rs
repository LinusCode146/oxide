use std::collections::HashMap;
use crate::ast::{Expression, Program, Statement};
use crate::builtins::get_builtin;
use crate::environment::Environment;
use crate::object::{ArrayObject, FunctionObject, HashObject, HashPair, Object};

pub fn eval_program(program: &Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for stmt in &program.statements {
        result = eval_statement(stmt, env);

        match &result {
            Object::ReturnValue(val) => return *val.clone(),
            Object::Error(_)         => return result,
            _                        => {}
        }
    }

    result
}

fn eval_statement(stmt: &Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::Expression(es) => eval_expression(&es.expression, env),

        Statement::Return(rs) => {
            let val = eval_expression(&rs.return_value, env);
            if val.is_error() { return val; }
            Object::ReturnValue(Box::new(val))
        }

        Statement::Let(ls) => {
            let mut val = eval_expression(&ls.value, env);
            if val.is_error() { return val; }

            if let Object::Function(ref mut func) = val {
                func.name = Some(ls.name.value.clone());
            }

            env.set(ls.name.value.clone(), val);
            Object::Null
        }

        Statement::Block(bs) => {
            let mut result = Object::Null;
            for s in &bs.statements {
                result = eval_statement(s, env);
                match &result {
                    Object::ReturnValue(_) | Object::Error(_) => return result,
                    _ => {}
                }
            }
            result
        }
    }
}

pub fn eval_expression(expr: &Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::IntegerLiteral(i) => Object::Integer(i.value),
        Expression::Boolean(b)        => Object::Boolean(b.value),
        Expression::StringLiteral(s) => Object::StringObj(s.value.clone()),

        Expression::Identifier(id) => eval_identifier(&id.value, env),

        Expression::HashLiteral(hl) => {
            let mut pairs = HashMap::new();
            for (key_expr, val_expr) in &hl.pairs {
                let key = eval_expression(key_expr, env);
                if key.is_error() { return key; }

                let hash_key = match key.hash_key() {
                    Some(k) => k,
                    None    => return Object::Error(format!("unusable as hash key: {}", key.object_type())),
                };

                let value = eval_expression(val_expr, env);
                if value.is_error() { return value; }

                pairs.insert(hash_key, HashPair { key, value });
            }
            Object::Hash(HashObject { pairs })
        }

        Expression::ArrayLiteral(arr) => {
            let elements = eval_expressions(&*arr.elements, env);
            Object::Array(ArrayObject {
                elements
            })
        },

        Expression::Index(ie) => {
            let left = eval_expression(&ie.left, env);
            if left.is_error() { return left; }
            let index = eval_expression(&ie.index, env);
            if index.is_error() { return index; }
            eval_index_expression(left, index)
        }

        Expression::Prefix(p) => {
            let right = eval_expression(&p.right, env);
            if right.is_error() { return right; }
            eval_prefix_expression(&p.operator, right)
        }

        Expression::Infix(i) => {
            let left = eval_expression(&i.left, env);
            if left.is_error() { return left; }
            let right = eval_expression(&i.right, env);
            if right.is_error() { return right; }
            eval_infix_expression(&i.operator, left, right)
        }

        Expression::If(ie) => {
            let condition = eval_expression(&ie.condition, env);
            if condition.is_error() { return condition; }

            if condition.is_truthy() {
                eval_block_statement(&ie.consequence, env)
            } else if let Some(alt) = &ie.alternative {
                eval_block_statement(alt, env)
            } else {
                Object::Null
            }
        }

        Expression::FunctionLiteral(fl) => {
            Object::Function(FunctionObject {
                name: None,            
                parameters: fl.parameters.clone(),
                body: fl.body.clone(),
                env: env.clone(),
            })
        }

        Expression::Call(call) => {
            let function = eval_expression(&call.function, env);
            if function.is_error() { return function; }

            let args = eval_expressions(&call.arguments, env);
            if args.len() == 1 && args[0].is_error() {
                return args.into_iter().next().unwrap();
            }

            apply_function(function, args)
        }
    }
}


fn eval_block_statement(block: &crate::ast::BlockStatement, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmt in &block.statements {
        result = eval_statement(stmt, env);
        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => {}
        }
    }
    result
}

fn eval_index_expression(left: Object, index: Object) -> Object {
    match (&left, &index) {
        (Object::Array(arr), Object::Integer(i)) => {
            let len = arr.elements.len() as i64;
            if *i < 0 || *i >= len {
                Object::Null
            } else {
                arr.elements[*i as usize].clone()
            }
        },
        (Object::Hash(hash), _) => {
            match index.hash_key() {
                None => Object::Error(format!(
                    "unusable as hash key: {}", index.object_type()
                )),
                Some(key) => hash.pairs
                    .get(&key)
                    .map(|pair| pair.value.clone())
                    .unwrap_or(Object::Null),
            }
        },
        _ => Object::Error(format!(
            "index operator not supported: {}",
            left.object_type()
        )),
    }
}

fn eval_identifier(name: &str, env: &Environment) -> Object {
    if let Some(val) = env.get(name) {
        return val.clone();
    }

    if let Some(builtin) = get_builtin(name) {
        return builtin;
    }

    Object::Error(format!("identifier not found: {}", name))
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator(right),
        "-" => eval_minus_prefix_operator(right),
        op  => Object::Error(format!("unknown prefix operator: {}{}", op, right.object_type())),
    }
}

fn eval_bang_operator(right: Object) -> Object {
    Object::Boolean(!right.is_truthy())
}

fn eval_minus_prefix_operator(right: Object) -> Object {
    match right {
        Object::Integer(i) => Object::Integer(-i),
        other => Object::Error(format!("unknown operator: -{}", other.object_type())),
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => {
            eval_integer_infix_expression(operator, *l, *r)
        }
        (Object::Boolean(l), Object::Boolean(r)) => match operator {
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            op   => Object::Error(format!(
                "unknown operator: {} {} {}",
                left.object_type(), op, right.object_type()
            )),
        },
        (Object::StringObj(l), Object::StringObj(r)) => match operator {
            "+" => Object::StringObj(format!("{}{}", l, r)),
            op  => Object::Error(format!(
                "unknown operator: STRING {} STRING", op
            )),
        },
        _ => {
            if left.object_type() != right.object_type() {
                Object::Error(format!(
                    "type mismatch: {} {} {}",
                    left.object_type(), operator, right.object_type()
                ))
            } else {
                Object::Error(format!(
                    "unknown operator: {} {} {}",
                    left.object_type(), operator, right.object_type()
                ))
            }
        }
    }
}

fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+"  => Object::Integer(left + right),
        "-"  => Object::Integer(left - right),
        "*"  => Object::Integer(left * right),
        "/"  => {
            if right == 0 {
                return Object::Error("division by zero".to_string());
            }
            Object::Integer(left / right)
        }
        "<"  => Object::Boolean(left < right),
        ">"  => Object::Boolean(left > right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        op   => Object::Error(format!("unknown operator: INTEGER {} INTEGER", op)),
    }
}

pub fn eval_expressions(args: &[Box<Expression>], env: &mut Environment) -> Vec<Object> {
    let mut result = Vec::new();
    for arg in args {
        let evaluated = eval_expression(arg, env);
        if evaluated.is_error() {
            return vec![evaluated];
        }
        result.push(evaluated);
    }
    result
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    match function {
        Object::Function(func) => {
            if func.parameters.len() != args.len() {
                return Object::Error(format!(
                    "wrong number of arguments: expected {}, got {}",
                    func.parameters.len(),
                    args.len()
                ));
            }

            let mut enclosed_env = Environment::new_enclosed(func.env.clone());

            if let Some(name) = &func.name {
                enclosed_env.set(name.clone(), Object::Function(func.clone()));
            }

            for (param, arg) in func.parameters.iter().zip(args) {
                enclosed_env.set(param.value.clone(), arg);
            }

            let result = eval_block_statement(&func.body, &mut enclosed_env);
            match result {
                Object::ReturnValue(val) => *val,
                other => other,
            }
        }
        Object::Builtin(func) => func(args),
        other => Object::Error(format!("not a function: {}", other.object_type())),
    }
}