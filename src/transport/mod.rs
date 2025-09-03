use core::fmt::Debug;

use decode::Decode;
use encode::Encode;
use geonetworking::NextAfterCommon;
use serde::{Deserialize, Serialize};

use crate::map_err_to_string;

pub(crate) mod decode;
pub(crate) mod encode;

#[derive(Debug, Clone, PartialEq)]
pub enum TransportHeader {
    BtpA(BasicTransportAHeader),
    BtpB(BasicTransportBHeader),
    IPv6(Box<IPv6Header>),
}

impl TransportHeader {
    #[allow(clippy::missing_errors_doc)]
    pub fn decode_with_gn_next_header(
        next_header: NextAfterCommon,
        bytes: &[u8],
    ) -> Result<(&[u8], TransportHeader), String> {
        match next_header {
            NextAfterCommon::Any => {
                Err("Currently, only BTP and IPv6 Headers can be decoded!".to_string())
            }
            NextAfterCommon::BTPA => BasicTransportAHeader::decode(bytes)
                .map(|(rem, btpa)| (rem, TransportHeader::BtpA(btpa)))
                .map_err(map_err_to_string),
            NextAfterCommon::BTPB => BasicTransportBHeader::decode(bytes)
                .map(|(rem, btpb)| (rem, TransportHeader::BtpB(btpb)))
                .map_err(map_err_to_string),
            NextAfterCommon::IPv6 => IPv6Header::decode(bytes)
                .map(|(rem, ipv6)| (rem, TransportHeader::IPv6(Box::new(ipv6))))
                .map_err(map_err_to_string),
        }
    }

    #[allow(clippy::missing_errors_doc, reason = "no documentation present")]
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        match self {
            TransportHeader::BtpA(a) => a.encode().map_err(map_err_to_string),
            TransportHeader::BtpB(b) => b.encode().map_err(map_err_to_string),
            TransportHeader::IPv6(_) => Err(String::from("Encoding IPv6 headers is unsupported!")),
        }
    }

    #[allow(clippy::missing_errors_doc, reason = "no documentation present")]
    pub fn encode_to_json(&self) -> Result<String, String> {
        match self {
            TransportHeader::BtpA(a) => a.encode_to_json().map_err(map_err_to_string),
            TransportHeader::BtpB(b) => b.encode_to_json().map_err(map_err_to_string),
            TransportHeader::IPv6(_) => Err(String::from("Encoding IPv6 headers is unsupported!")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicTransportAHeader {
    /// identifies the protocol entity at the destination's ITS facilities layer
    pub destination_port: u16,
    /// identifies the protocol entity at the source's ITS facilities layer
    pub source_port: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicTransportBHeader {
    /// It identifies the protocol entity at the ITS facilities layer in the destination.
    /// For well-known ports it shall be set to a value corresponding to the
    /// identified facilities layer service as specified the values in ETSI TS 103 248 [6]
    pub destination_port: u16,
    /// It provides additional information. If Destination port is a well-known port
    /// and the field value is specified in ETSI TS 103 248 [6], it shall be set to a value
    /// corresponding to the identified facilities layer service as specified in ETSI TS 103 248 [6].
    /// Default setting is 0
    pub destination_port_info: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IPv6Header {
    pub ip: Option<etherparse::IpHeader>,
    pub link: Option<etherparse::Ethernet2Header>,
    pub transport: Option<etherparse::TransportHeader>,
    pub vlan: Option<etherparse::VlanHeader>,
}
