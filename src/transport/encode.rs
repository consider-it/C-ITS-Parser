
use super::*;

#[derive(Debug)]
pub enum EncodeError {
    Unsupported(String),
    Json(String),
}

pub trait Encode: Sized {
    fn encode(&self) -> Result<Vec<u8>, EncodeError>;
}

impl Encode for BasicTransportAHeader {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok([
            self.destination_port.to_be_bytes(),
            self.source_port.to_be_bytes(),
        ]
        .concat())
    }
}

impl BasicTransportAHeader {
    pub fn encode_to_json(&self) -> Result<String, EncodeError> {
        serde_json::to_string(&self)
            .map_err(|e| EncodeError::Json(format!("Error encoding to JSON: {e:?}")))
    }
}

impl Encode for BasicTransportBHeader {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok([
            self.destination_port.to_be_bytes(),
            self.destination_port_info.to_be_bytes(),
        ]
        .concat())
    }
}

impl BasicTransportBHeader {
    pub fn encode_to_json(&self) -> Result<String, EncodeError> {
        serde_json::to_string(&self)
            .map_err(|e| EncodeError::Json(format!("Error encoding to JSON: {e:?}")))
    }
}

impl Encode for IPv6Header {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Err(EncodeError::Unsupported(format!(
            "Encoding IPv6 headers is currently unsupported."
        )))
    }
}