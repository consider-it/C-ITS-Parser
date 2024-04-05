use core::fmt::Debug;

use serde::{Deserialize, Serialize};

pub(crate) mod decode;
pub(crate) mod encode;

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
