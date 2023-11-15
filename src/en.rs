use crate::{map_err_to_string, EtsiJson};
use etsi_geonetworking::{Decode, Encode, Encoder, Header as GeoNetworkingHeader};
use etsi_transports::{BasicTransportAHeader, BasicTransportBHeader, Encode as TpEncode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = encodeDenm)]
/// Encodes a DENM message into binary UPER with optional headers 
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, denms of the following versions are supported: v2.1.1 (211) and v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_denm(denm: &EtsiJson, version: u32) -> Result<js_sys::Uint8Array, String> {
    let mut encoded = vec![];
    optionally_encode_headers(&denm.geonetworking, &denm.transport, &mut encoded)?;
    match (&denm.its, version) {
        (None, 131) | (None, 211) => return Err(format!("No DENM JSON provided.")),
        (Some(denm_json), 131) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::denm_1_3_1::DENM,
            >(denm_json)?);
        }
        (Some(denm_json), 211) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::denm_2_1_1::DENM,
            >(denm_json)?);
        }
        _ => {
            return Err(format!(
                "Unsupported DENM version: Supported DENM versions are 131 and 211."
            ))
        }
    };
    Ok(js_sys::Uint8Array::from(encoded.as_slice()))
}

#[wasm_bindgen(js_name = encodeCam)]
/// Encodes a CAM message into binary UPER with optional headers 
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, cams of the following versions are supported: v1.4.1 (141)
/// Throws string error on encoding error
pub fn encode_cam(cam: &EtsiJson, version: u32) -> Result<js_sys::Uint8Array, String> {
    let mut encoded = vec![];
    optionally_encode_headers(&cam.geonetworking, &cam.transport, &mut encoded)?;
    match (&cam.its, version) {
        (None, 141) => return Err(format!("No CAM JSON provided.")),
        (Some(json), 141) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::cam_1_4_1::CAM,
            >(json)?);
        }
        _ => {
            return Err(format!(
                "Unsupported CAM version: Supported CAM version is 141."
            ))
        }
    };
    Ok(js_sys::Uint8Array::from(encoded.as_slice()))
}

#[wasm_bindgen(js_name = encodeMapem)]
/// Encodes a MAPEM message into binary UPER with optional headers 
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, mapems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_mapem(mapem: &EtsiJson, version: u32) -> Result<js_sys::Uint8Array, String> {
    let mut encoded = vec![];
    optionally_encode_headers(&mapem.geonetworking, &mapem.transport, &mut encoded)?;
    match (&mapem.its, version) {
        (None, 131) => return Err(format!("No MAPEM JSON provided.")),
        (Some(json), 131) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::MAPEM,
            >(json)?);
        }
        _ => {
            return Err(format!(
                "Unsupported MAPEM version: Supported MAPEM version is 131."
            ))
        }
    };
    Ok(js_sys::Uint8Array::from(encoded.as_slice()))
}

#[wasm_bindgen(js_name = encodeSpatem)]
/// Encodes a SPATEM message into binary UPER with optional headers 
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, spatems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_spatem(spatem: &EtsiJson, version: u32) -> Result<js_sys::Uint8Array, String> {
    let mut encoded = vec![];
    optionally_encode_headers(&spatem.geonetworking, &spatem.transport, &mut encoded)?;
    match (&spatem.its, version) {
        (None, 131) => return Err(format!("No SPATEM JSON provided.")),
        (Some(json), 131) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::SPATEM,
            >(json)?);
        }
        _ => {
            return Err(format!(
                "Unsupported SPATEM version: Supported SPATEM version is 131."
            ))
        }
    };
    Ok(js_sys::Uint8Array::from(encoded.as_slice()))
}

#[wasm_bindgen(js_name = encodeIvim)]
/// Encodes a IVIM message into binary UPER with optional headers 
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, ivims of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_ivim(ivim: &EtsiJson, version: u32) -> Result<js_sys::Uint8Array, String> {
    let mut encoded = vec![];
    optionally_encode_headers(&ivim.geonetworking, &ivim.transport, &mut encoded)?;
    match (&ivim.its, version) {
        (None, 131) => return Err(format!("No IVIM JSON provided.")),
        (Some(json), 131) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::IVIM,
            >(json)?);
        }
        _ => {
            return Err(format!(
                "Unsupported IVIM version: Supported IVIM version is 131."
            ))
        }
    };
    Ok(js_sys::Uint8Array::from(encoded.as_slice()))
}

