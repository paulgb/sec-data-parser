use crate::document_tree::parse_doc;
use crate::document_tree::DocumentTree;
use crate::error::Result;
pub use crate::schema::*;
pub use crate::document_body::*;
use crate::tag::ContainerTag;
use crate::tokens::tokenize_submission;
use std::collections::VecDeque;
use std::io::BufRead;

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
    if let Ok(DocumentTree::ContainerNode(ContainerTag::Submission, parts)) = parse_doc(&mut tokens)
    {
        Submission::from_parts(&parts)
    } else {
        panic!("here1");
    }
}
