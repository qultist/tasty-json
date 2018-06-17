use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    True,
    False,
    Null
}