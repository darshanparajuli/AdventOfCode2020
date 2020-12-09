use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input() -> Vec<String> {
    read_input_map(|e| e)
}

pub fn read_input_map<T>(mapper: fn(String) -> T) -> Vec<T> {
    let arg = env::args().skip(1).next().unwrap();
    match File::open(&arg) {
        Ok(f) => BufReader::new(f)
            .lines()
            .map(|line| mapper(line.unwrap()))
            .collect(),
        Err(e) => {
            eprintln!("Error opening input file '{}': {}", arg, e);
            std::process::exit(1);
        }
    }
}
