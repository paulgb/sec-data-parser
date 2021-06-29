pub use crate::document_body::*;
use crate::document_tree::parse_doc;
use crate::document_tree::DocumentTree;
use crate::error::Result;
pub use crate::schema::*;
use crate::tag::ContainerTag;
use crate::tokens::tokenize_submission;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::Path;

mod document_body;
mod document_tree;
mod error;
mod schema;
mod tag;
mod tokens;
mod types;

pub fn parse_submission(path: &Path) -> Result<Submission> {
    let st = read_to_string(path).unwrap();
    let mut tokens = VecDeque::from(tokenize_submission(st)?);

    if let Ok(DocumentTree::ContainerNode(ContainerTag::Submission, parts)) = parse_doc(&mut tokens)
    {
        Submission::from_parts(&parts)
    } else {
        panic!("here1");
    }
}
