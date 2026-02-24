use crate::object::Object;

pub type BuiltinFn = fn(Vec<Object>) -> Object;

pub fn get_builtin(name: &str) -> Option<Object> {
    match name {
        "log" => Some(Object::Builtin(builtin_log)),
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