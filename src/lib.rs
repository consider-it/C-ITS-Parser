pub mod de;
pub mod en;
#[cfg(not(doctest))]
pub mod standards;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, PartialEq)]
pub struct EtsiJson {
    pub geonetworking: Option<String>,
    pub transport: Option<String>,
    pub its: Option<String>,
}

pub(crate) fn map_err_to_string<E: core::fmt::Debug>(error: E) -> String {
    format!("{error:?}")
}
