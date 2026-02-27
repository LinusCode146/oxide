use std::collections::HashMap;
use crate::object::Object;

#[derive(Clone, Debug)]

#[derive(PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Environment) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name).or_else(|| {
            self.outer.as_deref()?.get(name)
        })
    }

    pub fn set(&mut self, name: String, value: Object) -> &Object {
        self.store.entry(name).or_insert(value)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}