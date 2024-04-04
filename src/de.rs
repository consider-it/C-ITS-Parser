#![allow(non_snake_case)]
use crate::transport::{
    BasicTransportAHeader, BasicTransportBHeader, decode::Decode as TransportDecode, IPv6Header,
};
use geonetworking::{Decode, Encode, NextAfterCommon, Packet};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{map_err_to_string, standards::is_1_3_1::{self, ItsPduHeader}, EtsiJson};

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
pub fn decode_to_json(message: &[u8], includesHeaders: bool) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(message, includesHeaders)?;
    let message_type = rasn::uper::decode::<ItsPduHeader>(input);
    etsi_json.its = match message_type {
        Ok(ItsPduHeader { message_i_d: 1, .. }) => decode_denm_to_json(input, None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 2, .. }) => decode_cam_to_json(input, None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 4, .. }) => decode_spatem_to_json(input, None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 5, .. }) => decode_mapem_to_json(input, None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 6, .. }) => decode_ivim_to_json(input, None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 9, .. }) => decode_srem_to_json(input, None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 10, .. }) => decode_ssem_to_json(input, None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 14, .. }) => decode_cpm_to_json(input, None, false)?.its,
        Ok(ItsPduHeader { message_i_d, .. }) => {
            return Err(format!(
                "Unsupported ITS message type: Found message id {message_i_d}."
            ))
        }
        _ => return Err("Failed to detect message ID of ITS PDU header.".to_string()),
    };
    Ok(etsi_json)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeDenm))]
/// Decodes a DENM message with the default decoding options.
/// The default options expect a message with headers and version 2.2.1
/// Throws string error on decoding errors.
pub fn decode_denm_default_to_json(denm: &[u8]) -> Result<EtsiJson, String> {
    decode_denm_to_json(denm, None, true)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeDenmVersion))]
/// Decodes a DENM message with custom decoding options.
/// Currently, the library supports DENM versions v2.1.1 (211) and v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary denm does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_denm_to_json(
    denm: &[u8],
    mut version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(denm, includesHeaders)?;
    if version.is_none() {
        version = match input.first() {
            Some(1) => Some(131),
            Some(2) => Some(211),
            _ => None
        };
    }
    etsi_json.its = match version {
        Some(131) => Some(transcode_uper_to_jer::<crate::standards::denm_1_3_1::DENM>(
            input,
        ))
        .transpose(),
        None | Some(211) => Some(transcode_uper_to_jer::<crate::standards::denm_2_1_1::d_e_n_m__p_d_u__description::DENM>(
            input,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported DENM version: Supported DENM versions are 131 and 211.".to_string())
        }
    }?;
    Ok(etsi_json)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeCam))]
/// Decodes a CAM message with the default decoding options.
/// The default options expect a message with headers and version 1.4.1
/// Throws string error on decoding errors.
pub fn decode_cam_default_to_json(cam: &[u8]) -> Result<EtsiJson, String> {
    decode_cam_to_json(cam, None, true)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeCamVersion))]
/// Decodes a CAM message with custom decoding options.
/// Currently, the library supports CAM version v1.4.1 (141)
/// Set `includesHeaders` to `false` if the given binary CAM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_cam_to_json(
    cam: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(cam, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(141) => Some(transcode_uper_to_jer::<crate::standards::cam_1_4_1::CAM>(
            input,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported DENM version: Supported CAM version is 141.".to_string())
        }
    }?;
    Ok(etsi_json)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeMapem))]
/// Decodes a MAPEM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_mapem_default_to_json(mapem: &[u8]) -> Result<EtsiJson, String> {
    decode_mapem_to_json(mapem, None, true)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeMapemVersion))]
/// Decodes a MAPEM message with custom decoding options.
/// Currently, the library supports MAPEM versions v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary MAPEM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_mapem_to_json(
    mapem: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(mapem, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::MAPEM>(
            input,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported MAPEM version: Supported MAPEM version is 131.".to_string())
        }
    }?;
    Ok(etsi_json)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeSpatem))]
/// Decodes a SPATEM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_spatem_default_to_json(spatem: &[u8]) -> Result<EtsiJson, String> {
    decode_spatem_to_json(spatem, None, true)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeSpatemVersion))]
/// Decodes a SPATEM message with custom decoding options.
/// Currently, the library supports SPATEM versions v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary SPATEM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_spatem_to_json(
    spatem: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(spatem, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::SPATEM>(
            input,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported SPATEM version: Supported SPATEM version is 131.".to_string())
        }
    }?;
    Ok(etsi_json)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeIvim))]
/// Decodes a IVIM message with the default decoding options.
/// The default options expect a message with headers and version 2.2.1
/// Throws string error on decoding errors.
pub fn decode_ivim_default_to_json(ivim: &[u8]) -> Result<EtsiJson, String> {
    decode_ivim_to_json(ivim, None, true)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeIvimVersion))]
