#[macro_use]
extern crate nom;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use nom::*;
use nom::types::CompleteStr;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
enum Value {
    String(String),
    True,
    False,
    Null
}

named!(string<&str, &str>,
    delimited!(
        tag!("\""),
        take_until!("\""),
        tag!("\"")
    )
);

named!(literal_true<&str, Value>,
    value!(Value::True, tag!("true"))
);

named!(literal_false<&str, Value>,
    value!(Value::False, tag!("false"))
);

named!(literal_null<&str, Value>,
    value!(Value::Null, tag!("null"))
);

named!(json_value<&str, Value>,
    alt!(
        string => { |s: &str| Value::String(s.to_owned()) }
        | literal_true
        | literal_false
        | literal_null
    )
);

named!(number<CompleteStr, Result<i32,ParseIntError>>,
    map!(digit, |s: CompleteStr| { FromStr::from_str(s.0) })
);

named!(pair<&str, (String, Value)>,
    separated_pair!(
        map!(string, |s: &str| { s.to_owned() }),
        ws!(char!(':')),
        json_value
    )
);

named!(object<&str, HashMap<String, Value>>,
    map!(
        separated_list_complete!(
            ws!(char!(',')),
            pair
        ),
        |vec: Vec<(String, Value)>| { HashMap::from_iter(vec.into_iter()) }
    )
);

fn read_json_file(path: &String) -> String {
    println!("{}", path);
    let mut f = File::open(path).expect("File not found");
    let mut content = String::new();

    f.read_to_string(&mut content).expect("Could not read the file");

    return content
}

#[test]
fn parse_string() {
    let string_test = string("\"Hallo Welt!\"");
    assert_eq!(string_test, Ok(("", "Hallo Welt!")))
}

#[test]
fn parse_number() {
    let number_test = number(CompleteStr("42"));
    assert_eq!(number_test, Ok((CompleteStr(""), Ok(42))))
}

#[test]
fn parse_string_pair() {
    let pair_string = "\"manufacturer\": \"BMW\"";
    let pair_test = pair(pair_string);
    assert_eq!(pair_test, Ok(("", ("manufacturer".to_owned(), Value::String(String::from("BMW"))))))
}

#[test]
fn parse_literal_true() {
    let literal_test = literal_true("true");
    assert_eq!(literal_test, Ok(("", Value::True)))
}

#[test]
fn parse_value() {
    let value_test = json_value("null");
    assert_eq!(value_test, Ok(("", Value::Null)))
}

#[test]
fn parse_object() {
    let object_string = "\"manufacturer\": \"BMW\",\"model\": \"1 Series\", \"hatchback\": true";
    let object_test = object(object_string);

    let vec = vec![
        ("manufacturer".to_string(), Value::String("BMW".to_string())),
        ("model".to_string(), Value::String("1 Series".to_string())),
        ("hatchback".to_string(), Value::True)
    ];

    let map: HashMap<String, Value> = HashMap::from_iter(vec.into_iter());
    assert_eq!(object_test, Ok(("", map)))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments")
    }

    let json_string = read_json_file(&args[1]);
    println!("{}", json_string)
}
