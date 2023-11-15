extern crate alloc;

use alloc::{format, string::String};
use etsi_geonetworking::{
    Decodable, Decode, Encode, Header as GeoNetworkingHeader, NextAfterCommon,
};
use etsi_transports::{
    BasicTransportAHeader, BasicTransportBHeader, Decode as TransportDecode, IPv6Header,
};
use wasm_bindgen::prelude::*;

use crate::EtsiJson;

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

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn decodeDenmToJson(
    denm: &[u8],
    version: u32,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(denm, includesHeaders)?;
    etsi_json.its = match version {
        131 => Some(transcode_uper_to_jer_bytes::<
            crate::standards::denm_1_3_1::DENM,
        >(&denm[bytes_read..]))
        .transpose(),
        211 => Some(transcode_uper_to_jer_bytes::<
            crate::standards::denm_2_1_1::DENM,
        >(&denm[bytes_read..]))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported DENM version: Supported DENM versions are 131 and 211."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn decodeCamToJson(
    cam: &[u8],
    version: u32,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(cam, includesHeaders)?;
    etsi_json.its = match version {
        141 => Some(transcode_uper_to_jer_bytes::<
            crate::standards::cam_1_4_1::CAM,
        >(&cam[bytes_read..]))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported DENM version: Supported CAM version is 141."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn decodeMapemToJson(
    mapem: &[u8],
    version: u32,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(mapem, includesHeaders)?;
    etsi_json.its = match version {
        131 => Some(transcode_uper_to_jer_bytes::<
            crate::standards::is_1_3_1::MAPEM,
        >(&mapem[bytes_read..]))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported MAPEM version: Supported MAPEM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn decodeSpatemToJson(
    spatem: &[u8],
    version: u32,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(spatem, includesHeaders)?;
    etsi_json.its = match version {
        131 => Some(transcode_uper_to_jer_bytes::<
            crate::standards::is_1_3_1::SPATEM,
        >(&spatem[bytes_read..]))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported SPATEM version: Supported SPATEM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn decodeIvimToJson(
    ivim: &[u8],
    version: u32,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(ivim, includesHeaders)?;
    etsi_json.its = match version {
        131 => Some(transcode_uper_to_jer_bytes::<
            crate::standards::is_1_3_1::IVIM,
        >(&ivim[bytes_read..]))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported IVIM version: Supported IVIM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn decodeSremToJson(
    srem: &[u8],
    version: u32,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(srem, includesHeaders)?;
    etsi_json.its = match version {
        131 => Some(transcode_uper_to_jer_bytes::<
            crate::standards::is_1_3_1::SREM,
        >(&srem[bytes_read..]))
        .transpose(),
        _ => {
            return Err(format!(
                "Unsupported SREM version: Supported SREM version is 131."
            ))
        }
    }?;
    Ok(etsi_json)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn decodeSsemToJson(
    ssem: &[u8],
    version: u32,
    includesHeaders: bool,
) -> Result<EtsiJson, String> {
    let (bytes_read, mut etsi_json) = optionally_decode_headers(ssem, includesHeaders)?;
    etsi_json.its = match version {
        131 => Some(transcode_uper_to_jer_bytes::<
            crate::standards::is_1_3_1::SSEM,
        >(&ssem[bytes_read..]))
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

fn map_err_to_string<E: core::fmt::Debug>(error: E) -> String {
    format!("{error:?}")
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

fn transcode_uper_to_jer_bytes<T: rasn::Decode + rasn::Encode>(
    input: &[u8],
) -> Result<String, String> {
    rasn::jer::encode(&rasn::uper::decode::<T>(input).map_err(map_err_to_string)?)
        .map_err(map_err_to_string)
}
