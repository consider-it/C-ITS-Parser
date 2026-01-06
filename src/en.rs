#[cfg(any(
    all(target_arch = "wasm32", feature = "json"),
    not(target_arch = "wasm32")
))]
use crate::map_err_to_string;
#[cfg(not(target_arch = "wasm32"))]
use crate::ItsMessage;
#[cfg(all(target_arch = "wasm32", feature = "json"))]
use crate::JsonItsMessage;
#[cfg(not(target_arch = "wasm32"))]
use crate::{EncodingRules, Packet};
#[cfg(any(
    all(target_arch = "wasm32", feature = "json"),
    not(target_arch = "wasm32")
))]
use geonetworking::{Encode, ExtendedHeader, HeaderType, UnsecuredHeader};

#[cfg(all(target_arch = "wasm32", feature = "json"))]
use crate::transport::encode::Encode as TpEncode;
#[cfg(all(target_arch = "wasm32", feature = "json"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "json"))]
pub type Encoded = js_sys::Uint8Array;
#[cfg(not(target_arch = "wasm32"))]
pub type Encoded = Vec<u8>;

#[cfg(not(target_arch = "wasm32"))]
impl ItsMessage<'_> {
    #[allow(clippy::too_many_lines)]
    /// Encodes an ITS message with optional headers.
    ///
    /// Supports XER, JER, and UPER encoding rules.
    /// XER and JER values are returned as UTF8 buffers.
    ///
    /// # Errors
    ///
    /// Gives a human-readable error description when ASN.1 parsing failed or an
    /// unexpected set of headers was found.
    pub fn encode(self, encoding_rules: EncodingRules) -> Result<Encoded, String> {
        let (geo, tp, mut etsi_uper) = match self {
            ItsMessage::DenmV1 {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::DenmV2 {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::Cam {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::Spatem {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::Mapem {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::IvimV1 {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::IvimV2 {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::Srem {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::Ssem {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::CpmV1 {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
            ItsMessage::CpmV2 {
                geonetworking,
                transport,
                etsi,
            } => encoding_rules
                .codec()
                .encode_to_binary(&etsi)
                .map(|enc| (geonetworking, transport, enc)),
        }
        .map_err(map_err_to_string)?;

        match (tp, geo) {
            (None, None) => Ok(etsi_uper),
            (
                Some(tp),
                Some(Packet::Unsecured {
                    basic,
                    common,
                    extended,
                    ..
                }),
            ) => {
                let mut encoded = tp.encode()?;
                encoded.append(&mut etsi_uper);
                fill_gn_and_encode(UnsecuredHeader { basic, common, extended }, &encoded)
            }
            _ => Err(
                "Expecting either both or neither GeoNetworking and Transport headers to be present!"
                    .to_string(),
            ),
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(js_name = encodeDenm)]
/// Encodes a DENM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, denms of the following versions are supported: v2.1.1 (211) and v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_denm(denm: &JsonItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&denm.its, version) {
        (None, 131) | (None, 211) => return Err("No DENM JSON provided.".to_string()),
        (Some(denm_json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::denm_1_3_1::denm_pdu_descriptions::DENM,
            >(denm_json)?);
        }
        (Some(denm_json), 211) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::denm_2_1_1::denm_pdu_description::DENM,
            >(denm_json)?);
        }
        _ => {
            return Err(
                "Unsupported DENM version: Supported DENM versions are 131 and 211.".to_string(),
            );
        }
    };
    let encoded = optionally_encode_headers(&denm.geonetworking, &denm.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(js_name = encodeCam)]
/// Encodes a CAM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, cams of the following versions are supported: v1.4.1 (141)
/// Throws string error on encoding error
pub fn encode_cam(cam: &JsonItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&cam.its, version) {
        (None, 141) => return Err("No CAM JSON provided.".to_string()),
        (Some(json), 141) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::cam_1_4_1::cam_pdu_descriptions::CAM,
            >(json)?);
        }
        _ => return Err("Unsupported CAM version: Supported CAM version is 141.".to_string()),
    };
    let encoded = optionally_encode_headers(&cam.geonetworking, &cam.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(js_name = encodeMapem)]
/// Encodes a MAPEM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, mapems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_mapem(mapem: &JsonItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&mapem.its, version) {
        (None, 131) => return Err("No MAPEM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::etsi_schema::MAPEM,
            >(json)?);
        }
        _ => return Err("Unsupported MAPEM version: Supported MAPEM version is 131.".to_string()),
    };
    let encoded = optionally_encode_headers(&mapem.geonetworking, &mapem.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(js_name = encodeSpatem)]
/// Encodes a SPATEM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, spatems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_spatem(spatem: &JsonItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&spatem.its, version) {
        (None, 131) => return Err("No SPATEM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::etsi_schema::SPATEM,
            >(json)?);
        }
        _ => {
            return Err("Unsupported SPATEM version: Supported SPATEM version is 131.".to_string());
        }
    };
    let encoded = optionally_encode_headers(&spatem.geonetworking, &spatem.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(js_name = encodeIvim)]
/// Encodes a IVIM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, ivims of the following versions are supported: v2.2.1 (221)
/// Throws string error on encoding error
pub fn encode_ivim(ivim: &JsonItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&ivim.its, version) {
        (None, 221) => return Err("No IVIM JSON provided.".to_string()),
        (Some(json), 221) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::ivim_2_2_1::ivim_pdu_descriptions::IVIM,
            >(json)?);
        }
        _ => return Err("Unsupported IVIM version: Supported IVIM version is 221.".to_string()),
    };
    let encoded = optionally_encode_headers(&ivim.geonetworking, &ivim.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(js_name = encodeSrem)]
