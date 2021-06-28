use crate::tag::{ContainerTag, ValueTag};

#[derive(Debug)]
pub enum Token {
    TagOpen(ContainerTag),
    TagClose(ContainerTag),
    TagValue(ValueTag, String),
    TextData(String),
}
