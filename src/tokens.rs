use crate::tag::{ContainerTag, ValueTag};

#[derive(Debug, PartialEq)]
pub enum Token {
    TagOpen(ContainerTag),
    TagClose(ContainerTag),
    TagValue(ValueTag, String),
    TextData(String),
}
