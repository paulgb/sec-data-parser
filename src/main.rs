mod error;
mod tag;
mod tokens;

use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};

use crate::error::{Result, ParseError};
use crate::tag::{ContainerTag, ValueTag};
use crate::tokens::Token;
use std::collections::VecDeque;
use crate::DocumentTree::ContainerNode;

enum ParsedLine<'a> {
    TagOpen(&'a str),
    TagClose(&'a str),
    TagValue(&'a str, &'a str),
}

enum DocumentTree {
    ContainerNode(ContainerTag, Vec<DocumentTree>),
    ValueNode(ValueTag, String),
    TextNode(String),
    Empty,
}

fn parse_doc(tokens: &mut VecDeque<Token>) -> Result<DocumentTree> {
    Ok(if let Some(token) = tokens.pop_front() {

        match token {
            Token::TagOpen(tag) => {
                let mut parts = Vec::new();

                while let Some(next_token) = tokens.front() {
                    if next_token == &Token::TagClose(tag) {
                        let t = tokens.pop_front();

                        return Ok(ContainerNode(tag, parts))
                    } else if let Token::TagClose(_) = next_token {
                        return Ok(ContainerNode(tag, parts))
                    } else {
                        parts.push(parse_doc(tokens)?);
                    }
                }

                return Err(ParseError::UnexpectedEndOfInput(tag))
            }
            Token::TagClose(tag) => return Err(ParseError::UnexpectedCloseTag(tag)),
            Token::TagValue(tag, value) => DocumentTree::ValueNode(tag, value),
            Token::TextData(text) => DocumentTree::TextNode(text)
        }
    } else {
        DocumentTree::Empty
    })
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

fn next_token(mut reader: &mut impl BufRead) -> Result<Option<Token>> {
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

fn tokenize_submission(submission: &mut impl BufRead) -> Vec<Token> {
    let mut tokens = Vec::new();

    while let Some(token) = next_token(submission).unwrap() {
        tokens.push(token);
    }

    tokens
}

fn main() {
    for file in read_dir("./data").unwrap() {
        let path = file.unwrap().path();
        println!("Reading: {:?}", &path);

        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut tokens = VecDeque::from(tokenize_submission(&mut reader));
        let _doc = parse_doc(&mut tokens).unwrap();
    }
}
