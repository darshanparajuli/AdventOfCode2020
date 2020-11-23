use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input() -> Vec<String> {
    let arg = env::args().skip(1).next().unwrap();
    BufReader::new(File::open(arg).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect()
}
