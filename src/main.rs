use std::collections::VecDeque;
use std::fs::{read_dir, File};
use std::io::BufReader;

mod document_tree;
mod error;
mod parse;
mod tag;
mod tokens;

fn main() {
    for file in read_dir("./data").unwrap() {
        let path = file.unwrap().path();
        println!("Reading: {:?}", &path);

        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut tokens = VecDeque::from(tokens::tokenize_submission(&mut reader));
        let _doc = document_tree::parse_doc(&mut tokens).unwrap();
    }
}