/// Decodes a IVIM message with custom decoding options.
/// Currently, the library supports IVIM versions v1.3.1 (131) and v2.2.1 (221)
/// Set `includesHeaders` to `false` if the given binary IVIM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_ivim_to_json(
    ivim: &[u8],
    mut version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(ivim, includesHeaders)?;
    if version.is_none() {
        version = match input.first() {
            Some(1) => Some(131),
            Some(2) => Some(221),
            _ => None
        };
    }
    etsi_json.its = match version {
        Some(131) => Some(transcode_uper_to_jer::<is_1_3_1::IVIM>(input)).transpose(),
        None | Some(221) => Some(transcode_uper_to_jer::<crate::standards::ivim_2_2_1::IVIM>(
            input,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported IVIM version: Supported IVIM versions are 131 and 221.".to_string())
        }
    }?;
    Ok(etsi_json)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeSrem))]
/// Decodes a DENM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_srem_default_to_json(srem: &[u8]) -> Result<EtsiJson, String> {
    decode_srem_to_json(srem, None, true)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeSremVersion))]
/// Decodes a DENM message with custom decoding options.
/// Currently, the library supports DENM versions v1.3.1 (211) and v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary denm does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_srem_to_json(
    srem: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(srem, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::SREM>(
            input,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported SREM version: Supported SREM version is 131.".to_string())
        }
    }?;
    Ok(etsi_json)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeCpm))]
/// Decodes a CPM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_cpm_default_to_json(cpm: &[u8]) -> Result<EtsiJson, String> {
    decode_cpm_to_json(cpm, None, true)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeCpmVersion))]
/// Decodes a CPM message with custom decoding options.
/// Currently, the library supports CPM versions v1.3.1 (131) and v2.1.1 (211)
/// Set `includesHeaders` to `false` if the given binary CPM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_cpm_to_json(
    cpm: &[u8],
    mut version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(cpm, includesHeaders)?;
    if version.is_none() {
        version = match input.first() {
            Some(1) => Some(131),
            Some(2) => Some(211),
            _ => None
        };
    }
    etsi_json.its = match version {
        None | Some(211) => Some(transcode_uper_to_jer::<crate::standards::cpm_2_1_1::c_p_m__p_d_u__descriptions::CollectivePerceptionMessage>(
            input,
        ))
        .transpose(),
        Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::CPM>(
            input,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported CPM version: Supported CPM versions are 131 and 211.".to_string())
        }
    }?;
    Ok(etsi_json)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeSsem))]
/// Decodes a SSEM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_ssem_default_to_json(ssem: &[u8]) -> Result<EtsiJson, String> {
    decode_ssem_to_json(ssem, None, true)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = decodeSsemVersion))]
/// Decodes a SSEM message with custom decoding options.
/// Currently, the library supports SSEM versions v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary SSEM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_ssem_to_json(
    ssem: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (input, mut etsi_json) = optionally_decode_headers(ssem, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::SSEM>(
            input,
        ))
        .transpose(),
        _ => {
            return Err("Unsupported SSEM version: Supported SSEM version is 131.".to_string())
        }
    }?;
    Ok(etsi_json)
}

fn optionally_decode_headers(
    input: &[u8],
    decode_headers: bool,
) -> Result<(&[u8], EtsiJson), String> {
    decode_headers
        .then(|| {
            decode_geonetworking_header(input).and_then(|(remaining, gn_json, next_header)| {
                decode_transport_header(remaining, next_header).map(|(rem, tp)| {
                    (
                        rem,
                        EtsiJson {
                            geonetworking: Some(gn_json),
                            transport: Some(tp),
                            its: None,
                        },
                    )
                })
            })
        })
        .unwrap_or(Ok((
            input,
            EtsiJson {
                geonetworking: None,
                transport: None,
                its: None,
            },
        )))
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
        NextAfterCommon::Any => Err("Currently, only BTP and IPv6 Headers can be decoded!".to_string()),
        NextAfterCommon::BTPA => btp![BasicTransportAHeader, input],
        NextAfterCommon::BTPB => btp![BasicTransportBHeader, input],
        NextAfterCommon::IPv6 => {
            let (remaining, ipv6) = IPv6Header::decode(input).map_err(map_err_to_string)?;
            Ok((remaining, to_ipv6_debug(ipv6)))
        }
    }
}

fn transcode_uper_to_jer<T: rasn::Decode + rasn::Encode>(input: &[u8]) -> Result<String, String> {
    rasn::jer::encode(&rasn::uper::decode::<T>(input).map_err(map_err_to_string)?)
        .map_err(map_err_to_string)
}

fn to_ipv6_debug(ipv6: IPv6Header) -> String {
    format!(r#"{{"ipv6Debug":"{ipv6:?}"}}"#)
}
