use std::collections::VecDeque;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};

mod document_tree;
mod error;
mod tag;
mod tokens;
mod parse;

fn read_line(reader: &mut impl BufRead) -> Option<String> {
    let mut line: String = Default::default();
    let result = reader.read_line(&mut line).unwrap();

    if result == 0 {
        None
    } else {
        Some(line)
    }
}

fn main() {
    for file in read_dir("./data").unwrap() {
        let path = file.unwrap().path();
        println!("Reading: {:?}", &path);

        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut tokens = VecDeque::from(tokens::tokenize_submission(&mut reader));
        let _doc = document_tree::parse_doc(&mut tokens).unwrap();
    }
}
