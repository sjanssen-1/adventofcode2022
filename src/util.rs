use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let br = BufReader::new(file);
    return  br.lines().collect::<Result<_, _>>().unwrap();
}