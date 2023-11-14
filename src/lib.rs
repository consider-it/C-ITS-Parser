#![no_std]
extern crate alloc;

pub mod standards;

use core::str::Bytes;

use alloc::{fmt::format, format, string::String};
use etsi_geonetworking::{
    Decodable, Decode, Encode, Header as GeoNetworkingHeader, NextAfterCommon,
};
use etsi_transports::{BasicTransportAHeader, BasicTransportBHeader, Decode as TransportDecode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct EtsiJson {
    pub geonetworking: Option<String>,
    pub transport: Option<String>,
    pub its: Option<String>,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn decodeDenmToJson(
    denm: &[u8],
    version: u32,
    includesGnHeader: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, geonetworking, next_header) = includesGnHeader
        .then(|| decode_geonetworking_header(denm))
        .transpose()?
        .unwrap_or((0, None, None));
    let denm = &denm[bytes_read..];
    let (bytes_read, transport) = next_header
        .map(|next| decode_transport_header(denm, next))
        .transpose()?
        .unwrap_or((0, None));
    match version {
        131 => rasn::jer::encode(
            &rasn::uper::decode::<standards::denm_1_3_1::DENM>(&denm[bytes_read..])
                .map_err(|e| format!("{e:?}"))?,
        )
        .map_err(|e| format!("{e:?}"))
        .map(|its| EtsiJson {
            geonetworking,
            transport,
            its: Some(its),
        }),
        211 => rasn::jer::encode(
            &rasn::uper::decode::<standards::denm_2_1_1::DENM>(&denm[bytes_read..])
                .map_err(|e| format!("{e:?}"))?,
        )
        .map_err(|e| format!("{e:?}"))
        .map(|its| EtsiJson {
            geonetworking,
            transport,
            its: Some(its),
        }),
        _ => Err(format!("Supported version IDs are 131 and 211.")),
    }
}

fn decode_geonetworking_header(
    input: &[u8],
) -> Result<(usize, Option<String>, Option<NextAfterCommon>), String> {
    let input_bits = input.as_decodable();
    let length_before = input_bits.len();
    let (remaining, next_header, geonetworking) = GeoNetworkingHeader::decode(input_bits)
        .map(|(res, gn)| {
            (
                res,
                Some(gn.common.next_header),
                Some(gn.encode_to_json().map_err(|e| format!("{e:?}"))),
            )
        })
        .map_err(|e| format!("{e:?}"))?;
    let bits_read = length_before - remaining.len();
    if bits_read % 8 != 0 {
        return Err(format!(
            "Not bit-aligned after decoding GeoNetworking header!"
        ));
    }
    Ok((bits_read / 8, geonetworking.transpose()?, next_header))
}

fn decode_transport_header(
    input: &[u8],
    header_type: NextAfterCommon,
) -> Result<(usize, Option<String>), String> {
    match header_type {
        NextAfterCommon::Any => Err(format!(
            "Currently, only BTP and IPv6 Headers can be decoded!"
        )),
        NextAfterCommon::BTPA => BasicTransportAHeader::decode(input)
            .map_err(|e| format!("{e:?}"))?
            .1
            .encode_to_json()
            .map_err(|e| format!("{e:?}"))
            .map(|json| (4, Some(json))),
        NextAfterCommon::BTPB => BasicTransportBHeader::decode(input)
            .map_err(|e| format!("{e:?}"))?
            .1
            .encode_to_json()
            .map_err(|e| format!("{e:?}"))
            .map(|json| (4, Some(json))),
        NextAfterCommon::IPv6 => todo!(),
    }
}
