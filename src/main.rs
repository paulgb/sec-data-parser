use crate::document_tree::DocumentTree;
use crate::schema::Submission;
use crate::tag::ContainerTag;
use std::collections::VecDeque;
use std::fs::{read_dir, File};
use std::io::BufReader;

mod document_body;
mod document_tree;
mod error;
mod parse;
mod schema;
mod tag;
mod tokens;
mod types;

fn main() {
    for file in read_dir("./data").unwrap() {
        let path = file.unwrap().path();
        println!("Reading: {:?}", &path);

        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut tokens = VecDeque::from(tokens::tokenize_submission(&mut reader));
        if let Ok(DocumentTree::ContainerNode(ContainerTag::Submission, parts)) =
            document_tree::parse_doc(&mut tokens)
        {
            let _submission = Submission::from_parts(&parts);
        } else {
            panic!("here1");
        }
    }
}
