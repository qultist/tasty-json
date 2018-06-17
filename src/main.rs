#[macro_use]
extern crate nom;
extern crate regex;

mod parsers;
mod json;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_json_file(path: &String) -> String {
    println!("{}", path);
    let mut f = File::open(path).expect("File not found");
    let mut content = String::new();

    f.read_to_string(&mut content).expect("Could not read the file");

    return content
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments")
    }

    let json_string = read_json_file(&args[1]);
    println!("{}", json_string)
}
