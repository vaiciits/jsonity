use std::fs;
use crate::parser::decode_json;

mod data_structures;
mod parser;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

fn main() {
    let filename = "data/input/1.json";
    match read_file(filename) {
        Ok(content) => {
            let decoded = decode_json(content);
            println!("{}", decoded);
        },
        Err(e) => println!("Failed to read {}: {}", filename, e),
    }
}
