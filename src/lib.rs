use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input() -> Vec<String> {
    let arg = env::args().skip(1).next().unwrap();
    match File::open(&arg) {
        Ok(f) => BufReader::new(f)
            .lines()
            .map(|line| line.unwrap())
            .collect(),
        Err(e) => {
            eprintln!("Error opening input file '{}': {}", arg, e);
            std::process::exit(1);
        }
    }
}
