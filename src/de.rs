#![allow(non_snake_case)]
use crate::{
    pcap::remove_pcap_headers,
    transport::{
        decode::Decode as TransportDecode, BasicTransportAHeader, BasicTransportBHeader, IPv6Header,
    },
    EncodingRules, Headers,
};
use geonetworking::{Decode, Encode, NextAfterCommon, Packet};
use nom::FindSubstring;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    map_err_to_string,
    standards::is_1_3_1::{self, ItsPduHeader},
    ItsMessage,
};

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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decode))]
/// Decodes an ITS message of undefined type.
/// Tries to parse the ITS PDU header to read the message ID that identifies the message type.
/// ### Params
///  - `message`: binary input containing the ITS message
///  - `headersPresent`: indicate which headers are present in the binary input. Geonetworking and transport headers will be decoded and returned, other headers will be skipped.
///  - `inputEncodingRules`: ASN.1 encoding rules used to encode the ITS message in the input
///  - `outputEncodingRules`: ASN.1 encoding rules that will be used for re-encoding the message in the `ItsMessage`'s `its` field. (UPER output will be rendered as a UTF-8 hex string)
/// Throws string error on decoding errors.
pub fn decode(
    message: &[u8],
    headersPresent: Headers,
    inputEncodingRules: EncodingRules,
    outputEncodingRules: EncodingRules,
) -> Result<ItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(message, headersPresent)?;
    let (protocol_version, message_type) = message_type(inputEncodingRules, input)?;
    let (msg_ty, decoded) = match (message_type, protocol_version) {
        (1, 2) => (
            1,
            decode_denm(
                input,
                Some(211),
                Headers::None,
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
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
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        (message_i_d, _) => {
            return Err(format!(
                "Unsupported ITS message type: Found message id {message_i_d}."
            ))
        }
    };
    etsi_json.its = decoded;
    etsi_json.message_type = msg_ty;
    Ok(etsi_json)
}

fn message_type(inputEncodingRules: EncodingRules, input: &[u8]) -> Result<(u8, u8), String> {
    match inputEncodingRules {
        EncodingRules::XER => {
            let message_id_start = input
                .find_substring("<messageID>")
                .or(input.find_substring("<messageId>"))
                .ok_or("Failed to determine message ID.")?;
            let message_id_end = input
                .find_substring("</messageID>")
                .or(input.find_substring("</messageId>"))
                .ok_or("Failed to determine message ID.")?;
            let message_id = std::str::from_utf8(&input[(message_id_start + 11)..message_id_end])
                .map_err(map_err_to_string)?
                .trim()
                .parse()
                .map_err(map_err_to_string)?;
            let protocol_version_start = input
                .find_substring("<protocolVersion>")
                .ok_or("Failed to determine protocol version.")?;
            let protocol_version_end = input
                .find_substring("</protocolVersion>")
                .ok_or("Failed to determine protocol version.")?;
            let protocol_version =
                std::str::from_utf8(&input[(protocol_version_start + 17)..protocol_version_end])
                    .map_err(map_err_to_string)?
                    .trim()
                    .parse()
                    .map_err(map_err_to_string)?;
            Ok((protocol_version, message_id))
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
            Ok((protocol_version, message_id))
        }
        EncodingRules::UPER => inputEncodingRules
            .codec()
            .decode_from_binary::<ItsPduHeader>(input)
            .map(|header| (header.protocol_version, header.message_i_d))
            .map_err(map_err_to_string),
    }
}

fn decode_denm(
    denm: &[u8],
    mut version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<ItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(denm, headers_present)?;
    if version.is_none() {
        version = match input.first() {
            Some(1) => Some(131),
            Some(2) => Some(211),
            _ => None,
        };
    }
    etsi_json.its = match version {
        Some(131) => Some(transcode::<crate::standards::denm_1_3_1::DENM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        None | Some(211) => Some(transcode::<
            crate::standards::denm_2_1_1::d_e_n_m__p_d_u__description::DENM,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        _ => {
            return Err(
                "Unsupported DENM version: Supported DENM versions are 131 and 211.".to_string(),
            )
        }
    }?;
    Ok(etsi_json)
}

fn decode_cam(
    cam: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<ItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(cam, headers_present)?;
    etsi_json.its = match version {
        None | Some(141) => Some(transcode::<crate::standards::cam_1_4_1::CAM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        _ => return Err("Unsupported DENM version: Supported CAM version is 141.".to_string()),
    }?;
    Ok(etsi_json)
}

fn decode_mapem(
    mapem: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<ItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(mapem, headers_present)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode::<crate::standards::is_1_3_1::MAPEM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        _ => return Err("Unsupported MAPEM version: Supported MAPEM version is 131.".to_string()),
    }?;
    Ok(etsi_json)
}

fn decode_spatem(
    spatem: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<ItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(spatem, headers_present)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode::<crate::standards::is_1_3_1::SPATEM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported SPATEM version: Supported SPATEM version is 131.".to_string())
        }
    }?;
    Ok(etsi_json)
}

fn decode_ivim(
    ivim: &[u8],
    mut version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<ItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(ivim, headers_present)?;
    if version.is_none() {
        version = match input.first() {
            Some(1) => Some(131),
            Some(2) => Some(221),
            _ => None,
        };
    }
    etsi_json.its = match version {
        Some(131) => Some(transcode::<is_1_3_1::IVIM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        None | Some(221) => Some(transcode::<crate::standards::ivim_2_2_1::IVIM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        _ => {
            return Err(
                "Unsupported IVIM version: Supported IVIM versions are 131 and 221.".to_string(),
            )
        }
    }?;
    Ok(etsi_json)
}

fn decode_srem(
    srem: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<ItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(srem, headers_present)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode::<crate::standards::is_1_3_1::SREM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        _ => return Err("Unsupported SREM version: Supported SREM version is 131.".to_string()),
    }?;
    Ok(etsi_json)
}

fn decode_cpm(
    cpm: &[u8],
    mut version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<ItsMessage, String> {
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
            crate::standards::cpm_2_1_1::c_p_m__p_d_u__descriptions::CollectivePerceptionMessage,
        >(input, input_encoding_rules, output_encoding_rules))
        .transpose(),
        Some(131) => Some(transcode::<crate::standards::is_1_3_1::CPM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        _ => {
            return Err(
                "Unsupported CPM version: Supported CPM versions are 131 and 211.".to_string(),
            )
        }
    }?;
    Ok(etsi_json)
}

fn decode_ssem(
    ssem: &[u8],
    version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<ItsMessage, String> {
    let (input, mut etsi_json) = optionally_decode_headers(ssem, headers_present)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode::<crate::standards::is_1_3_1::SSEM>(
            input,
            input_encoding_rules,
            output_encoding_rules,
        ))
        .transpose(),
        _ => return Err("Unsupported SSEM version: Supported SSEM version is 131.".to_string()),
    }?;
    Ok(etsi_json)
}

fn optionally_decode_headers(
    input: &[u8],
    headers: Headers,
) -> Result<(&[u8], ItsMessage), String> {
    match headers {
        Headers::None => Ok((input, ItsMessage::default())),
        Headers::GnBtp => decode_gn_and_btp(input),
        Headers::RadioTap802LlcGnBtp => remove_pcap_headers(input).and_then(decode_gn_and_btp),
    }
}

fn decode_gn_and_btp(input: &[u8]) -> Result<(&[u8], ItsMessage), String> {
    decode_geonetworking_header(input).and_then(|(remaining, gn_json, next_header)| {
        decode_transport_header(remaining, next_header).map(|(rem, tp)| {
            (
                rem,
                ItsMessage {
                    geonetworking: Some(gn_json),
                    transport: Some(tp),
                    ..Default::default()
                },
            )
        })
    })
}

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

fn transcode<T: rasn::Decode + rasn::Encode>(
    input: &[u8],
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<String, String> {
    if let (EncodingRules::UPER, EncodingRules::UPER) =
        (input_encoding_rules, output_encoding_rules)
    {
        return Ok(input.iter().map(|byte| format!("{byte:02X?}")).collect());
    }
    let decoded: T = input_encoding_rules
        .codec()
        .decode_from_binary(input)
        .map_err(map_err_to_string)?;
    output_encoding_rules
        .codec()
        .encode_to_string(&decoded)
        .map_err(map_err_to_string)
}

fn to_ipv6_debug(ipv6: IPv6Header) -> String {
    format!(r#"{{"ipv6Debug":"{ipv6:?}"}}"#)
}

#[cfg(test)]
mod tests {
    use crate::de::message_type;

    #[test]
    fn recognizes_message_type_and_version() {
        assert_eq!((2,14), message_type(crate::EncodingRules::XER, "<CPM><header><protocolVersion>2</protocolVersion><messageID>14</messageID><stationID>".as_bytes()).unwrap());
        assert_eq!(
            (1, 5),
            message_type(
                crate::EncodingRules::XER,
                r#"<?xml version="1.0"?><MAPEM><header><protocolVersion>  1  </protocolVersion><messageID>
        5
        </messageID><stationID>"#
                    .as_bytes()
            )
            .unwrap()
        );
        assert_eq!(
            (2, 2),
            message_type(
                crate::EncodingRules::JER,
                r#"{"header":{"protocolVersion":2,"messageID":2,"stationID":2624309139}"#
                    .as_bytes()
            )
            .unwrap()
        );
        assert_eq!(
            (1, 9),
            message_type(
                crate::EncodingRules::JER,
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
