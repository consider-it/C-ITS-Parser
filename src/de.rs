//! C-ITS Message Decoding
//!
//! Provides Rust and wasm functions to decode messages

#![allow(non_snake_case)]

#[cfg(any(feature = "transport", feature = "_etsi"))]
use crate::map_err_to_string;
#[cfg(feature = "_etsi")]
use crate::pcap::remove_pcap_headers;
#[cfg(feature = "transport")]
use crate::transport::TransportHeader;
#[cfg(feature = "transport")]
use crate::transport::{
    decode::Decode as TransportDecode, BasicTransportAHeader, BasicTransportBHeader, IPv6Header,
};
#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
use crate::JsonItsMessage;
#[cfg(feature = "_etsi")]
use crate::{standards, Headers, ItsMessage};
#[cfg(any(
    feature = "_etsi",
    all(target_arch = "wasm32", feature = "v2x", feature = "json"),
    all(test, feature = "_etsi")
))]
use crate::{standards::cdd_2_2_1::etsi_its_cdd::ItsPduHeader, EncodingRules};
#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
use geonetworking::Encode;
#[cfg(feature = "transport")]
use geonetworking::{Decode, NextAfterCommon, Packet};
#[cfg(any(
    feature = "_etsi",
    all(target_arch = "wasm32", feature = "v2x", feature = "json"),
    all(test, feature = "_etsi")
))]
use nom::FindSubstring;
#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
use std::fmt::Write;
#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
macro_rules! btp {
    ($btp_ty:ty, $input:ident) => {
        <$btp_ty>::decode($input)
            .map_err(map_err_to_string)
            .and_then(|(rem, tp)| {
                tp.encode_to_json()
                    .map_err(map_err_to_string)
                    .map(|json| (rem, json))
            })
    };
}

