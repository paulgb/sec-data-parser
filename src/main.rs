use std::collections::VecDeque;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};

mod document_tree;
mod error;
mod tag;
mod tokens;

enum ParsedLine<'a> {
    OpenTag(&'a str),
    CloseTag(&'a str),
    TagWithValue(&'a str, &'a str),
}

fn parse_line(line: &str) -> ParsedLine {
    if let Some(i) = line.find('>') {
        let (tag, value) = line.split_at(i);
        let value = &value[1..];
        if let Some(tag) = tag.strip_prefix("</") {
            if value.is_empty() {
                panic!("Unexpected value after closing tag: {} / {}", line, value);
            }

            ParsedLine::CloseTag(&tag)
        } else if let Some(tag) = tag.strip_prefix("<") {
            if value.is_empty() {
                ParsedLine::OpenTag(&tag[1..])
            } else {
                ParsedLine::TagWithValue(&tag[1..], value)
            }
        } else {
            panic!("Expected line to start with <, got {:?}", &line);
        }
    } else {
        panic!("Line did not contain >.")
    }
}

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
