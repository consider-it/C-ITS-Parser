//! GeoNetworking Transport Layer Parser

use alloc::string::ToString;
use core::fmt::Debug;

use decode::Decode;
use encode::Encode;
use geonetworking::NextAfterCommon;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use crate::map_err_to_string;

pub(crate) mod decode;
pub(crate) mod encode;

#[derive(Debug, Clone, PartialEq)]
/// GeoNetworking "next header" of the Common Header
pub enum TransportHeader {
    /// Transport protocol (BTP-A for interactive packet transport) as defined in ETSI EN 302 636-5-1
    BtpA(BasicTransportAHeader),
    /// Transport protocol (BTP-B for non-interactive packet transport) as defined in ETSI EN 302 636-5-1
    BtpB(BasicTransportBHeader),
    /// IPv6 header as defined in ETSI EN 302 636-6-1
    IPv6(alloc::boxed::Box<IPv6Header>),
}

impl TransportHeader {
    /// Decodes a GeoNetworking Transport Header from binary buffer
    ///
    /// The "next header" type needs to be supplied in `next_header`.
    /// Returns the remaining data after the transport header.
    ///
    /// # Errors
    /// Returns a human-readable error when parsing failed of unsupported header type was selected.
    pub fn decode_with_gn_next_header(
        next_header: NextAfterCommon,
        bytes: &[u8],
    ) -> Result<(&[u8], TransportHeader), alloc::string::String> {
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
                .map(|(rem, ipv6)| (rem, TransportHeader::IPv6(alloc::boxed::Box::new(ipv6))))
                .map_err(map_err_to_string),
        }
    }

    /// Encodes a GeoNetworking Transport Header returning the binary buffer
    ///
    /// Note: Encoding IPv6 headers is not supported.
    ///
    /// # Errors
    /// Returns a human-readable error when encoding failed.
    pub fn encode(&self) -> Result<alloc::vec::Vec<u8>, alloc::string::String> {
        match self {
            TransportHeader::BtpA(a) => a.encode().map_err(map_err_to_string),
            TransportHeader::BtpB(b) => b.encode().map_err(map_err_to_string),
            TransportHeader::IPv6(_) => Err(alloc::string::String::from(
                "Encoding IPv6 headers is unsupported!",
            )),
        }
    }

    #[cfg(feature = "json")]
    /// Encodes a GeoNetworking Transport Header as a JSON representation
    ///
    /// Note: Encoding IPv6 headers is not supported.
    ///
    /// # Errors
    /// Returns a human-readable error when encoding failed.
    pub fn encode_to_json(&self) -> Result<alloc::string::String, alloc::string::String> {
        match self {
            TransportHeader::BtpA(a) => a.encode_to_json().map_err(map_err_to_string),
            TransportHeader::BtpB(b) => b.encode_to_json().map_err(map_err_to_string),
            TransportHeader::IPv6(_) => Err(alloc::string::String::from(
                "Encoding IPv6 headers is unsupported!",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
/// An ETSI EN 302 636-5-1 BTP-A header
pub struct BasicTransportAHeader {
    /// identifies the protocol entity at the destination's ITS facilities layer
    pub destination_port: u16,
    /// identifies the protocol entity at the source's ITS facilities layer
    pub source_port: u16,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
/// An ETSI EN 302 636-5-1 BTP-B header
pub struct BasicTransportBHeader {
    /// It identifies the protocol entity at the ITS facilities layer in the destination.
    /// For well-known ports it shall be set to a value corresponding to the
    /// identified facilities layer service as specified the values in ETSI TS 103 248
    pub destination_port: u16,
    /// It provides additional information. If Destination port is a well-known port
    /// and the field value is specified in ETSI TS 103 248, it shall be set to a value
    /// corresponding to the identified facilities layer service as specified in ETSI TS 103 248.
    /// Default setting is 0
    pub destination_port_info: u16,
}

#[derive(Debug, Clone, PartialEq)]
/// An ETSI EN 302 636-6-1 IPv6 header
pub struct IPv6Header {
    pub ip: Option<etherparse::NetHeaders>,
    pub link: Option<etherparse::LinkHeader>,
    pub transport: Option<etherparse::TransportHeader>,
    // note: etherparse supports multiple link header extensions, but we assume 1 or none
    pub link_ext: Option<etherparse::LinkExtHeader>,
}
