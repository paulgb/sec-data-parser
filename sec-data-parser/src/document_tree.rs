use std::collections::VecDeque;

use crate::document_tree::DocumentTree::ContainerNode;
use crate::error;
use crate::error::ParseError;
use crate::tag::{ContainerTag, ValueTag};
use crate::tokens::Token;

#[derive(Debug)]
pub enum DocumentTree {
    ContainerNode(ContainerTag, Vec<DocumentTree>),
    ValueNode(ValueTag, String),
    TextNode(String),
    Empty,
}

pub fn parse_doc(tokens: &mut VecDeque<Token>) -> error::Result<DocumentTree> {
    Ok(if let Some(token) = tokens.pop_front() {
        match token {
            Token::ContainerTagOpen(tag) => {
                let mut parts = Vec::new();

                while let Some(next_token) = tokens.front() {
                    if next_token == &Token::ContainerTagClose(tag) {
                        tokens.pop_front();

                        return Ok(ContainerNode(tag, parts));
                    } else if let Token::ContainerTagClose(c) = next_token {
                        panic!("Expected {:?}, got {:?}", c, tag);
                        //return Ok(ContainerNode(tag, parts));
                    } else {
                        parts.push(parse_doc(tokens)?);
                    }
                }

                return Err(ParseError::UnexpectedEndOfInput(tag));
            }
            Token::ContainerTagClose(tag) => return Err(ParseError::UnexpectedCloseTag(tag)),
            Token::ValueTag(tag) => {
                let mut value = "".to_string();
                while let Some(Token::RawText(c)) = tokens.front() {
                    value.push_str(c);
                    tokens.pop_front();
                }
                DocumentTree::ValueNode(tag, value)
            }
            Token::TextBlock(text) => DocumentTree::TextNode(text),
            _ => panic!("Unexpected: {:?}", &token),
        }
    } else {
        DocumentTree::Empty
    })
}
