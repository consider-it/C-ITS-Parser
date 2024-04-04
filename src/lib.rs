pub mod de;
pub mod en;
pub mod standards;

pub(crate) mod transport;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, PartialEq)]
/// Wrapper for the stringified JSON of headers and ITS ETSI message
pub struct EtsiJson {
    /// Optional GeoNetworking header, encoded as stringified JSON
    pub geonetworking: Option<String>,
    /// Optional transport header, encoded as stringified JSON
    pub transport: Option<String>,
    /// Optional ITS ETSI message, encoded as stringified JSON
    pub its: Option<String>,
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
        }
    }
}

pub(crate) fn map_err_to_string<E: core::fmt::Debug>(error: E) -> String {
    format!("{error:?}")
}