#[wasm_bindgen(js_name = encodeSrem)]
/// Encodes a SREM message into binary UPER with optional headers 
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, srems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_srem(srem: &EtsiJson, version: u32) -> Result<js_sys::Uint8Array, String> {
    let mut encoded = vec![];
    optionally_encode_headers(&srem.geonetworking, &srem.transport, &mut encoded)?;
    match (&srem.its, version) {
        (None, 131) => return Err(format!("No SREM JSON provided.")),
        (Some(json), 131) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::SREM,
            >(json)?);
        }
        _ => {
            return Err(format!(
                "Unsupported SREM version: Supported SREM version is 131."
            ))
        }
    };
    Ok(js_sys::Uint8Array::from(encoded.as_slice()))
}

#[wasm_bindgen(js_name = encodeCpm)]
/// Encodes a CPM message into binary UPER with optional headers 
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, cpms of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_cpm(cpm: &EtsiJson, version: u32) -> Result<js_sys::Uint8Array, String> {
    let mut encoded = vec![];
    optionally_encode_headers(&cpm.geonetworking, &cpm.transport, &mut encoded)?;
    match (&cpm.its, version) {
        (None, 131) => return Err(format!("No CPM JSON provided.")),
        (Some(json), 131) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::CPM,
            >(json)?);
        }
        _ => {
            return Err(format!(
                "Unsupported CPM version: Supported CPM version is 131."
            ))
        }
    };
    Ok(js_sys::Uint8Array::from(encoded.as_slice()))
}

#[wasm_bindgen(js_name = encodeSsem)]
/// Encodes a SSEM message into binary UPER with optional headers 
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, ssems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_ssem(ssem: &EtsiJson, version: u32) -> Result<js_sys::Uint8Array, String> {
    let mut encoded = vec![];
    optionally_encode_headers(&ssem.geonetworking, &ssem.transport, &mut encoded)?;
    match (&ssem.its, version) {
        (None, 131) => return Err(format!("No SSEM JSON provided.")),
        (Some(json), 131) => {
            encoded.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::SSEM,
            >(json)?);
        }
        _ => {
            return Err(format!(
                "Unsupported SSEM version: Supported SSEM version is 131."
            ))
        }
    };
    Ok(js_sys::Uint8Array::from(encoded.as_slice()))
}

fn optionally_encode_headers(
    gn_json: &Option<String>,
    tp_json: &Option<String>,
    buffer: &mut Vec<u8>,
) -> Result<(), String> {
    match (gn_json, tp_json) {
        (Some(_), None) | (None, Some(_)) => {
            return Err(format!(
            "Expecting either both or neither GeoNetworking and Transport headers to be present!"
        ))
        }
        (Some(gn), Some(tp)) => {
            let mut encoder = Encoder::new();
            let geonetworking =
                GeoNetworkingHeader::decode_from_json(gn).map_err(map_err_to_string)?;
            geonetworking
                .encode(&mut encoder)
                .map_err(map_err_to_string)?;
            buffer.append(&mut encoder.into());
            match geonetworking.common.next_header {
                etsi_geonetworking::NextAfterCommon::BTPA => {
                    buffer.append(
                        &mut BasicTransportAHeader::decode_from_json(tp)
                            .map_err(map_err_to_string)?
                            .encode()
                            .map_err(map_err_to_string)?,
                    );
                }
                etsi_geonetworking::NextAfterCommon::BTPB => buffer.append(
                    &mut BasicTransportBHeader::decode_from_json(tp)
                        .map_err(map_err_to_string)?
                        .encode()
                        .map_err(map_err_to_string)?,
                ),
                h => {
                    return Err(format!(
                        "Currently only BTP-A and BTP-B headers can be encoded: Encountered {h:?}"
                    ))
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn transcode_jer_to_uper<T: rasn::Decode + rasn::Encode>(
    input: &String,
) -> Result<Vec<u8>, String> {
    rasn::uper::encode(&rasn::jer::decode::<T>(input).map_err(map_err_to_string)?)
        .map_err(map_err_to_string)
}
