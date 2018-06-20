use std::collections::HashMap;
use parsers::object;
use nom::types::CompleteStr;


#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Bool(bool),
    Null
}

impl Value {

    pub fn as_string(&self) -> Option<String> {
        if let Value::String(string) = self {
            return Some(string.to_owned())
        }

        None
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Value::Number(float) = self {
            return Some(*float)
        }

        None
    }

    pub fn as_map(&self) -> Option<HashMap<String, Value>> {
        if let Value::Object(map) = self {
            return Some(map.to_owned())
        }

        None
    }

    pub fn as_vec(&self) -> Option<Vec<Value>> {
        if let Value::Array(vec) = self {
            return Some(vec.to_owned())
        }

        None
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            return Some(*b)
        }

        None
    }
}

pub fn parse_json(string: &str) -> HashMap<String, Value> {
    object(CompleteStr(string)).unwrap().1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string() {
        let test = Value::String(String::from("ABC"));
        let string = test.as_string();

        assert_eq!("ABC", string.unwrap())
    }

    #[test]
    fn test_float() {
        let test = Value::Number(13.37);
        let float = test.as_f64();

        assert_eq!(13.37, float.unwrap());
    }

    #[test]
    fn test_vec() {
        let array = Value::Array(vec![Value::Null, Value::Bool(true), Value::String(String::from("ABC"))]);
        let vec = array.as_vec();

        assert_eq!(vec![Value::Null, Value::Bool(true), Value::String(String::from("ABC"))], vec.unwrap())
    }

    #[test]
    fn test_bool() {
        let test = Value::Bool(true);
        let b = test.as_bool();

        assert_eq!(true, b.unwrap());
    }

    #[test]
    fn test_failure() {
        let test = Value::String(String::from("Not a bool!"));
        let fail = test.as_bool();

        assert_eq!(None, fail)
    }
}