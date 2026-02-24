use std::fmt;
use crate::ast::{BlockStatement, Identifier};
use crate::environment::Environment;

pub type BuiltinFn = fn(Vec<Object>) -> Object;

#[derive(Clone, Debug)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    StringObj(String),
    ReturnValue(Box<Object>),
    Function(FunctionObject),
    Builtin(BuiltinFn),
    Null,
    Error(String),
}

#[derive(Clone, Debug)]
pub struct FunctionObject {
    pub parameters: Vec<Identifier>,
    pub name: Option<String>,
    pub body: BlockStatement,
    pub env: Environment,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i)      => write!(f, "{}", i),
            Object::Boolean(b)      => write!(f, "{}", b),
            Object::StringObj(s) => write!(f, "{}", s),
            Object::Null            => write!(f, "null"),
            Object::ReturnValue(v)  => write!(f, "{}", v),
            Object::Function(func)  => {
                use crate::ast::Node;
                let params = func.parameters
                    .iter()
                    .map(|p| p.string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "fn({}) {{\n{}\n}}", params, func.body.string())
            },
            Object::Builtin(_) => write!(f, "[builtin function]"),
            Object::Error(msg)      => write!(f, "ERROR: {}", msg),
        }
    }
}

impl Object {
    pub fn object_type(&self) -> &'static str {
        match self {
            Object::Integer(_)     => "INTEGER",
            Object::Boolean(_)     => "BOOLEAN",
            Object::Null           => "NULL",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Function(_)    => "FUNCTION",
            Object::Builtin(_)     => "BUILTIN",
            Object::StringObj(_) => "STRING",
            Object::Error(_)       => "ERROR",
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Null        => false,
            Object::Boolean(b)  => *b,
            _                   => true,
        }
    }
}