#![allow(non_snake_case)]
use crate::{
    pcap::remove_pcap_headers,
    transport::{
        decode::Decode as TransportDecode, BasicTransportAHeader, BasicTransportBHeader, IPv6Header,
    },
    EncodingRules, Headers,
};
use geonetworking::{Decode, Encode, NextAfterCommon, Packet};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    map_err_to_string,
    standards::is_1_3_1::{self, ItsPduHeader},
    EtsiJson,
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
/// Set `includesHeaders` to `false` if the given binary message does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode(
    message: &[u8],
    headersPresent: Headers,
    inputEncodingRules: EncodingRules,
    outputEncodingRules: EncodingRules,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(message, headersPresent)?;
    let message_type = inputEncodingRules.codec().decode_from_binary(input);
    let (msg_ty, decoded) = match message_type {
        Ok(ItsPduHeader {
            message_i_d: 1,
            protocol_version: 2,
            ..
        }) => (
            Some(1),
            decode_denm(
                input,
                Some(211),
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader { message_i_d: 1, .. }) => (
            Some(1),
            decode_denm(
                input,
                Some(131),
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader { message_i_d: 2, .. }) => (
            Some(2),
            decode_cam(
                input,
                None,
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader { message_i_d: 4, .. }) => (
            Some(4),
            decode_spatem(
                input,
                None,
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader { message_i_d: 5, .. }) => (
            Some(5),
            decode_mapem(
                input,
                None,
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader {
            message_i_d: 6,
            protocol_version: 2,
            ..
        }) => (
            Some(6),
            decode_ivim(
                input,
                Some(221),
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader { message_i_d: 6, .. }) => (
            Some(6),
            decode_ivim(
                input,
                Some(131),
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader { message_i_d: 9, .. }) => (
            Some(9),
            decode_srem(
                input,
                None,
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader {
            message_i_d: 10, ..
        }) => (
            Some(10),
            decode_ssem(
                input,
                None,
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader {
            message_i_d: 14,
            protocol_version: 2,
            ..
        }) => (
            Some(14),
            decode_cpm(
                input,
                Some(211),
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader {
            message_i_d: 14, ..
        }) => (
            Some(14),
            decode_cpm(
                input,
                Some(131),
                Headers::None,
                inputEncodingRules,
                outputEncodingRules,
            )?
            .its,
        ),
        Ok(ItsPduHeader { message_i_d, .. }) => {
            return Err(format!(
                "Unsupported ITS message type: Found message id {message_i_d}."
            ))
        }
        _ => return Err("Failed to detect message ID of ITS PDU header.".to_string()),
    };
    etsi_json.its = decoded;
    etsi_json.message_type = msg_ty;
    Ok(etsi_json)
}

fn decode_denm(
    denm: &[u8],
    mut version: Option<u32>,
    headers_present: Headers,
    input_encoding_rules: EncodingRules,
    output_encoding_rules: EncodingRules,
) -> Result<EtsiJson, String> {
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
) -> Result<EtsiJson, String> {
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
) -> Result<EtsiJson, String> {
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
) -> Result<EtsiJson, String> {
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
) -> Result<EtsiJson, String> {
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
) -> Result<EtsiJson, String> {
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
) -> Result<EtsiJson, String> {
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
) -> Result<EtsiJson, String> {
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

fn optionally_decode_headers(input: &[u8], headers: Headers) -> Result<(&[u8], EtsiJson), String> {
    match headers {
        Headers::None => Ok((input, EtsiJson::default())),
        Headers::GnBtp => decode_gn_and_btp(input),
        Headers::RadioTap802LlcGnBtp => remove_pcap_headers(input).and_then(decode_gn_and_btp),
    }
}

fn decode_gn_and_btp(input: &[u8]) -> Result<(&[u8], EtsiJson), String> {
    decode_geonetworking_header(input).and_then(|(remaining, gn_json, next_header)| {
        decode_transport_header(remaining, next_header).map(|(rem, tp)| {
            (
                rem,
                EtsiJson {
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
