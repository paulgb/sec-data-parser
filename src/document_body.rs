use crate::tag::ContainerTag::Document;
use uuencode::uudecode;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Plaintext,
    Xml,
    Pdf,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DocumentBody {
    BinaryData(String, Vec<u8>),
    Text(String),
}

impl DocumentBody {
    pub fn from_string(st: &str) -> DocumentBody {
        if st.starts_with("begin 644") {
            let (data, filename) = uudecode(st).unwrap();
            DocumentBody::BinaryData(filename, data)
        } else {
            DocumentBody::Text(st.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedData {
    data_type: DataType,
    body: DocumentBody,
}

impl TypedData {
    pub fn from_string(st: &str) -> TypedData {
        let st = st.trim();
        if let Some(st) = st.strip_prefix("<XML>") {
            TypedData {
                data_type: DataType::Xml,
                body: DocumentBody::from_string(st.strip_suffix("</XML>").unwrap()),
            }
        } else if let Some(st) = st.strip_prefix("<PDF>") {
            TypedData {
                data_type: DataType::Pdf,
                body: DocumentBody::from_string(st.strip_suffix("</PDF>").unwrap()),
            }
        } else {
            TypedData {
                data_type: DataType::Plaintext,
                body: DocumentBody::from_string(st),
            }
        }
    }
}
