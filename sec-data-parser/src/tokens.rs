use crate::error::Result;
use crate::tag::{ContainerTag, ValueTag};

#[derive(Debug, PartialEq)]
pub enum Token {
    ContainerTagOpen(ContainerTag),
    ContainerTagClose(ContainerTag),
    ValueTag(ValueTag),
    RawText(String),
    TextBlock(String),
}

pub fn next_token(st: &str) -> Result<(Token, &str)> {
    Ok(if st.starts_with('<') {
        let closing = st.starts_with("</");
        let end_idx = st.find('>').unwrap();
        let start_idx = if closing { 2 } else { 1 };
        let tag = st[start_idx..end_idx].to_string();
        if tag == "TEXT" {
            let start_idx = "<TEXT>".len();
            let end_idx = st.find("</TEXT>").unwrap();
            let content = st[start_idx..end_idx].to_string();
            let st = &st[end_idx + "</TEXT>".len()..];

            (Token::TextBlock(content), st)
        } else if let Ok(container_tag) = ContainerTag::parse(&tag) {
            if closing {
                (Token::ContainerTagClose(container_tag), &st[end_idx + 1..])
            } else {
                (Token::ContainerTagOpen(container_tag), &st[end_idx + 1..])
            }
        } else {
            (Token::ValueTag(ValueTag::parse(&tag)?), &st[end_idx + 1..])
        }
    } else {
        let end_idx = st.find('<').unwrap();
        (
            Token::RawText(st[..end_idx].trim().to_string()),
            &st[end_idx..],
        )
    })
}

pub fn tokenize_submission(submission: String) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut st = submission.as_str();

    while st.len() > 0 {
        if st.starts_with('\n') || st.starts_with(' ') {
            st = &st[1..];
            continue;
        }

        let (tok, new_st) = next_token(st)?;
        tokens.push(tok);
        st = new_st;
    }

    Ok(tokens)
}
