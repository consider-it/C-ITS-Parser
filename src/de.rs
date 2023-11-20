#![allow(non_snake_case)]
use etsi_geonetworking::{
    Decodable, Decode, Encode, Header as GeoNetworkingHeader, NextAfterCommon,
};
use etsi_transports::{
    BasicTransportAHeader, BasicTransportBHeader, Decode as TransportDecode, IPv6Header,
};
use wasm_bindgen::prelude::*;

use crate::{map_err_to_string, EtsiJson, standards::is_1_3_1::ItsPduHeader};

macro_rules! btp {
    ($btp_ty:ty, $input:ident) => {
        <$btp_ty>::decode($input)
            .map_err(map_err_to_string)?
            .1
            .encode_to_json()
            .map_err(map_err_to_string)
            .map(|buf| (4, Some(buf)))
    };
}

#[wasm_bindgen(js_name = decode)]
/// Decodes an ITS message of undefined type.
/// Tries to parse the ITS PDU header to read the message ID that identifies the message type.
/// Set `includesHeaders` to `false` if the given binary message does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode(
    message: &[u8],
    includesHeaders: bool
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(message, includesHeaders)?;
    let message_type = rasn::uper::decode::<ItsPduHeader>(&message[bytes_read..]);
    etsi_json.its = match message_type {
        Ok(ItsPduHeader { message_i_d: 1, .. }) => decode_denm(&message[bytes_read..], None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 2, .. }) => decode_cam(&message[bytes_read..], None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 4, .. }) => decode_spatem(&message[bytes_read..], None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 5, .. }) => decode_mapem(&message[bytes_read..], None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 6, .. }) => decode_ivim(&message[bytes_read..], None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 9, .. }) => decode_srem(&message[bytes_read..], None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 10, .. }) => decode_ssem(&message[bytes_read..], None, false)?.its,
        Ok(ItsPduHeader { message_i_d: 14, .. }) => decode_cpm(&message[bytes_read..], None, false)?.its,
        Ok(ItsPduHeader { message_i_d, .. }) => return Err(format!(
            "Unsupported ITS message type: Found message id {message_i_d}."
        )),
        _ => return Err(format!(
                "Failed to detect message ID of ITS PDU header."
            ))
    };
    Ok(etsi_json)
}

#[wasm_bindgen(js_name = decodeDenm)]
/// Decodes a DENM message with the default decoding options.
/// The default options expect a message with headers and version 2.2.1
/// Throws string error on decoding errors.
pub fn decode_denm_default(denm: &[u8]) -> Result<EtsiJson, String> {
    decode_denm(denm, None, true)
}

#[wasm_bindgen(js_name = decodeDenmVersion)]
/// Decodes a DENM message with custom decoding options.
/// Currently, the library supports DENM versions v2.1.1 (211) and v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary denm does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_denm(
    denm: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(denm, includesHeaders)?;
    etsi_json.its = match version {
        Some(131) => Some(transcode_uper_to_jer::<crate::standards::denm_1_3_1::DENM>(
            &denm[bytes_read..],
        ))
        .transpose(),
        None | Some(211) => Some(transcode_uper_to_jer::<crate::standards::denm_2_1_1::DENM>(
            &denm[bytes_read..],
        ))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported DENM version: Supported DENM versions are 131 and 211."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen(js_name = decodeCam)]
/// Decodes a CAM message with the default decoding options.
/// The default options expect a message with headers and version 1.4.1
/// Throws string error on decoding errors.
pub fn decode_cam_default(cam: &[u8]) -> Result<EtsiJson, String> {
    decode_cam(cam, None, true)
}

#[wasm_bindgen(js_name = decodeCamVersion)]
/// Decodes a CAM message with custom decoding options.
/// Currently, the library supports CAM version v1.4.1 (141)
/// Set `includesHeaders` to `false` if the given binary CAM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_cam(
    cam: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(cam, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(141) => Some(transcode_uper_to_jer::<crate::standards::cam_1_4_1::CAM>(
            &cam[bytes_read..],
        ))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported DENM version: Supported CAM version is 141."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen(js_name = decodeMapem)]
/// Decodes a MAPEM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_mapem_default(mapem: &[u8]) -> Result<EtsiJson, String> {
    decode_mapem(mapem, None, true)
}

#[wasm_bindgen(js_name = decodeMapemVersion)]
/// Decodes a MAPEM message with custom decoding options.
/// Currently, the library supports MAPEM versions v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary MAPEM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_mapem(
    mapem: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(mapem, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::MAPEM>(
            &mapem[bytes_read..],
        ))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported MAPEM version: Supported MAPEM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen(js_name = decodeSpatem)]
/// Decodes a SPATEM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_spatem_default(spatem: &[u8]) -> Result<EtsiJson, String> {
    decode_spatem(spatem, None, true)
}

