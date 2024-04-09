pub mod de;
pub mod en;
pub mod standards;

pub(crate) mod pcap;
pub(crate) mod transport;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, PartialEq, Default)]
/// Wrapper for the stringified JSON of headers and ITS ETSI message
pub struct EtsiJson {
    /// Optional GeoNetworking header, encoded as stringified JSON
    pub geonetworking: Option<String>,
    /// Optional transport header, encoded as stringified JSON
    pub transport: Option<String>,
    /// Optional ITS ETSI message, encoded as stringified JSON
    pub its: Option<String>,
    /// Optional ITS ETSI message type
    pub message_type: Option<u8>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Headers {
    None,
    GnBtp,
    RadioTap802LlcGnBtp,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl EtsiJson {
    #[wasm_bindgen(constructor)]
    pub fn from(
        its: Option<String>,
        geonetworking: Option<String>,
        transport: Option<String>,
    ) -> EtsiJson {
        EtsiJson {
            its,
            geonetworking,
            transport,
            message_type: None,
        }
    }
}

pub(crate) fn map_err_to_string<E: core::fmt::Debug>(error: E) -> String {
    format!("{error:?}")
}
