use crate::object::Object;

pub type BuiltinFn = fn(Vec<Object>) -> Object;

pub fn get_builtin(name: &str) -> Option<Object> {
    match name {
        "log" => Some(Object::Builtin(builtin_log)),
        "len" => Some(Object::Builtin(builtin_len)),
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
        obj => Object::Error(format!(
            "argument to `len` not supported, got {}",
            obj.object_type()
        )),
    }
}