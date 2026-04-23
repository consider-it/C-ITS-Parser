//! Parser and encoder for ETSI C-ITS (V2X) messages including GeoNetworking headers and optionally Radiotap and IEEE 802.11 headers.
//! It supports UPER, XER and JER ASN.1 encodings for parsing and encoding.
//!
//! Supported messages/ standards: See documentation of the [`ItsMessage`] variants.
//!
//! ## Decoding
//!
//! A packet can be decoded using the [`de::decode`] function:
//!
//! ```
//! let data = &[0x02, 0x02, 0xde, 0x14, 0x0c, 0xe5]; // provide actual message buffer here
//! #[cfg(feature = "_etsi")]
//! match etsi_web::de::decode(data, etsi_web::Headers::RadioTap802LlcGnBtp) {
//!     Ok(etsi_web::ItsMessage::Cam {
//!         geonetworking: _,
//!         transport: _,
//!         etsi: cam,
//!     }) => {
//!         println!("Got a CAM: {cam:?}")
//!     }
//!     Ok(msg) => println!("Got: {msg:?}"),
//!     Err(err) => println!("Failed to parse message: {err}"),
//! }
//! ```
//!
//! The `headers` argument needs to specify which headers are present: None, GeoNetworking + BTP or Radiotap + 802.11p + LLC + GeoNetworking + BTP.
//! Headers are expected to be present in binary form.
//! When no headers are present, it can auto-detect the ASN.1 encoding (UPER/ XER/ JER) and decodes the message.
//! This means, that XER and JER message buffers can only be decoded without headers.
//!
//! ## Encoding
//!
//! To encode an [`ItsMessage`] struct, call the [`encode()`](`ItsMessage::encode`) method supplying the intended encoding rules.
//! Again, XER and JER messages can only be encoded without headers.
//! GeoNetworking and transport (BTP) headers will be added when present if UPER encoding is used.
//!
//! ## Feature Flags
//!
//! This library has several feature flags to allow fine-grained control over the feature set and additional dependencies.
//!
//! By default, all V2X messages and conversion to and from JSON are enabled.
//! If only some messages, or even just specific versions of messages are needed, they can be enabled one-by-one, e.g. using `denm` to enable both `denm_1_3_1` and `denm_2_2_1` support.
//!
//! When no parsing of the geonetworking and pcap headers is needed, the `transport` feature can be disabled.
//!
//! Besides parsing, the Rust API also provides helper functions to convert between ETSI data types and "normal"/ SI units.
//! Additional conversions are only available by adding some feature flags:
//!
//! - `time`: Enable conversions to [chrono](https://crates.io/crates/chrono) timestamps
//! - `geo`: Enable conversions to [geo-types](https://crates.io/crates/geo-types) (as lon/lat coordinates in degrees)

#![cfg_attr(not(target_arch = "wasm32"), no_std)]

extern crate alloc;

#[cfg(any(feature = "std", target_arch = "wasm32", test))]
extern crate std;

pub mod de;
#[cfg(feature = "_etsi")]
pub mod en;
#[cfg(feature = "_etsi")]
#[allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code)]
pub mod standards;

#[cfg(feature = "geo")]
pub mod geo_utils;
#[cfg(feature = "time")]
pub mod time_utils;

#[cfg(feature = "transport")]
pub(crate) mod pcap;
#[cfg(feature = "transport")]
pub mod transport;

#[cfg(feature = "transport")]
pub use geonetworking::{Decode, Packet};
#[cfg(feature = "transport")]
pub use pcap::remove_pcap_headers;
#[cfg(feature = "_etsi")]
use transport::TransportHeader;
#[cfg(all(target_arch = "wasm32", any(feature = "json", feature = "_etsi")))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, PartialEq, Default)]
/// Wrapper for the stringified JSON of headers and ITS ETSI message
pub struct JsonItsMessage {
    /// Optional GeoNetworking header, encoded as stringified JSON
    pub geonetworking: Option<String>,
    /// Optional transport header, encoded as stringified JSON
    pub transport: Option<String>,
    /// Optional ITS ETSI message, encoded as a UTF-8 String for JER and XER, as a hex string for UPER
    pub its: Option<String>,
    /// Optional ITS ETSI message type, as specified in ETSI TS 102 894-2
    /// - 1  - `denm`              - for Decentralized Environmental Notification Message (DENM) as specified in ETSI EN 302 637-3 [2],
    /// - 2  - `cam`               - for Cooperative Awareness Message (CAM) as specified in ETSI EN 302 637-2 [1],
    /// - 3  - `poi`               - for Point of Interest message as specified in ETSI TS 101 556-1 [9],
    /// - 4  - `spatem`            - for Signal Phase And Timing Extended Message (SPATEM) as specified in ETSI TS 103 301 [15],
    /// - 5  - `mapem`             - for MAP Extended Message (MAPEM) as specified in ETSI TS 103 301 [15],
    /// - 6  - `ivim`              - for in Vehicle Information Message (IVIM) as specified in ETSI TS 103 301 [15],
    /// - 7  - `ev-rsr`            - for Electric vehicle recharging spot reservation message, as defined in ETSI TS 101 556-3 [11],
    /// - 8  - `tistpgtransaction` - for messages for Tyre Information System (TIS) and Tyre Pressure Gauge (TPG) interoperability, as specified in ETSI TS 101 556-2 [10],
    /// - 9  - `srem`              - for Signal Request Extended Message as specified in ETSI TS 103 301 [15],
    /// - 10 - `ssem`              - for Signal request Status Extended Message as specified in ETSI TS 103 301 [15],
    /// - 11 - `evcsn`             - for Electrical Vehicle Charging Spot Notification message as specified in ETSI TS 101 556-1 [9],
    /// - 12 - `saem`              - for Services Announcement Extended Message as specified in ETSI EN 302 890-1 [17],
    /// - 13 - `rtcmem`            - for Radio Technical Commission for Maritime Services Extended Message (RTCMEM) as specified in ETSI TS 103 301 [15],
    /// - 14 - `cpm`               - reserved for Collective Perception Message (CPM),
    /// - 15 - `imzm`              - for Interference Management Zone Message (IMZM) as specified in ETSI TS 103 724 [13],
    /// - 16 - `vam`               - for Vulnerable Road User Awareness Message as specified in ETSI TS 130 300-3 [12],
    /// - 17 - `dsm`               - reserved for Diagnosis, logging and Status Message,
    /// - 18 - `pcim`              - reserved for Parking Control Infrastructure Message,
    /// - 19 - `pcvm`              - reserved for Parking Control Vehicle Message,
    /// - 20 - `mcm`               - reserved for Manoeuver Coordination Message,
    /// - 21 - `pam`               - reserved for Parking Availability Message,
    /// - 22-255                   - reserved for future usage.
    pub message_type: u8,
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen]
impl JsonItsMessage {
    #[wasm_bindgen(constructor)]
    pub fn from(
        its: Option<String>,
        geonetworking: Option<String>,
        transport: Option<String>,
        message_type: u8,
    ) -> Self {
        Self {
            its,
            geonetworking,
            transport,
            message_type,
        }
    }
}

