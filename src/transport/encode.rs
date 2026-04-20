use super::{BasicTransportAHeader, BasicTransportBHeader, Debug, IPv6Header};

#[allow(dead_code)]
#[derive(Debug)]
pub enum EncodeError {
    Unsupported(String),
    #[cfg(feature = "json")]
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
    #[cfg(feature = "json")]
    /// Encodes a BTP-A as a JSON representation
    ///
    /// # Errors
    /// Returns an error when encoding failed.
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
    #[cfg(feature = "json")]
    /// Encodes a BTP-B as a JSON representation
    ///
    /// # Errors
    /// Returns an error when encoding failed.
    pub fn encode_to_json(&self) -> Result<String, EncodeError> {
        serde_json::to_string(&self)
            .map_err(|e| EncodeError::Json(format!("Error encoding to JSON: {e:?}")))
    }
}

impl Encode for IPv6Header {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Err(EncodeError::Unsupported(
            "Encoding IPv6 headers is currently unsupported.".to_string(),
        ))
    }
}
