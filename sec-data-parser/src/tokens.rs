use std::io::BufRead;

use crate::error;
use crate::parse::ParsedLine;
use crate::tag::{ContainerTag, ValueTag};

#[derive(Debug, PartialEq)]
pub enum Token {
    TagOpen(ContainerTag),
    TagClose(ContainerTag),
    TagValue(ValueTag, String),
    TextData(String),
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

fn next_token(mut reader: &mut impl BufRead) -> error::Result<Option<Token>> {
    let line = read_line(&mut reader);
    if let Some(line) = line {
        let parsed = crate::parse::parse_line(&line.trim());

        Ok(Some(match parsed {
            ParsedLine::OpenTag("TEXT") => {
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
            ParsedLine::OpenTag(tag) => {
                if let Ok(container_tag) = ContainerTag::parse(tag) {
                    Token::TagOpen(container_tag)
                } else {
                    Token::TagValue(ValueTag::parse(tag)?, "".to_string())
                }
            }
            ParsedLine::CloseTag(tag) => Token::TagClose(ContainerTag::parse(tag)?),
            ParsedLine::TagWithValue(tag, value) => {
                Token::TagValue(ValueTag::parse(tag)?, value.to_string())
            }
        }))
    } else {
        Ok(None)
    }
}

pub fn tokenize_submission(submission: &mut impl BufRead) -> Vec<Token> {
    let mut tokens = Vec::new();

    while let Some(token) = next_token(submission).unwrap() {
        tokens.push(token);
    }

    tokens
}
