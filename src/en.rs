use crate::transport::{encode::Encode as TpEncode, BasicTransportAHeader, BasicTransportBHeader};
use crate::{map_err_to_string, ItsMessage};
use geonetworking::{Encode, ExtendedHeader, HeaderType, UnsecuredHeader};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub type Encoded = js_sys::Uint8Array;
#[cfg(not(target_arch = "wasm32"))]
pub type Encoded = Vec<u8>;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodeDenm))]
/// Encodes a DENM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, denms of the following versions are supported: v2.1.1 (211) and v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_denm(denm: &ItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&denm.its, version) {
        (None, 131) | (None, 211) => return Err("No DENM JSON provided.".to_string()),
        (Some(denm_json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::denm_1_3_1::DENM,
            >(denm_json)?);
        }
        (Some(denm_json), 211) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::denm_2_1_1::d_e_n_m__p_d_u__description::DENM,
            >(denm_json)?);
        }
        _ => {
            return Err(
                "Unsupported DENM version: Supported DENM versions are 131 and 211.".to_string(),
            )
        }
    };
    let encoded = optionally_encode_headers(&denm.geonetworking, &denm.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodeCam))]
/// Encodes a CAM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, cams of the following versions are supported: v1.4.1 (141)
/// Throws string error on encoding error
pub fn encode_cam(cam: &ItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&cam.its, version) {
        (None, 141) => return Err("No CAM JSON provided.".to_string()),
        (Some(json), 141) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::cam_1_4_1::CAM,
            >(json)?);
        }
        _ => return Err("Unsupported CAM version: Supported CAM version is 141.".to_string()),
    };
    let encoded = optionally_encode_headers(&cam.geonetworking, &cam.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodeMapem))]
/// Encodes a MAPEM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, mapems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_mapem(mapem: &ItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&mapem.its, version) {
        (None, 131) => return Err("No MAPEM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::MAPEM,
            >(json)?);
        }
        _ => return Err("Unsupported MAPEM version: Supported MAPEM version is 131.".to_string()),
    };
    let encoded = optionally_encode_headers(&mapem.geonetworking, &mapem.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodeSpatem))]
/// Encodes a SPATEM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, spatems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_spatem(spatem: &ItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&spatem.its, version) {
        (None, 131) => return Err("No SPATEM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::SPATEM,
            >(json)?);
        }
        _ => {
            return Err("Unsupported SPATEM version: Supported SPATEM version is 131.".to_string())
        }
    };
    let encoded = optionally_encode_headers(&spatem.geonetworking, &spatem.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodeIvim))]
/// Encodes a IVIM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, ivims of the following versions are supported: v2.2.1 (221)
/// Throws string error on encoding error
pub fn encode_ivim(ivim: &ItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&ivim.its, version) {
        (None, 221) => return Err("No IVIM JSON provided.".to_string()),
        (Some(json), 221) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::ivim_2_2_1::IVIM,
            >(json)?);
        }
        _ => return Err("Unsupported IVIM version: Supported IVIM version is 221.".to_string()),
    };
    let encoded = optionally_encode_headers(&ivim.geonetworking, &ivim.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodeSrem))]
/// Encodes a SREM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, srems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_srem(srem: &ItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&srem.its, version) {
        (None, 131) => return Err("No SREM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::SREM,
            >(json)?);
        }
        _ => return Err("Unsupported SREM version: Supported SREM version is 131.".to_string()),
    };
    let encoded = optionally_encode_headers(&srem.geonetworking, &srem.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodeCpm))]
/// Encodes a CPM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, cpms of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_cpm(cpm: &ItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&cpm.its, version) {
        (None, 131) => return Err("No CPM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<crate::standards::is_1_3_1::CPM>(json)?);
        }
        _ => return Err("Unsupported CPM version: Supported CPM version is 131.".to_string()),
    };
    let encoded = optionally_encode_headers(&cpm.geonetworking, &cpm.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodeSsem))]
/// Encodes a SSEM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, ssems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_ssem(ssem: &ItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&ssem.its, version) {
        (None, 131) => return Err("No SSEM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::SSEM,
            >(json)?);
        }
        _ => return Err("Unsupported SSEM version: Supported SSEM version is 131.".to_string()),
    };
    let encoded = optionally_encode_headers(&ssem.geonetworking, &ssem.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

fn optionally_encode_headers(
    gn_json: &Option<String>,
    tp_json: &Option<String>,
    mut its: Vec<u8>,
) -> Result<Vec<u8>, String> {
    match (gn_json, tp_json) {
        (Some(_), None) | (None, Some(_)) => Err(
            "Expecting either both or neither GeoNetworking and Transport headers to be present!"
                .to_string(),
        ),
        (Some(gn), Some(tp)) => {
            let mut geonetworking = UnsecuredHeader::from_json(gn).map_err(map_err_to_string)?;
            let mut transport = match geonetworking.common.next_header {
                geonetworking::NextAfterCommon::BTPA => BasicTransportAHeader::decode_from_json(tp)
                    .map_err(map_err_to_string)?
                    .encode()
                    .map_err(map_err_to_string)?,
                geonetworking::NextAfterCommon::BTPB => BasicTransportBHeader::decode_from_json(tp)
                    .map_err(map_err_to_string)?
                    .encode()
                    .map_err(map_err_to_string)?,
                h => {
                    return Err(format!(
                        "Currently only BTP-A and BTP-B headers can be encoded: Encountered {h:?}"
                    ))
                }
            };
            transport.append(&mut its);
            geonetworking.common.payload_length = transport.len() as u16;
            geonetworking.common.header_type_and_subtype = match geonetworking.extended {
                Some(ExtendedHeader::Beacon(_)) => HeaderType::Beacon,
                Some(ExtendedHeader::GAC(_)) => {
                    HeaderType::GeoAnycast(geonetworking::AreaType::Circular)
                }
                Some(ExtendedHeader::GBC(_)) => {
                    HeaderType::GeoBroadcast(geonetworking::AreaType::Circular)
                }
                Some(ExtendedHeader::GUC(_)) => HeaderType::GeoUnicast,
                Some(ExtendedHeader::TSB(_)) => {
                    HeaderType::TopologicallyScopedBroadcast(geonetworking::BroadcastType::MultiHop)
                }
                Some(ExtendedHeader::SHB(_)) => HeaderType::TopologicallyScopedBroadcast(
                    geonetworking::BroadcastType::SingleHop,
                ),
                Some(ExtendedHeader::LSRequest(_)) => {
                    HeaderType::LocationService(geonetworking::LocationServiceType::Request)
                }
                Some(ExtendedHeader::LSReply(_)) => {
                    HeaderType::LocationService(geonetworking::LocationServiceType::Reply)
                }
                None => HeaderType::Any,
            };
            geonetworking
                .with_payload(&transport)
                .map_err(map_err_to_string)?
                .encode_to_vec()
                .map_err(map_err_to_string)
        }
        _ => Ok(its),
    }
}

fn transcode_jer_to_uper<T: rasn::Decode + rasn::Encode>(
    input: &String,
) -> Result<Vec<u8>, String> {
    rasn::uper::encode(&rasn::jer::decode::<T>(input).map_err(map_err_to_string)?)
        .map_err(map_err_to_string)
}
