pub enum ParsedLine<'a> {
    OpenTag(&'a str),
    CloseTag(&'a str),
    TagWithValue(&'a str, &'a str),
}

pub fn parse_line(line: &str) -> ParsedLine {
    if let Some(i) = line.find('>') {
        let (tag, value) = line.split_at(i);
        let value = &value[1..];
        if let Some(tag) = tag.strip_prefix("</") {
            if !value.is_empty() {
                panic!("Unexpected value after closing tag: {} / {:?}", line, value);
            }

            ParsedLine::CloseTag(&tag)
        } else if let Some(tag) = tag.strip_prefix("<") {
            if value.is_empty() {
                ParsedLine::OpenTag(&tag)
            } else {
                ParsedLine::TagWithValue(&tag, value)
            }
        } else {
            panic!("Expected line to start with <, got {:?}", &line);
        }
    } else {
        panic!("Line did not contain >.")
    }
}
