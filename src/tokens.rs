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

fn next_token(mut reader: &mut impl BufRead) -> error::Result<Option<Token>> {
    let line = crate::read_line(&mut reader);
    if let Some(line) = line {
        let parsed = crate::parse::parse_line(&line.trim());

        Ok(Some(match parsed {
            ParsedLine::OpenTag("TEXT") => {
                let mut body = String::new();
                while let Some(v) = crate::read_line(&mut reader) {
                    if v == "</TEXT>\n" {
                        return Ok(Some(Token::TextData(body)));
                    } else {
                        body.push_str(&v);
                    }
                }

                unimplemented!()
            }
            ParsedLine::OpenTag(tag) => Token::TagOpen(ContainerTag::parse(tag)?),
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
