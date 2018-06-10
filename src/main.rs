#[macro_use]
extern crate nom;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use nom::*;
use nom::types::CompleteStr;
use std::str::FromStr;
use std::num::ParseIntError;

named!(string<&str, &str>,
    delimited!(
        tag!("\""),
        take_until!("\""),
        tag!("\"")
    )
);

named!(number<CompleteStr, Result<i32,ParseIntError>>,
    map!(digit, |s: CompleteStr| { FromStr::from_str(s.0) })
);

named!(pair<&str, (&str, &str)>,
    do_parse!(
        s: string >>
        tag!(":") >>
        v: string >>

        ((s, v))
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
    let pair_string = "\"manufacturer\":\"BMW\"";
    let pair_test = pair(pair_string);
    assert_eq!(pair_test, Ok(("", ("manufacturer", "BMW"))))
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments")
    }

    let json_string = read_json_file(&args[1]);
    println!("{}", json_string)
}
