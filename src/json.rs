use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Bool(bool),
    Null
}