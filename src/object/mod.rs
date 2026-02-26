use std::fmt;
use crate::ast::{BlockStatement, Identifier};
use crate::environment::Environment;
use std::collections::HashMap;

pub type BuiltinFn = fn(Vec<Object>) -> Object;

#[derive(Clone, Debug)]
pub struct HashObject {
    pub pairs: HashMap<HashKey, HashPair>,
}

#[derive(Clone, Debug)]
pub struct HashPair {
    pub key: Object,
    pub value: Object,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HashKey {
    Integer(i64),
    Boolean(bool),
    StringObj(String),
}

impl Object {
    pub fn hash_key(&self) -> Option<HashKey> {
        match self {
            Object::Integer(i)    => Some(HashKey::Integer(*i)),
            Object::Boolean(b)    => Some(HashKey::Boolean(*b)),
            Object::StringObj(s)  => Some(HashKey::StringObj(s.clone())),
            _                     => None,
        }
    }
}

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
    Array(ArrayObject),
    Hash(HashObject),
}

#[derive(Clone, Debug)]
pub struct ArrayObject {
    pub elements: Vec<Object>
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
            Object::Array(arr) => {
                let elements = arr.elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{}]", elements)
            },
            Object::Hash(h) => {
                let pairs = h.pairs
                    .values()
                    .map(|p| format!("{}: {}", p.key, p.value))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{{{}}}", pairs)
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
            Object::Array(_)       => "ARRAY",
            Object::Hash(_) => "HASH",
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