#[cfg(feature = "_etsi")]
#[allow(clippy::too_many_lines)]
/// Decodes an ASN.1 message with headers. Supported encoding rules are UPER, JER, and XER. JSON and XML strings are expected as UTF-8 slices.
///
/// # Params
///  - `input`: binary input containing the ITS message
///  - `headers`: indicate which headers are present in the binary input. GeoNetworking and transport headers will be decoded and returned, other headers will be skipped.
///
/// # Notes
/// CDD 2.2.1 has changed the capitalization of the `messageId` and `stationId` data fields compared to CDD 1.3.1.
/// This didn't change the UPER encoding and therefore hasn't lead to a new protocol version in the ITS PDU header, but changes the XER and JER encodings.
/// There's a fallback in place to handle XER or JER-encoded DENM, IVIM, MAPEM, SPATEM, SREM and SSEM messages which were encoded using the old CDD.
///
/// # Errors
/// Throws string error on decoding errors.
pub fn decode(input: &'_ [u8], headers: Headers) -> Result<ItsMessage<'_>, String> {
    let (input, transport, geonetworking) = match headers {
        Headers::None => Ok((input, None, None)),
        Headers::GnBtp => {
            decode_gn_btp_headers(input).map(|(rem, tp, gn)| (rem, Some(tp), Some(gn)))
        }
        Headers::RadioTap802LlcGnBtp => remove_pcap_headers(input)
            .and_then(decode_gn_btp_headers)
            .map(|(rem, tp, gn)| (rem, Some(tp), Some(gn))),
    }?;
    let (encoding_rules, mut protocol_version, msg_type) = message_type(input)?;

    let input = match msg_type {
        // workaround to parse DENM and IVIM as XER/ JER which still uses old CDD
        1 | 6 => {
            if let Ok(data) = std::str::from_utf8(input) {
                if (data.trim_start().starts_with('<') || data.trim_start().starts_with('{'))
                    && data.contains("messageID")
                {
                    // CDD 2.2.1 is using slightly different names for some things that CDD 1.3.1, e.g.
                    // `messageID` (and other IDs) were changed to `messageId`, etc.
                    // The IVIM 2.2.1 also changed how other things are named.
                    // So we're using a fictional protocol_version 1 to fall into the right message type
                    // in the match statement below.
                    protocol_version = 1;
                }
            }
            input.to_owned()
        }
        // workaround to parse MAPEM, SPATEM, SREM, SSEM as XER/ JER which still uses old CDD
        4 | 5 | 9 | 10 => {
            if let Ok(data) = std::str::from_utf8(input) {
                // live-patch messageID and stationID in PDU header for MAPEMs, SPATEMs, SREMs and SSEMs
                // Note: an SREM may contain stationID in the requestor, but that's actually fine!!! So we shall only patch the PDU header
                if data.trim_start().starts_with('<') && data.contains("messageID") {
                    // XML end tags are not allowed to contain a space between the `</` and the name, so this find is safe to use
                    let patched_msg = match data.find("</header") {
                        Some(header_end_pos) => {
                            let (header, remains) = data.split_at(header_end_pos);

                            let patched_msg = header.replace("messageID", "messageId");
                            let patched_msg = patched_msg.replace("stationID", "stationId");

                            patched_msg + remains
                        }
                        None => data.to_string(),
                    };

                    patched_msg.as_bytes().to_owned()
                } else if data.trim_start().starts_with('{') && data.contains("messageID") {
                    let patched_msg = data.replace("messageID", "messageId");

                    // we can't easily find the end of the header in JSON, so just replace the first occurrence
                    let patched_msg = patched_msg.replacen("stationID", "stationId", 1);

                    patched_msg.as_bytes().to_owned()
                } else {
                    input.to_owned()
                }
            } else {
                input.to_owned()
            }
        }
        _ => input.to_owned(),
    };

    match (msg_type, protocol_version) {
        #[cfg(feature = "denm_2_2_1")]
        (1, 2) => encoding_rules
            .codec()
            .decode_from_binary::<standards::denm_2_2_1::denm_pdu_description::DENM>(&input)
            .map(|etsi| ItsMessage::DenmV2 {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "denm_1_3_1")]
        (1, _) => encoding_rules
            .codec()
            .decode_from_binary::<standards::denm_1_3_1::denm_pdu_descriptions::DENM>(&input)
            .map(|etsi| ItsMessage::DenmV1 {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "cam_1_4_1")]
        (2, _) => encoding_rules
            .codec()
            .decode_from_binary::<standards::cam_1_4_1::cam_pdu_descriptions::CAM>(&input)
            .map(|etsi| ItsMessage::Cam {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "spatem_2_2_1")]
        (4, _) => encoding_rules
            .codec()
            .decode_from_binary::<standards::spatem_2_2_1::spatem_pdu_descriptions::SPATEM>(&input)
            .map(|etsi| ItsMessage::Spatem {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "mapem_2_2_1")]
        (5, _) => encoding_rules
            .codec()
            .decode_from_binary::<standards::mapem_2_2_1::mapem_pdu_descriptions::MAPEM>(&input)
            .map(|etsi| ItsMessage::Mapem {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "ivim_2_2_1")]
        (6, 2) => encoding_rules
            .codec()
            .decode_from_binary::<standards::ivim_2_2_1::ivim_pdu_descriptions::IVIM>(&input)
            .map(|etsi| ItsMessage::IvimV2 {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "ivim_2_1_1")]
         (6, _) => encoding_rules
            .codec()
            .decode_from_binary::<standards::ivim_2_1_1::ivim_pdu_descriptions::IVIM>(&input)
            .map(|etsi| ItsMessage::IvimV1 {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "srem_2_2_1")]
        (9, _) => encoding_rules
            .codec()
            .decode_from_binary::<standards::srem_2_2_1::srem_pdu_descriptions::SREM>(&input)
            .map(|etsi| ItsMessage::Srem {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "ssem_2_2_1")]
        (10, _) => encoding_rules
            .codec()
            .decode_from_binary::<standards::ssem_2_2_1::ssem_pdu_descriptions::SSEM>(&input)
            .map(|etsi| ItsMessage::Ssem {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "cpm_2_1_1")]
        (14, 2) => encoding_rules
            .codec()
            .decode_from_binary::<standards::cpm_2_1_1::cpm_pdu_descriptions::CollectivePerceptionMessage>(&input)
            .map(|etsi| ItsMessage::CpmV2 {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        #[cfg(feature = "cpm_1")]
        (14, _) => encoding_rules
            .codec()
            .decode_from_binary::<standards::cpm_1::cpm_pdu_descriptions::CPM>(&input)
            .map(|etsi| ItsMessage::CpmV1 {
                geonetworking,
                transport,
                etsi: Box::new(etsi)
            }),
        (message_i_d, _) => {
            return Err(format!(
                "Unsupported ITS message type: Found message id {message_i_d}."
            ))
        }
    }.map_err(map_err_to_string)
}

#[cfg(feature = "transport")]
/// Decodes the GeoNetworking and BTP headers and returns the remaining data
///
/// # Errors
/// Returns human-readable error descriptions when decoding failed
pub fn decode_gn_btp_headers(
    input: &'_ [u8],
) -> Result<(&'_ [u8], Box<TransportHeader>, Packet<'_>), String> {
    let result = Packet::decode(input).map_err(map_err_to_string)?;
    let payload = match &result.decoded {
        Packet::Unsecured { payload, .. } => *payload,
        s @ Packet::Secured { .. } => s
            .secured_payload_after_gn()
            .ok_or("No payload in secured geonetworking header!")?,
    };
    let (remaining, tp) = match result.decoded.common().next_header {
        NextAfterCommon::Any => {
            Err("Currently, only BTP and IPv6 Headers can be decoded!".to_string())
        }
        NextAfterCommon::BTPA => BasicTransportAHeader::decode(payload)
            .map(|(rem, btpa)| (rem, TransportHeader::BtpA(btpa)))
            .map_err(map_err_to_string),
        NextAfterCommon::BTPB => BasicTransportBHeader::decode(payload)
            .map(|(rem, btpb)| (rem, TransportHeader::BtpB(btpb)))
            .map_err(map_err_to_string),
        NextAfterCommon::IPv6 => IPv6Header::decode(payload)
            .map(|(rem, ipv6)| (rem, TransportHeader::IPv6(Box::new(ipv6))))
            .map_err(map_err_to_string),
    }?;
    Ok((remaining, Box::new(tp), result.decoded))
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
#[wasm_bindgen(js_name = decode)]
/// Decodes an ITS message of undefined type.
/// Tries to parse the ITS PDU header to read the message ID that identifies the message type.
/// ### Params
///  - `message`: binary input containing the ITS message
///  - `headersPresent`: indicate which headers are present in the binary input. GeoNetworking and transport headers will be decoded and returned, other headers will be skipped.
///  - `outputEncodingRules`: ASN.1 encoding rules that will be used for re-encoding the message in the `JsonItsMessage`'s `its` field. (UPER output will be rendered as a UTF-8 hex string)
/// Throws string error on decoding errors.
pub fn decode_to(
    message: &[u8],
    headersPresent: Headers,
    outputEncodingRules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(message, headersPresent)?;
    let (input_encoding_rules, protocol_version, message_type) = message_type(input)?;
    let (msg_ty, decoded) = match (message_type, protocol_version) {
        (1, 2) => (
            1,
            decode_denm(
                input,
                Some(211),
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (1, _) => (
            1,
            decode_denm(
                input,
                Some(131),
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (2, _) => (
            2,
            decode_cam(
                input,
                None,
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (4, _) => (
            4,
            decode_spatem(
                input,
                None,
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (5, _) => (
            5,
            decode_mapem(
                input,
                None,
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (6, 2) => (
            6,
            decode_ivim(
                input,
                Some(221),
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (6, _) => (
            6,
            decode_ivim(
                input,
                Some(131),
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (9, _) => (
            9,
            decode_srem(
                input,
                None,
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (10, _) => (
            10,
            decode_ssem(
                input,
                None,
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (14, 2) => (
            14,
            decode_cpm(
                input,
                Some(211),
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (14, _) => (
            14,
            decode_cpm(
                input,
                Some(131),
                Headers::None,
                input_encoding_rules,
                outputEncodingRules,
            )?
            .its,
        ),
        (message_i_d, _) => {
            return Err(format!(
                "Unsupported ITS message type: Found message id {message_i_d}."
            ));
        }
    };
    etsi_json.its = decoded;
    etsi_json.message_type = msg_ty;
    Ok(etsi_json)
}

#[cfg(any(
    feature = "_etsi",
    all(target_arch = "wasm32", feature = "v2x", feature = "json"),
    all(test, feature = "_etsi")
))]
fn message_type(input: &[u8]) -> Result<(EncodingRules, u8, u8), String> {
    let encoding_rules = match std::str::from_utf8(input) {
        Ok(s) if s.trim_start().starts_with('<') => EncodingRules::XER,
        Ok(s) if s.trim_start().starts_with('{') => EncodingRules::JER,
        _ => EncodingRules::UPER,
    };
    match encoding_rules {
        EncodingRules::XER => {
            let message_id_start = input
                .find_substring("messageID>")
                .or(input.find_substring("messageId>"))
                .ok_or("Failed to determine message ID.")?
                + 10;
            let message_id_end = (&input[message_id_start..])
                .find_substring("</")
                .ok_or("Failed to determine message ID.")?
                + message_id_start;
            let message_id = std::str::from_utf8(&input[message_id_start..message_id_end])
                .map_err(map_err_to_string)?
                .trim()
                .parse()
                .map_err(map_err_to_string)?;
            let protocol_version_start = input
                .find_substring("protocolVersion>")
                .ok_or("Failed to determine protocol version.")?
                + 16;
            let protocol_version_end = (&input[protocol_version_start..])
                .find_substring("</")
                .ok_or("Failed to determine protocol version.")?
                + protocol_version_start;
            let protocol_version =
                std::str::from_utf8(&input[protocol_version_start..protocol_version_end])
                    .map_err(map_err_to_string)?
                    .trim()
                    .parse()
                    .map_err(map_err_to_string)?;
            Ok((encoding_rules, protocol_version, message_id))
        }
        EncodingRules::JER => {
            let message_id = input
                .find_substring("messageID\":")
                .or(input.find_substring("messageId\":"))
                .ok_or(String::from("Failed to determine message ID."))
                .and_then(|start| {
                    let mut end = start + 11;
                    let mut value = input[end] as char;
                    while end < input.len() - 1 && (value.is_whitespace() || value.is_numeric()) {
                        end += 1;
                        value = input[end] as char;
                    }
                    std::str::from_utf8(&input[(start + 11)..end])
                        .map_err(map_err_to_string)
                        .and_then(|s| s.trim().parse::<u8>().map_err(map_err_to_string))
                })?;
            let protocol_version = input
                .find_substring("protocolVersion\":")
                .ok_or(String::from("Failed to determine message ID."))
                .and_then(|start| {
                    let mut end = start + 17;
                    let mut value = input[end] as char;
                    while end < input.len() - 1 && (value.is_whitespace() || value.is_numeric()) {
                        end += 1;
                        value = input[end] as char;
                    }
                    std::str::from_utf8(&input[(start + 17)..end])
                        .map_err(map_err_to_string)
                        .and_then(|s| s.trim().parse::<u8>().map_err(map_err_to_string))
                })?;
            Ok((encoding_rules, protocol_version, message_id))
        }
        EncodingRules::UPER => EncodingRules::UPER
            .codec()
            .decode_from_binary::<ItsPduHeader>(input)
            .map(|header| {
                (
                    encoding_rules,
                    header.protocol_version.0,
                    header.message_id.0,
                )
            })
            .map_err(map_err_to_string),
    }
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_denm(
    denm: &[u8],
    mut version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(denm, headers_present)?;
    if version.is_none() {
        version = match input.first() {
            Some(1) => Some(131),
            Some(2) => Some(211),
            _ => None,
        };
    }
    etsi_json.its = match version {
        Some(131) => Some(transcode::<
            standards::denm_1_3_1::denm_pdu_descriptions::DENM,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        None | Some(221) => Some(
            transcode::<standards::denm_2_2_1::denm_pdu_description::DENM>(
                input,
                input_encoding_rules,
                output_encoding_rules,
            ),
        )
        .transpose(),
        _ => {
            return Err(
                "Unsupported DENM version: Supported DENM versions are 131 and 221.".to_string(),
            );
        }
    }?;
    Ok(etsi_json)
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_cam(
    cam: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(cam, headers_present)?;
    etsi_json.its = match version {
        None | Some(141) => Some(
            transcode::<standards::cam_1_4_1::cam_pdu_descriptions::CAM>(
                input,
                input_encoding_rules,
                output_encoding_rules,
            ),
        )
        .transpose(),
        _ => return Err("Unsupported DENM version: Supported CAM version is 141.".to_string()),
    }?;
    Ok(etsi_json)
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_mapem(
    mapem: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(mapem, headers_present)?;
    etsi_json.its = match version {
        None | Some(221) => Some(transcode::<
            standards::mapem_2_2_1::mapem_pdu_descriptions::MAPEM,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        _ => return Err("Unsupported MAPEM version: Supported MAPEM version is 221.".to_string()),
    }?;
    Ok(etsi_json)
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_spatem(
    spatem: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(spatem, headers_present)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode::<
            standards::spatem_2_2_1::spatem_pdu_descriptions::SPATEM,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        _ => {
            return Err("Unsupported SPATEM version: Supported SPATEM version is 131.".to_string());
        }
    }?;
    Ok(etsi_json)
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_ivim(
    ivim: &[u8],
    mut version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(ivim, headers_present)?;
    if version.is_none() {
        version = match input.first() {
            Some(1) => Some(211),
            Some(2) => Some(221),
            _ => None,
        };
    }
    etsi_json.its = match version {
        Some(131) | Some(211) => Some(transcode::<
            standards::ivim_2_1_1::ivim_pdu_descriptions::IVIM,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        None | Some(221) => Some(transcode::<
            standards::ivim_2_2_1::ivim_pdu_descriptions::IVIM,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        _ => {
            return Err(
                "Unsupported IVIM version: Supported IVIM versions are 131, 211 and 221."
                    .to_string(),
            );
        }
    }?;
    Ok(etsi_json)
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_srem(
    srem: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(srem, headers_present)?;
    etsi_json.its = match version {
        None | Some(221) => Some(transcode::<
            standards::srem_2_2_1::srem_pdu_descriptions::SREM,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        _ => return Err("Unsupported SREM version: Supported SREM version is 221.".to_string()),
    }?;
    Ok(etsi_json)
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_cpm(
    cpm: &[u8],
    mut version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(cpm, headers_present)?;
    if version.is_none() {
        version = match input.first() {
            Some(1) => Some(131),
            Some(2) => Some(211),
            _ => None,
        };
    }
    etsi_json.its = match version {
        None | Some(211) => Some(transcode::<
            standards::cpm_2_1_1::cpm_pdu_descriptions::CollectivePerceptionMessage,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        Some(131) => Some(transcode::<standards::cpm_1::cpm_pdu_descriptions::CPM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        _ => {
            return Err(
                "Unsupported CPM version: Supported CPM versions are 131 and 211.".to_string(),
            );
        }
    }?;
    Ok(etsi_json)
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_ssem(
    ssem: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<JsonItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(ssem, headers_present)?;
    etsi_json.its = match version {
        None | Some(221) => Some(transcode::<
            standards::ssem_2_2_1::ssem_pdu_descriptions::SSEM,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        _ => return Err("Unsupported SSEM version: Supported SSEM version is 221.".to_string()),
    }?;
    Ok(etsi_json)
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
pub fn optionally_decode_headers(
    input: &[u8],
    headers: Headers,
) -> Result<(&[u8], JsonItsMessage), String> {
    match headers {
        Headers::None => Ok((input, JsonItsMessage::default())),
        Headers::GnBtp => transcode_gn_tp_to_json(input),
        Headers::RadioTap802LlcGnBtp => {
            remove_pcap_headers(input).and_then(transcode_gn_tp_to_json)
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn transcode_gn_tp_to_json(input: &[u8]) -> Result<(&[u8], JsonItsMessage), String> {
    decode_geonetworking_header(input).and_then(|(remaining, gn_json, next_header)| {
        decode_transport_header(remaining, next_header).map(|(rem, tp)| {
            (
                rem,
                JsonItsMessage {
                    geonetworking: Some(gn_json),
                    transport: Some(tp),
                    ..Default::default()
                },
            )
        })
    })
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_geonetworking_header(input: &[u8]) -> Result<(&[u8], String, NextAfterCommon), String> {
    let result = Packet::decode(input).map_err(map_err_to_string)?;
    let gn_json = result.decoded.encode_to_json().map_err(map_err_to_string)?;
    match result.decoded {
        Packet::Unsecured {
            common, payload, ..
        } => Ok((payload, gn_json, common.next_header)),
        p => p
            .secured_payload_after_gn()
            .ok_or("Secured GeoNetworking Packet carries no data!".into())
            .map(|payload| (payload, gn_json, p.common().next_header)),
    }
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn decode_transport_header(
    input: &[u8],
    header_type: NextAfterCommon,
) -> Result<(&[u8], String), String> {
    match header_type {
        NextAfterCommon::Any => {
            Err("Currently, only BTP and IPv6 Headers can be decoded!".to_string())
        }
        NextAfterCommon::BTPA => btp![BasicTransportAHeader, input],
        NextAfterCommon::BTPB => btp![BasicTransportBHeader, input],
        NextAfterCommon::IPv6 => {
            let (remaining, ipv6) = IPv6Header::decode(input).map_err(map_err_to_string)?;
            Ok((remaining, to_ipv6_debug(ipv6)))
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn transcode<T: rasn::Decode + rasn::Encode>(
    input: &[u8],
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<String, String> {
    if let (EncodingRules::UPER, EncodingRules::UPER) =
        (input_encoding_rules, output_encoding_rules)
    {
        return input.iter().try_fold(String::new(), |mut acc, byte| {
            write!(&mut acc, "{byte:02X?}")
                .map_err(map_err_to_string)
                .map(|_| acc)
        });
    }
    let decoded: T = input_encoding_rules
        .codec()
        .decode_from_binary(input)
        .map_err(map_err_to_string)?;
    match output_encoding_rules {
        EncodingRules::UPER => rasn::uper::encode(&decoded)
            .map(hex::encode)
            .map_err(map_err_to_string),
        o => o
            .codec()
            .encode_to_string(&decoded)
            .map_err(map_err_to_string),
    }
}

#[cfg(all(target_arch = "wasm32", feature = "v2x", feature = "json"))]
fn to_ipv6_debug(ipv6: IPv6Header) -> String {
    format!(r#"{{"ipv6Debug":"{ipv6:?}"}}"#)
}

#[cfg(all(test, feature = "_etsi"))]
mod tests {
    use crate::de::message_type;

    #[test]
    fn recognizes_message_type_and_version() {
        assert_eq!((crate::EncodingRules::XER, 2,14), message_type("<CPM><header><protocolVersion>2</protocolVersion><messageID>14</messageID><stationID>".as_bytes()).unwrap());
        assert_eq!(
            (crate::EncodingRules::XER, 1, 5),
            message_type(
                r#"<?xml version="1.0"?><MAPEM><header><protocolVersion>  1  </protocolVersion><messageID>
        5
        </messageID><stationID>"#
                    .as_bytes()
            )
            .unwrap()
        );
        assert_eq!(
            (crate::EncodingRules::JER, 2, 2),
            message_type(
                r#"{"header":{"protocolVersion":2,"messageID":2,"stationID":2624309139}"#
                    .as_bytes()
            )
            .unwrap()
        );
        assert_eq!(
            (crate::EncodingRules::JER, 1, 9),
            message_type(
                r#"{
                    "header": {
                            "protocolVersion": 1,
                            "messageID": 9,
                            "stationID": 2624309139
                    }"#
                .as_bytes()
            )
            .unwrap()
        );
    }
}
