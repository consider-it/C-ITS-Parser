pub mod de;
#[cfg(feature = "etsi")]
pub mod en;
#[cfg(feature = "etsi")]
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
#[cfg(feature = "etsi")]
use transport::TransportHeader;
#[cfg(all(target_arch = "wasm32", any(feature = "json", feature = "etsi")))]
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

#[cfg(feature = "etsi")]
#[derive(Debug, Clone, PartialEq)]
pub enum ItsMessage<'a> {
    #[cfg(feature = "denm_1_3_1")]
    DenmV1 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::denm_1_3_1::denm_pdu_descriptions::DENM>,
    },
    #[cfg(feature = "denm_2_2_1")]
    DenmV2 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::denm_2_2_1::denm_pdu_description::DENM>,
    },
    #[cfg(feature = "cam_1_4_1")]
    Cam {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::cam_1_4_1::cam_pdu_descriptions::CAM>,
    },
    #[cfg(feature = "spatem_2_2_1")]
    Spatem {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::spatem_2_2_1::spatem_pdu_descriptions::SPATEM>,
    },
    #[cfg(feature = "mapem_2_2_1")]
    Mapem {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::mapem_2_2_1::mapem_pdu_descriptions::MAPEM>,
    },
    #[cfg(feature = "ivim_2_1_1")]
    IvimV1 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::ivim_2_1_1::ivim_pdu_descriptions::IVIM>,
    },
    #[cfg(feature = "ivim_2_2_1")]
    IvimV2 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::ivim_2_2_1::ivim_pdu_descriptions::IVIM>,
    },
    #[cfg(feature = "srem_2_2_1")]
    Srem {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::srem_2_2_1::srem_pdu_descriptions::SREM>,
    },
    #[cfg(feature = "ssem_2_2_1")]
    Ssem {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::ssem_2_2_1::ssem_pdu_descriptions::SSEM>,
    },
    #[cfg(feature = "cpm_1")]
    CpmV1 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::cpm_1::cpm_pdu_descriptions::CPM>,
    },
    #[cfg(feature = "cpm_2_1_1")]
    CpmV2 {
        geonetworking: Option<Packet<'a>>,
        transport: Option<Box<TransportHeader>>,
        etsi: Box<standards::cpm_2_1_1::cpm_pdu_descriptions::CollectivePerceptionMessage>,
    },
}

#[cfg(feature = "etsi")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Headers {
    None,
    GnBtp,
    RadioTap802LlcGnBtp,
}

#[cfg(feature = "etsi")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EncodingRules {
    UPER,
    XER,
    JER,
}

#[cfg(any(
    feature = "etsi",
    all(target_arch = "wasm32", feature = "etsi", feature = "json"),
    all(test, feature = "etsi")
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

#[cfg(any(feature = "transport", feature = "etsi"))]
pub(crate) fn map_err_to_string<E: core::fmt::Debug>(error: E) -> String {
    format!("{error:?}")
}
