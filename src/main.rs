#[macro_use]
extern crate nom;
extern crate regex;

mod parsers;
mod json;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use json::*;

fn read_json_file(path: &str) -> String {
    let mut f = File::open(path).expect("File not found");
    let mut content = String::new();

    f.read_to_string(&mut content).expect("Could not read the file");

    return content
}

fn main() {
    let json_string = read_json_file("./resources/example.json");

    let json = parse_json(&json_string);

    let glossary = json["glossary"].as_map().unwrap();
    let title = glossary["title"].as_string().unwrap();

    println!("Title: {}", title);
}
