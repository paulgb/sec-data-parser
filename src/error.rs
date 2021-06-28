use crate::tag::ContainerTag;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    InvalidValueTag(String),
    InvalidContainerTag(String),
    UnexpectedEndOfInput(ContainerTag),
    UnexpectedCloseTag(ContainerTag),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ParseError {}
