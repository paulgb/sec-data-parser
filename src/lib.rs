use crate::document_tree::DocumentTree;
pub use crate::schema::*;
use crate::document_tree::parse_doc;
use crate::tokens::tokenize_submission;
use crate::tag::ContainerTag;
use std::io::BufRead;
use crate::error::Result;
use std::collections::VecDeque;

mod document_body;
mod document_tree;
mod error;
mod parse;
mod schema;
mod tag;
mod tokens;
mod types;

pub fn parse_submission(reader: &mut impl BufRead) -> Result<Submission> {
    let mut tokens = VecDeque::from(tokenize_submission(reader));
    if let Ok(DocumentTree::ContainerNode(ContainerTag::Submission, parts)) =
    parse_doc(&mut tokens)
    {
        Submission::from_parts(&parts)
    } else {
        panic!("here1");
    }
}