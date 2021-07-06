use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuencode::uudecode;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Plaintext,
    Xml,
    Pdf,
    Xbrl,
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Plaintext => write!(f, "Plain Text"),
            DataType::Xml => write!(f, "XML"),
            DataType::Pdf => write!(f, "PDF"),
            DataType::Xbrl => write!(f, "XBRL"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocumentBody {
    BinaryData(String, Vec<u8>),
    Text(String),
}

impl Display for DocumentBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentBody::BinaryData(filename, data) => {
                write!(f, "Binary file {} with {} bytes.", filename, data.len())
            }
            DocumentBody::Text(data) => write!(f, "Text data with {} bytes", data.len()),
        }
    }
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedData {
    pub data_type: DataType,
    pub body: DocumentBody,
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
        } else if let Some(st) = st.strip_prefix("<XBRL>") {
            TypedData {
                data_type: DataType::Pdf,
                body: DocumentBody::from_string(st.strip_suffix("</XBRL>").unwrap()),
            }
        } else {
            TypedData {
                data_type: DataType::Plaintext,
                body: DocumentBody::from_string(st),
            }
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        match &self.body {
            DocumentBody::BinaryData(_, b) => &b,
            DocumentBody::Text(s) => s.as_bytes(),
        }
    }
}