#[wasm_bindgen(js_name = decodeSpatemVersion)]
/// Decodes a SPATEM message with custom decoding options.
/// Currently, the library supports SPATEM versions v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary SPATEM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_spatem(
    spatem: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(spatem, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::SPATEM>(
            &spatem[bytes_read..],
        ))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported SPATEM version: Supported SPATEM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen(js_name = decodeIvim)]
/// Decodes a IVIM message with the default decoding options.
/// The default options expect a message with headers and version 2.2.1
/// Throws string error on decoding errors.
pub fn decode_ivim_default(ivim: &[u8]) -> Result<EtsiJson, String> {
    decode_ivim(ivim, None, true)
}

#[wasm_bindgen(js_name = decodeIvimVersion)]
/// Decodes a IVIM message with custom decoding options.
/// Currently, the library supports IVIM versions v2.2.1 (221)
/// Set `includesHeaders` to `false` if the given binary IVIM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_ivim(
    ivim: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(ivim, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(221) => Some(transcode_uper_to_jer::<crate::standards::ivim_2_2_1::IVIM>(
            &ivim[bytes_read..],
        ))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported IVIM version: Supported IVIM version is 221."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen(js_name = decodeSrem)]
/// Decodes a DENM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_srem_default(srem: &[u8]) -> Result<EtsiJson, String> {
    decode_srem(srem, None, true)
}

#[wasm_bindgen(js_name = decodeSremVersion)]
/// Decodes a DENM message with custom decoding options.
/// Currently, the library supports DENM versions v1.3.1 (211) and v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary denm does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_srem(
    srem: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(srem, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::SREM>(
            &srem[bytes_read..],
        ))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported SREM version: Supported SREM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen(js_name = decodeCpm)]
/// Decodes a CPM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_cpm_default(cpm: &[u8]) -> Result<EtsiJson, String> {
    decode_cpm(cpm, None, true)
}

#[wasm_bindgen(js_name = decodeCpmVersion)]
/// Decodes a CPM message with custom decoding options.
/// Currently, the library supports CPM versions v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary CPM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_cpm(
    cpm: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(cpm, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::CPM>(
            &cpm[bytes_read..],
        ))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported CPM version: Supported CPM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen(js_name = decodeSsem)]
/// Decodes a SSEM message with the default decoding options.
/// The default options expect a message with headers and version 1.3.1
/// Throws string error on decoding errors.
pub fn decode_ssem_default(ssem: &[u8]) -> Result<EtsiJson, String> {
    decode_ssem(ssem, None, true)
}

#[wasm_bindgen(js_name = decodeSsemVersion)]
/// Decodes a SSEM message with custom decoding options.
/// Currently, the library supports SSEM versions v1.3.1 (131)
/// Set `includesHeaders` to `false` if the given binary SSEM does not contain GeoNetworking or Transport headers.
/// Throws string error on decoding errors.
pub fn decode_ssem(
    ssem: &[u8],
    version: Option<u32>,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(ssem, includesHeaders)?;
    etsi_json.its = match version {
        None | Some(131) => Some(transcode_uper_to_jer::<crate::standards::is_1_3_1::SSEM>(
            &ssem[bytes_read..],
        ))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported SSEM version: Supported SSEM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

fn optionally_decode_headers(
    input: &[u8],
    decode_headers: bool,
) -> Result<(usize, EtsiJson), String> {
    let (gn_bytes_read, geonetworking, next_header) = decode_headers
        .then(|| decode_geonetworking_header(input))
        .transpose()?
        .unwrap_or((0, None, None));
    let after_gn = &input[gn_bytes_read..];
    let (tp_bytes_read, transport) = next_header
        .map(|next| decode_transport_header(after_gn, next))
        .transpose()?
        .unwrap_or((0, None));
    Ok((
        gn_bytes_read + tp_bytes_read,
        EtsiJson {
            geonetworking,
            transport,
            its: None,
        },
    ))
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
                Some(gn.encode_to_json().map_err(map_err_to_string)),
            )
        })
        .map_err(map_err_to_string)?;
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
        NextAfterCommon::BTPA => btp![BasicTransportAHeader, input],
        NextAfterCommon::BTPB => btp![BasicTransportBHeader, input],
        NextAfterCommon::IPv6 => {
            let (remaining, _) = IPv6Header::decode(input).map_err(map_err_to_string)?;
            Ok((input.len() - remaining.len(), None))
        }
    }
}

fn transcode_uper_to_jer<T: rasn::Decode + rasn::Encode>(input: &[u8]) -> Result<String, String> {
    rasn::jer::encode(&rasn::uper::decode::<T>(input).map_err(map_err_to_string)?)
        .map_err(map_err_to_string)
}