#[cfg(feature = "_etsi")]
#[derive(Debug, Clone, PartialEq)]
/// Wrapper for C-ITS messages
///
/// Each message consists of the `etsi` data and can optionally contain a `transport` (BTP) and a `geonetworking` header.
pub enum ItsMessage<'a> {
    #[cfg(feature = "denm_1_3_1")]
    /// ETSI EN 302 637-3 v1.3.1 DENM
    DenmV1 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::denm_1_3_1::denm_pdu_descriptions::DENM>,
    },
    #[cfg(feature = "denm_2_2_1")]
    /// ETSI TS 103 831 v2.2.1 (or v2.1.1) DENM
    DenmV2 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::denm_2_2_1::denm_pdu_description::DENM>,
    },
    #[cfg(feature = "cam_1_4_1")]
    /// ETSI TS 103 301 v2.2.1 (or v2.1.1 or v1.3.1) CAM
    Cam {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::cam_1_4_1::cam_pdu_descriptions::CAM>,
    },
    #[cfg(feature = "spatem_2_2_1")]
    /// ETSI TS 103 301 v2.2.1 (or v2.1.1 or v1.3.1) SPATEM
    Spatem {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::spatem_2_2_1::spatem_pdu_descriptions::SPATEM>,
    },
    #[cfg(feature = "mapem_2_2_1")]
    /// ETSI TS 103 301 v2.2.1 (or v2.1.1 or v1.3.1) MAPEM
    Mapem {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::mapem_2_2_1::mapem_pdu_descriptions::MAPEM>,
    },
    #[cfg(feature = "ivim_2_1_1")]
    /// ETSI TS 103 301 v2.1.1 (or v1.3.1) IVIM
    IvimV1 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::ivim_2_1_1::ivim_pdu_descriptions::IVIM>,
    },
    #[cfg(feature = "ivim_2_2_1")]
    /// ETSI TS 103 301 v2.2.1 IVIM
    IvimV2 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::ivim_2_2_1::ivim_pdu_descriptions::IVIM>,
    },
    #[cfg(feature = "srem_2_2_1")]
    /// ETSI TS 103 301 v2.2.1 (or v2.1.1 or v1.3.1) SREM
    Srem {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::srem_2_2_1::srem_pdu_descriptions::SREM>,
    },
    #[cfg(feature = "ssem_2_2_1")]
    /// ETSI TS 103 301 v2.2.1 (or v2.1.1 or v1.3.1) SSEM
    Ssem {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::ssem_2_2_1::ssem_pdu_descriptions::SSEM>,
    },
    #[cfg(feature = "cpm_1")]
    /// ETSI TR 103 562 v2.1.1 CPM
    CpmV1 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<standards::cpm_1::cpm_pdu_descriptions::CPM>,
    },
    #[cfg(feature = "cpm_2_1_1")]
    /// ETSI TS 103 324 v2.1.1 CPM
    CpmV2 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<alloc::boxed::Box<TransportHeader>>,
        etsi: alloc::boxed::Box<
            standards::cpm_2_1_1::cpm_pdu_descriptions::CollectivePerceptionMessage,
        >,
    },
}

#[cfg(feature = "_etsi")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
/// Choice which message headers are present in the binary message buffer
pub enum Headers {
    /// No headers before V2X message
    None,
    /// Binary message with GeoNetworking and BTP headers
    GnBtp,
    /// Binary message with Radiotap, IEEE 802.11p, LLC, GeoNetworking and BTP headers
    RadioTap802LlcGnBtp,
}

#[cfg(feature = "_etsi")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
/// Choice of ASN.1 encoding rule
pub enum EncodingRules {
    UPER,
    XER,
    JER,
}

#[cfg(any(
    feature = "_etsi",
    all(target_arch = "wasm32", feature = "_etsi", feature = "json"),
    all(test, feature = "_etsi")
))]
impl EncodingRules {
    pub(crate) fn codec(self) -> rasn::Codec {
        match self {
            EncodingRules::UPER => rasn::Codec::Uper,
            EncodingRules::XER => rasn::Codec::Xer,
            EncodingRules::JER => rasn::Codec::Jer,
        }
    }
}

#[cfg(any(feature = "transport", feature = "_etsi"))]
pub(crate) fn map_err_to_string<E: core::fmt::Debug>(error: E) -> alloc::string::String {
    alloc::format!("{error:?}")
}
