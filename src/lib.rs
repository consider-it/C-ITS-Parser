pub mod de;
#[cfg(feature = "wasm")]
pub mod en;
#[cfg(not(doctest))]
pub mod standards;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
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

pub(crate) fn map_err_to_string<E: core::fmt::Debug>(error: E) -> String {
    format!("{error:?}")
}
