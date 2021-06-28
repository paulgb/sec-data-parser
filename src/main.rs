mod error;
mod tag;
mod tokens;

use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};

use crate::error::Result;
use crate::tag::{ContainerTag, ValueTag};
use crate::tokens::Token;

enum ParsedLine<'a> {
    TagOpen(&'a str),
    TagClose(&'a str),
    TagValue(&'a str, &'a str),
}

fn parse_line(line: &str) -> ParsedLine {
    //println!("line: {:?}", line);
    if !line.starts_with('<') {
        panic!("Expected line to start with <, got {:?}", &line);
    }

    if let Some(i) = line.find(">") {
        let (tag, value) = line.split_at(i);
        let value = &value[1..];
        if tag.starts_with("</") {
            if value != "" {
                panic!("Unexpected value after closing tag: {} / {}", line, value);
            }

            ParsedLine::TagClose(&tag[2..])
        } else {
            if value == "" {
                ParsedLine::TagOpen(&tag[1..])
            } else {
                ParsedLine::TagValue(&tag[1..], value)
            }
        }
    } else {
        panic!("Line started with < but did not contain >.")
    }
}

fn read_line(reader: &mut impl BufRead) -> Option<String> {
    let mut line: String = Default::default();
    let result = reader.read_line(&mut line).unwrap();

    if result == 0 {
        return None;
    } else {
        return Some(line);
    }
}

fn next_token(mut reader: impl BufRead) -> Result<Option<Token>> {
    let line = read_line(&mut reader);
    if let Some(line) = line {
        let parsed = parse_line(&line.trim());

        Ok(Some(match parsed {
            ParsedLine::TagOpen("TEXT") => {
                let mut body = String::new();
                while let Some(v) = read_line(&mut reader) {
                    if v == "</TEXT>\n" {
                        return Ok(Some(Token::TextData(body)));
                    } else {
                        body.push_str(&v);
                    }
                }

                unimplemented!()
            }
            ParsedLine::TagOpen(tag) => Token::TagOpen(ContainerTag::parse(tag)?),
            ParsedLine::TagClose(tag) => Token::TagClose(ContainerTag::parse(tag)?),
            ParsedLine::TagValue(tag, value) => {
                Token::TagValue(ValueTag::parse(tag)?, value.to_string())
            }
        }))
    } else {
        Ok(None)
    }
}

fn tokenize_submission(mut submission: BufReader<File>) -> Vec<Token> {
    let mut tokens = Vec::new();

    while let Some(token) = next_token(&mut submission).unwrap() {
        //println!("Token: {:?}", token);

        tokens.push(token);
    }

    tokens
}

fn main() {
    for file in read_dir("./data").unwrap() {
        let path = file.unwrap().path();
        println!("{:?}", &path);
        tokenize_submission(BufReader::new(File::open(path).unwrap()));
    }
}
