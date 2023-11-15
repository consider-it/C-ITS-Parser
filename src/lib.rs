extern crate alloc;

pub mod de;
pub mod standards;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct EtsiJson {
    pub geonetworking: Option<String>,
    pub transport: Option<String>,
    pub its: Option<String>,
}
