use crate::object::{ArrayObject, Object};

pub type BuiltinFn = fn(Vec<Object>) -> Object;

pub fn get_builtin(name: &str) -> Option<Object> {
    match name {
        "log" => Some(Object::Builtin(builtin_log)),
        "len" => Some(Object::Builtin(builtin_len)),
        "first" => Some(Object::Builtin(builtin_first)),
        "last" => Some(Object::Builtin(builtin_last)),
        "tail" => Some(Object::Builtin(builtin_tail)),
        "push" => Some(Object::Builtin(builtin_push)),
        _ => None,
    }
}

fn builtin_log(args: Vec<Object>) -> Object {
    if args.is_empty() {
        println!();
        return Object::Null;
    }

    let output: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
    println!("{}", output.join(" "));

    Object::Null
}
fn builtin_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::StringObj(s) => Object::Integer(s.len() as i64),
        Object::Array(arr) => Object::Integer(arr.elements.len() as i64),
        obj => Object::Error(format!(
            "argument to `len` not supported, got {}",
            obj.object_type()
        )),
    }
}
fn builtin_first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.elements.is_empty() {
                Object::Null
            } else {
                arr.elements[0].clone()
            }
        }
        obj => Object::Error(format!(
            "argument to `first` not supported, got {}",
            obj.object_type()
        )),
    }
}
fn builtin_last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.elements.is_empty() {
                Object::Null
            } else {
                arr.elements[arr.elements.len() - 1].clone()
            }
        }
        obj => Object::Error(format!(
            "argument to `last` not supported, got {}",
            obj.object_type()
        )),
    }
}
fn builtin_tail(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.elements.is_empty() {
                Object::Null
            } else {
                Object::Array(ArrayObject {
                    elements: arr.elements[1..].to_vec()
                })
            }
        }
        obj => Object::Error(format!(
            "argument to `first` not supported, got {}",
            obj.object_type()
        )),
    }
}
fn builtin_push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(arr) => {
            let mut cop = arr.elements.clone();
            cop.push(args[1].clone());
            Object::Array(ArrayObject {

            elements: cop,
        })
        }
        obj => Object::Error(format!(
            "argument to `first` not supported, got {}",
            obj.object_type()
        )),
    }
}