/// Encodes a SREM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, srems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_srem(srem: &JsonItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&srem.its, version) {
        (None, 131) => return Err("No SREM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::etsi_schema::SREM,
            >(json)?);
        }
        _ => return Err("Unsupported SREM version: Supported SREM version is 131.".to_string()),
    };
    let encoded = optionally_encode_headers(&srem.geonetworking, &srem.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(js_name = encodeCpm)]
/// Encodes a CPM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, cpms of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_cpm(cpm: &JsonItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&cpm.its, version) {
        (None, 131) => return Err("No CPM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::etsi_schema::CPM,
            >(json)?);
        }
        _ => return Err("Unsupported CPM version: Supported CPM version is 131.".to_string()),
    };
    let encoded = optionally_encode_headers(&cpm.geonetworking, &cpm.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
#[wasm_bindgen(js_name = encodeSsem)]
/// Encodes a SSEM message into binary UPER with optional headers
/// The encoder expects either both (GeoNetworking and Transport) headers or none
/// Currently, ssems of the following versions are supported: v1.3.1 (131)
/// Throws string error on encoding error
pub fn encode_ssem(ssem: &JsonItsMessage, version: u32) -> Result<Encoded, String> {
    let mut payload = vec![];
    match (&ssem.its, version) {
        (None, 131) => return Err("No SSEM JSON provided.".to_string()),
        (Some(json), 131) => {
            payload.append(&mut transcode_jer_to_uper::<
                crate::standards::is_1_3_1::etsi_schema::SSEM,
            >(json)?);
        }
        _ => return Err("Unsupported SSEM version: Supported SSEM version is 131.".to_string()),
    };
    let encoded = optionally_encode_headers(&ssem.geonetworking, &ssem.transport, payload)?;
    Ok(Encoded::from(encoded.as_slice()))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
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
            let geonetworking = UnsecuredHeader::from_json(gn).map_err(map_err_to_string)?;
            let mut transport = match geonetworking.common.next_header {
                geonetworking::NextAfterCommon::BTPA => {
                    crate::transport::BasicTransportAHeader::decode_from_json(tp)
                        .map_err(map_err_to_string)?
                        .encode()
                        .map_err(map_err_to_string)?
                }
                geonetworking::NextAfterCommon::BTPB => {
                    crate::transport::BasicTransportBHeader::decode_from_json(tp)
                        .map_err(map_err_to_string)?
                        .encode()
                        .map_err(map_err_to_string)?
                }
                h => {
                    return Err(format!(
                        "Currently only BTP-A and BTP-B headers can be encoded: Encountered {h:?}"
                    ));
                }
            };
            transport.append(&mut its);
            fill_gn_and_encode(geonetworking, &transport)
        }
        _ => Ok(its),
    }
}

#[cfg(any(
    all(target_arch = "wasm32", feature = "json"),
    not(target_arch = "wasm32")
))]
fn fill_gn_and_encode(
    mut geonetworking: UnsecuredHeader,
    payload: &[u8],
) -> Result<Vec<u8>, String> {
    #[allow(clippy::cast_possible_truncation)]
    let gn_payload_length = payload.len() as u16;

    geonetworking.common.payload_length = gn_payload_length;
    geonetworking.common.header_type_and_subtype = match geonetworking.extended {
        Some(ExtendedHeader::Beacon(_)) => HeaderType::Beacon,
        Some(ExtendedHeader::GAC(_)) => HeaderType::GeoAnycast(geonetworking::AreaType::Circular),
        Some(ExtendedHeader::GBC(_)) => HeaderType::GeoBroadcast(geonetworking::AreaType::Circular),
        Some(ExtendedHeader::GUC(_)) => HeaderType::GeoUnicast,
        Some(ExtendedHeader::TSB(_)) => {
            HeaderType::TopologicallyScopedBroadcast(geonetworking::BroadcastType::MultiHop)
        }
        Some(ExtendedHeader::SHB(_)) => {
            HeaderType::TopologicallyScopedBroadcast(geonetworking::BroadcastType::SingleHop)
        }
        Some(ExtendedHeader::LSRequest(_)) => {
            HeaderType::LocationService(geonetworking::LocationServiceType::Request)
        }
        Some(ExtendedHeader::LSReply(_)) => {
            HeaderType::LocationService(geonetworking::LocationServiceType::Reply)
        }
        None => HeaderType::Any,
    };
    geonetworking
        .with_payload(payload)
        .map_err(map_err_to_string)?
        .encode_to_vec()
        .map_err(map_err_to_string)
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
fn transcode_jer_to_uper<T: rasn::Decode + rasn::Encode>(
    input: &String,
) -> Result<Vec<u8>, String> {
    rasn::uper::encode(&rasn::jer::decode::<T>(input).map_err(map_err_to_string)?)
        .map_err(map_err_to_string)
}
