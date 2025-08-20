use etsi_web::de::decode;
use etsi_web::standards::cam_1_4_1::cam_pdu_descriptions::{
    BasicContainer, BasicVehicleContainerHighFrequency, CAM, CamParameters, CoopAwareness,
    GenerationDeltaTime, HighFrequencyContainer, LowFrequencyContainer,
};
use etsi_web::standards::cdd_1_3_1_1::*;
use etsi_web::standards::is_1_3_1::etsi_schema::{
    ApproachID, DSecond, IntersectionID, ItsPduHeader, MinuteOfTheYear, MsgCount, RequestID,
    RequesterDescription, RequesterType, RoadRegulatorID, SREM, SignalRequest, SignalRequestList,
    SignalRequestMessage, SignalRequestPackage, Srm, VehicleID,
};
use etsi_web::transport::BasicTransportBHeader;
use etsi_web::{EncodingRules, Headers, ItsMessage};
#[cfg(target_arch = "wasm32")]
use etsi_web::{
    en::{
        encode_cam, encode_cpm, encode_denm, encode_ivim, encode_mapem, encode_spatem, encode_srem,
    },
    standards::{
        cam_1_4_1::CAM,
        denm_2_1_1::d_e_n_m__p_d_u__description::DENM,
        is_1_3_1::{CPM, IVIM, MAPEM, SPATEM},
    },
};
use geonetworking::{BasicHeader, Bits, CommonHeader, Encode, Lifetime, TrafficClass};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

const DENM: &[u8] = &[
    0x02, 0x01, 0xe0, 0xfd, 0x1d, 0x37, 0xe7, 0x46, 0x5a, 0xa8, 0xbc, 0x80, 0x06, 0x91, 0x0d, 0x64,
    0xc9, 0x04, 0x04, 0x43, 0x59, 0x32, 0x45, 0x9d, 0x59, 0x01, 0x92, 0x07, 0x13, 0x5f, 0x21, 0x42,
    0xbe, 0x2b, 0xe0, 0x00, 0x18, 0x6a, 0x09, 0x88, 0x00, 0x50, 0x14, 0x40, 0x18, 0x03, 0x00, 0x14,
    0xfb, 0x84, 0x3f, 0x0a, 0x2f, 0xdd, 0x6b, 0xfb, 0x00, 0xc5, 0x0a, 0x06, 0x02, 0x2d, 0x29, 0x7f,
    0xf5, 0x9f, 0xff, 0xe6, 0x06, 0x80, 0x5f, 0x9c, 0x00, 0xff, 0x00, 0x0d, 0x31, 0x24, 0x04, 0x9b,
    0xe0, 0x06, 0x48, 0x02, 0xad, 0x89, 0x20, 0x13, 0x87, 0x00, 0x1b, 0xc0, 0x39, 0x4c, 0x32, 0x81,
    0x12, 0xf8, 0x01, 0xa5, 0xfe, 0xd5, 0x61, 0x44, 0x09, 0x5f, 0xc0, 0x0d, 0x2f, 0xfc, 0x93, 0x11,
    0x00, 0x15, 0xde, 0x00, 0x57, 0x7f, 0x46, 0x58, 0x06, 0x02, 0xc8, 0x6f, 0xfd, 0x1b, 0xfc, 0xce,
    0xc7, 0xd8, 0x0b, 0xb7, 0x7f, 0xd5, 0x9f, 0xcd, 0xb7, 0x0e, 0x42, 0x89, 0xfb, 0xff, 0x7b, 0x00,
    0x8f, 0x33, 0xcc, 0x05, 0x77, 0xe0, 0x04, 0x18, 0x01, 0x0d, 0x86, 0xf0, 0x17, 0x6f, 0x00, 0x19,
    0xbf, 0xee, 0xec, 0xa8, 0x03, 0xa3, 0x37, 0xfc, 0x66, 0x04, 0x1e, 0x65, 0x68, 0x10, 0xfd, 0xc0,
    0x02, 0xf0, 0x02, 0x13, 0x17, 0x40, 0x6d, 0x5e, 0x00, 0x4a, 0x7f, 0xf1, 0xd8, 0xbf, 0x01, 0x51,
    0x70, 0x03, 0x7b, 0xf8, 0x62, 0xcd, 0xf0, 0x13, 0xd7, 0x80, 0x07, 0xa0, 0x02, 0x16, 0x4a, 0x00,
    0x1f, 0xdb, 0xff, 0xd2, 0xff, 0xf8, 0xb2, 0xaa, 0x02, 0x70, 0xdf, 0xfb, 0x98, 0x00, 0xb5, 0x8e,
    0x70, 0x0f, 0x9e, 0xff, 0xb3, 0x3f, 0xff, 0xec, 0x4b, 0x81, 0x3d, 0x77, 0xff, 0x2d, 0xff, 0xb3,
    0x62, 0xe8, 0x0a, 0xf9, 0xc0, 0x00, 0x90, 0x0c, 0x1b, 0x12, 0x40, 0x78, 0x4d, 0xff, 0x12, 0x80,
    0x81, 0x19, 0xf0, 0x03, 0x81, 0x6f, 0xf7, 0xf4, 0x01, 0xcc, 0xc8, 0x28, 0x42, 0x3f, 0x7f, 0xfa,
    0xdf, 0xff, 0xe6, 0x02, 0xc0, 0x53, 0x1c, 0x02, 0x78, 0xff, 0x0a, 0xb1, 0x10, 0x06, 0x71, 0xdf,
    0xfc, 0xe7, 0xff, 0x99, 0x9f, 0xa0, 0x1d, 0xae, 0xff, 0xf4, 0x40, 0x17, 0x2c, 0xc8, 0x80, 0xc7,
    0xf8, 0x00, 0x1a, 0x00, 0xee, 0x63, 0xd8, 0x0d, 0x0a, 0xbf, 0xfe, 0x0f, 0xfd, 0xca, 0xec, 0x20,
    0x93, 0xd5, 0xff, 0x4b, 0x7f, 0xff, 0xd8, 0x97, 0x0d, 0x1a, 0xb0, 0x05, 0x33, 0xfd, 0x64, 0xc9,
    0xb8, 0x0c, 0x1d, 0x80, 0x14, 0x5f, 0xe3, 0xb6, 0x46, 0x40, 0x54, 0x5c, 0x02, 0x3c, 0xff, 0x92,
    0x2f, 0xee, 0x08, 0x83, 0xe0, 0x08, 0x18, 0x01, 0x45, 0x8e, 0x20, 0x13, 0x87, 0x00, 0x52, 0xc0,
    0x6a, 0xcb, 0x72, 0x01, 0x5d, 0xf8, 0x00, 0xa1, 0xff, 0xf7, 0x63, 0x88, 0x06, 0x71, 0xc0, 0x1d,
    0x70, 0x15, 0xd3, 0x19, 0x20, 0x57, 0x7d, 0xff, 0xf5, 0x80, 0x60, 0xd7, 0xc0, 0x01, 0x06, 0x42,
    0x04, 0x80, 0x60, 0x0c,
];
const CAM: &[u8] = &[
    0x02, 0x02, 0xde, 0x14, 0x0c, 0xe5, 0xc7, 0xc0, 0x40, 0x5a, 0xb2, 0x3d, 0x82, 0xce, 0x27, 0x81,
    0xe9, 0xa2, 0x78, 0x27, 0x4b, 0xc6, 0x33, 0xfa, 0x54, 0x58, 0x7c, 0xa0, 0xa2, 0x7e, 0x83, 0x02,
    0x96, 0x8a, 0x97, 0x33, 0xff, 0x82, 0x00, 0x1a, 0x10, 0x3f, 0xe0, 0x14, 0x39, 0x80, 0x10, 0x6e,
    0x00, 0x75, 0x80, 0x11, 0x58, 0xce, 0x00, 0x02, 0xf0, 0x3a, 0xdc, 0x08, 0xc4, 0xc8, 0x00, 0x01,
    0x57, 0x81, 0xd6, 0x20, 0x46, 0x96, 0x33, 0x80, 0x0a, 0xbc, 0x0e, 0xdb, 0x02, 0x39, 0x31, 0x9c,
    0x00, 0x55, 0xe0, 0x75, 0x08, 0x11, 0x85, 0x90, 0x00, 0x02, 0xaf, 0x03, 0xa0, 0xc0, 0x91, 0x2c,
    0x80, 0x00, 0x16, 0x78, 0x1c, 0x9e, 0x05, 0x65, 0x64, 0x00, 0x00, 0xc3, 0xc0, 0xe0, 0x90, 0x2d,
    0xbb, 0x19, 0xc0, 0x06, 0xde, 0x05, 0x8d, 0x81, 0x02, 0x18, 0xce, 0x00, 0x35, 0xf0, 0x15, 0x5c,
    0x00, 0x06, 0xc6, 0x70, 0x00, 0xdf, 0x80, 0x8d, 0x5f, 0xde, 0x66, 0x27, 0x00, 0x07, 0x3c, 0x04,
    0x76, 0xfd, 0x67, 0x31, 0x9c, 0x00, 0x58, 0x60, 0x41, 0x37, 0xd3, 0x15, 0x89, 0xc0, 0x06, 0xdc,
];
const MAPEM: &[u8] = &[
    0x02, 0x05, 0x00, 0x00, 0x30, 0x16, 0x08, 0x00, 0x03, 0x09, 0x4d, 0x83, 0x42, 0xfc, 0x9a, 0x94,
    0xef, 0xb2, 0x6b, 0x71, 0x93, 0x56, 0x0c, 0x6e, 0x32, 0x5c, 0xca, 0x00, 0x06, 0x13, 0x8a, 0x08,
    0x55, 0x8f, 0x22, 0x36, 0x71, 0x3c, 0x83, 0x27, 0x02, 0x8a, 0x24, 0x48, 0x2a, 0x48, 0x04, 0x20,
    0x00, 0x00, 0x36, 0xd0, 0x2f, 0xc0, 0xde, 0x22, 0x00, 0xb0, 0x61, 0xc1, 0x0f, 0xc0, 0x5b, 0xe4,
    0xf4, 0xc3, 0xc7, 0xab, 0xc9, 0x87, 0x83, 0x47, 0xf0, 0x02, 0xc5, 0xa0, 0x00, 0x04, 0x04, 0x40,
    0x59, 0x08, 0x08, 0x40, 0x00, 0x00, 0x2c, 0xec, 0xa2, 0x26, 0x80, 0x46, 0x0c, 0x05, 0x54, 0x66,
    0x23, 0xdd, 0x7d, 0xc2, 0x40, 0x12, 0x42, 0x20, 0x00, 0x00, 0xdb, 0x40, 0xd5, 0x00, 0xe2, 0x80,
    0x02, 0x0e, 0xce, 0x06, 0x1e, 0x01, 0x1f, 0x10, 0x2d, 0xbb, 0x76, 0xe1, 0xe6, 0xbd, 0xf8, 0x20,
    0xb0, 0x68, 0x00, 0x01, 0x02, 0x10, 0x06, 0x42, 0x22, 0x00, 0x00, 0x04, 0x9d, 0xd5, 0xc0, 0xa6,
    0x11, 0xc6, 0xf4, 0x82, 0x4e, 0x5f, 0x89, 0x20, 0x11, 0x21, 0x00, 0x00, 0x00, 0x6d, 0xa0, 0x78,
    0x7f, 0x16, 0x40, 0x01, 0x0e, 0x40, 0x7e, 0x83, 0xd4, 0x7b, 0xd9, 0x85, 0xef, 0xef, 0x06, 0x2c,
    0xd0, 0x80, 0xc0, 0x58, 0x2c, 0x00, 0x00, 0x81, 0x88, 0x02, 0xa1, 0x10, 0x00, 0x00, 0x02, 0x4e,
    0xf4, 0x1e, 0xfb, 0x09, 0xf5, 0x7c, 0xa1, 0x03, 0x4f, 0x5c, 0x90, 0x24, 0xd0, 0x80, 0x00, 0x00,
    0x36, 0x7a, 0x06, 0xad, 0xaa, 0x00, 0x08, 0x8f, 0x8e, 0x07, 0x46, 0x46, 0xf7, 0xb1, 0x08, 0x69,
    0x9b, 0xc4, 0x87, 0x39, 0x3f, 0x52, 0x0b, 0x0b, 0x80, 0x00, 0x70, 0x41, 0x00, 0xb6, 0x22, 0x00,
    0x00, 0x00, 0x4b, 0x4c, 0xea, 0xe9, 0x78, 0x1c, 0xae, 0xec, 0x43, 0xe3, 0x5d, 0x29, 0x20, 0x51,
    0xa1, 0x10, 0x00, 0x00, 0x6c, 0xe0, 0x89, 0x45, 0xd4, 0x00, 0x10, 0x94, 0x47, 0xb6, 0x22, 0x46,
    0x7c, 0x81, 0x8d, 0x51, 0xcf, 0x83, 0x13, 0x7c, 0x02, 0xc1, 0x61, 0x90, 0x00, 0x0e, 0x0a, 0x20,
    0x18, 0xc4, 0x44, 0x00, 0x00, 0x09, 0x69, 0x94, 0x1c, 0xdd, 0x83, 0xeb, 0x1c, 0x40, 0x71, 0x33,
    0xcd, 0x24, 0x17, 0x34, 0x02, 0x10, 0x00, 0x00, 0x2b, 0x38, 0x29, 0x4d, 0xdd, 0x10, 0x05, 0x51,
    0x09, 0xc4, 0x7d, 0xa3, 0x34, 0x83, 0xd8, 0xc2, 0x03, 0x1a, 0xc0, 0x43, 0x0b, 0x88, 0x10, 0xe1,
    0xbe, 0x70, 0xcd, 0x85, 0x01, 0xe0, 0x58, 0xc4, 0x00, 0x03, 0x83, 0x08, 0x0c, 0x31, 0x01, 0x08,
    0x00, 0x00, 0x05, 0xb4, 0xc6, 0x2e, 0x50, 0xc0, 0x8a, 0x88, 0x78, 0x03, 0x90, 0x0e, 0xc1, 0x77,
    0x04, 0x80, 0xea, 0x84, 0x00, 0x00, 0x02, 0xb6, 0x81, 0x01, 0xe5, 0x9d, 0x00, 0x04, 0x0f, 0xed,
    0xe6, 0x09, 0xaf, 0x4a, 0x05, 0x2f, 0x8d, 0x02, 0xee, 0xab, 0xc1, 0xe8, 0xce, 0x23, 0xe2, 0x13,
    0xb7, 0x05, 0x61, 0x68, 0x00, 0x08, 0x0e, 0xb0, 0xc4, 0x00, 0x04, 0x08, 0x04, 0x0f, 0x60, 0x40,
    0x80, 0x00, 0x00, 0x33, 0xf1, 0xf8, 0x96, 0x10, 0x00, 0x41, 0x02, 0xcc, 0x80, 0xa1, 0x00, 0x20,
    0x90, 0x41, 0x06, 0x04, 0x08, 0x00, 0x00, 0x03, 0x3e, 0xec, 0x78, 0xdd, 0x00, 0x04, 0x0f, 0xdb,
    0x38, 0x0a, 0x0f, 0x03, 0x0a, 0x04, 0x11, 0x60, 0x40, 0x80, 0x00, 0x00, 0x33, 0xe6, 0xf5, 0x9f,
    0xd0, 0x00, 0x41, 0x02, 0xcc, 0x80, 0xa1, 0x20, 0x80, 0xb0, 0x41, 0x26, 0x04, 0x08, 0x00, 0x00,
    0x03, 0x3e, 0x3d, 0x49, 0x99, 0x00, 0x04, 0x0f, 0xdb, 0x38, 0x0a, 0x11, 0x09, 0x0c, 0x04, 0x13,
    0x60, 0x40, 0x80, 0x00, 0x00, 0x33, 0xee, 0x85, 0xfb, 0x10, 0x00, 0x40, 0xce, 0x3f, 0x40, 0xa1,
    0x40, 0x50, 0xd0, 0x41, 0x46, 0x04, 0x08, 0x00, 0x00, 0x03, 0x68, 0x84, 0xde, 0x07, 0x50, 0x00,
    0x41, 0x32, 0x40, 0xc0, 0xa1, 0x30, 0x60, 0xe0,
];
const SPATEM: &[u8] = &[
    0x02, 0x04, 0x00, 0x00, 0x30, 0x16, 0x00, 0x38, 0x4a, 0x6c, 0x1a, 0x17, 0xe4, 0xd4, 0xa7, 0x7d,
    0x93, 0x5b, 0x8c, 0x9a, 0xb0, 0x63, 0x71, 0x92, 0xe6, 0x50, 0x00, 0x30, 0x9c, 0x50, 0x40, 0x00,
    0x0e, 0xa7, 0x1d, 0x4f, 0xf0, 0x10, 0x00, 0x22, 0x8c, 0xe2, 0xf9, 0x44, 0xff, 0x63, 0x5b, 0x63,
    0x48, 0x71, 0xad, 0xb1, 0xae, 0xa1, 0xae, 0xa7, 0x80, 0x10, 0xa1, 0xb8, 0xbe, 0x29, 0x40, 0xa0,
    0xd6, 0xd8, 0xd1, 0x5c, 0x6b, 0x6c, 0x6c, 0x0c, 0x6c, 0x0d, 0xe0, 0x06, 0x28, 0x0e, 0x2d, 0xfa,
    0x68, 0xf6, 0x35, 0xb6, 0x34, 0x57, 0x1a, 0xdb, 0x1b, 0x03, 0x1b, 0x03, 0x78, 0x02, 0x0a, 0x1b,
    0x8d, 0x8b, 0x8d, 0xc0, 0x0d, 0xa7, 0x0d, 0x11, 0xc6, 0xd3, 0x86, 0xd4, 0xc6, 0xd4, 0xde, 0x00,
    0xa2, 0x8a, 0xe3, 0x5f, 0x23, 0x6c, 0x43, 0x66, 0x03, 0x43, 0x71, 0xb3, 0x01, 0xbb, 0x71, 0xba,
    0xd1, 0x80, 0x30, 0xa0, 0x38, 0xaf, 0x79, 0x86, 0x50, 0xd6, 0xd8, 0xd1, 0x5c, 0x6b, 0x6c, 0x6c,
    0x0c, 0x6c, 0x0c, 0x60, 0x0e, 0x28, 0xce, 0x35, 0xfc, 0x36, 0xce, 0x36, 0x6a, 0x34, 0x87, 0x1b,
    0x35, 0x1b, 0x44, 0x1b, 0x44, 0x78, 0x04, 0x0a, 0x1b, 0x8d, 0x8b, 0x8d, 0xc0, 0x0d, 0xa7, 0x0d,
    0x15, 0xc6, 0xd3, 0x86, 0xe3, 0xc6, 0xe1, 0x46, 0x01, 0x22, 0x80, 0xe2, 0xdf, 0xa5, 0xcd, 0x03,
    0x5b, 0x63, 0x45, 0x71, 0xad, 0xb1, 0xb1, 0xc1, 0xb0, 0xd1, 0x80,
];
const IVIM: &[u8] = &[
    0x02, 0x06, 0x00, 0x12, 0x10, 0xdc, 0x82, 0x50, 0x00, 0x00, 0x00, 0x00, 0x88, 0x05, 0x58, 0xea,
    0xad, 0x57, 0x13, 0xd7, 0xa6, 0x4f, 0xff, 0xff, 0xfe, 0x11, 0xdb, 0xba, 0x1f, 0x08, 0xc0, 0xe1,
    0x11, 0x01, 0x40, 0x75, 0x0f, 0xe3, 0x54, 0x07, 0x48, 0xff, 0x1a, 0xc0, 0xab, 0xcf, 0xf5, 0xe2,
    0xbc, 0x30, 0x78, 0x44, 0x40, 0x2f, 0xff, 0x24, 0x11, 0x2f, 0x01, 0x29, 0x45, 0x70, 0xea, 0xf0,
    0x81, 0x60, 0x00, 0x02, 0x00, 0x04, 0x08, 0x01, 0x4e,
];
const CPM: &[u8] = &[
    0x01, 0x0e, 0xa6, 0xc0, 0x52, 0x47, 0x7a, 0x3f, 0x70, 0x0b, 0x56, 0x40, 0x27, 0x01, 0xc4, 0xd7,
    0x81, 0x62, 0xb9, 0xeb, 0x9c, 0x00, 0x06, 0xce, 0xd2, 0x83, 0xa6, 0x3e, 0xd7, 0xd0, 0x0b, 0xfe,
    0xca, 0x4c, 0xca, 0x0c, 0xcf, 0xff, 0xd0, 0x14, 0xc4, 0x40, 0x07, 0xb2, 0x10, 0x03, 0xe8, 0x19,
    0x01, 0x90, 0x32, 0x00, 0x8a, 0x03, 0xbb, 0x83, 0xe9, 0x2f, 0x94, 0xfd, 0x90, 0x65, 0x7f, 0xff,
    0xf5, 0xff, 0xff, 0xd4, 0x48, 0xfc, 0xe0, 0x0e, 0xe5, 0x00, 0x45, 0x03, 0xdd, 0xc1, 0xf5, 0x34,
    0xca, 0x7f, 0x6c, 0xb2, 0xbf, 0xff, 0xfa, 0xff, 0xff, 0xe9, 0xc1, 0xfe, 0x70, 0x07, 0x72, 0x80,
    0x22, 0x82, 0x6e, 0xe0, 0xfb, 0x55, 0x65, 0x3f, 0xb4, 0x99, 0x5f, 0xff, 0xfd, 0x7f, 0xff, 0xf6,
    0x07, 0x44, 0x10, 0x03, 0xb9, 0x40, 0x11, 0x41, 0x57, 0x70, 0x7d, 0x59, 0x32, 0x9f, 0xe7, 0xfc,
    0xaf, 0xff, 0xfe, 0xbf, 0xff, 0xfb, 0x42, 0x46, 0x88, 0x01, 0xdc, 0xa0, 0x08, 0xa1, 0x2b, 0xb8,
    0x3e, 0x25, 0x79, 0x4f, 0xe0, 0x3e, 0x57, 0xff, 0xff, 0x5f, 0xff, 0xfd, 0x2d, 0x4f, 0xce, 0x00,
    0xee, 0x50, 0x04, 0x50, 0xe5, 0xdc, 0x1f, 0x04, 0x3c, 0xa8, 0x08, 0xaf, 0x2b, 0xff, 0xff, 0xaf,
    0xff, 0xfe, 0xd4, 0x61, 0xe2, 0x00, 0x77, 0x28, 0x02, 0x08, 0x8e, 0xee, 0x0f, 0x9d, 0x56, 0x53,
    0xf3, 0x8b, 0x95, 0xff, 0xff, 0xd7, 0xff, 0xff, 0x5c, 0x23, 0xf0, 0x01, 0xb9, 0x40, 0x11, 0x5a,
    0x37, 0x70, 0x7b, 0xf5, 0xf2, 0xa0, 0x11, 0x5c, 0xaf, 0xff, 0xfe, 0xbf, 0xff, 0xfa, 0x35, 0xdf,
    0x9c, 0x01, 0xdc, 0xa0, 0x08, 0xa3, 0x5b, 0xb8, 0x3e, 0xbc, 0x19, 0x50, 0x08, 0x86, 0x57, 0xff,
    0xff, 0x5f, 0xff, 0xfd, 0x51, 0xdf, 0xce, 0x00, 0xee, 0x50, 0x04, 0x56, 0x45, 0xdc, 0x1e, 0xf1,
    0xbc, 0xa7, 0xe8, 0xa7, 0x2b, 0xff, 0xff, 0xaf, 0xff, 0xfe, 0xd4, 0x61, 0xe2, 0x00, 0x77, 0x28,
    0x02, 0x2a, 0xfa, 0xee, 0x0f, 0x82, 0x2e, 0x53, 0xfe, 0xb3, 0x95, 0xff, 0xff, 0xd7, 0xff, 0xff,
    0x4c, 0x9b, 0xf3, 0x80, 0x3b, 0x94, 0x01, 0x14, 0x33, 0x77, 0x07, 0xbb, 0xf3, 0x29, 0xf7, 0xa5,
    0xca, 0xff, 0xff, 0xeb, 0xff, 0xff, 0xac, 0x1f, 0xf9, 0xc0, 0x0d, 0xca, 0x00, 0x8a, 0x42, 0xbb,
    0x83, 0xdd, 0x55, 0x94, 0xfc, 0x49, 0xe5, 0x7f, 0xff, 0xf5, 0xff, 0xff, 0xd4, 0xe9, 0xfc, 0xe0,
    0x0e, 0xe5, 0x00, 0x45, 0x02, 0xdd, 0xc1, 0xf4, 0xf7, 0xca, 0x7e, 0xf6, 0xf2, 0xbf, 0xff, 0xfa,
    0xff, 0xff, 0xea, 0x1e, 0xfe, 0x70, 0x03, 0x72, 0x80, 0x22, 0x8f, 0xee, 0xe0, 0xf8, 0x6e, 0x65,
    0x3f, 0x98, 0x79, 0x5f, 0xff, 0xfd, 0x7f, 0xff, 0xf6, 0x25, 0x86, 0x10, 0x03, 0xb9, 0x40, 0x11,
    0x44, 0xd7, 0x70, 0x7c, 0x37, 0xb2, 0x9f, 0xef, 0x1c, 0xaf, 0xff, 0xfe, 0xbf, 0xff, 0xfb, 0x45,
    0x86, 0xc8, 0x01, 0xdc, 0xa0, 0x08, 0xa2, 0xfb, 0xb8, 0x3e, 0x08, 0xb9, 0x50, 0x04, 0x26, 0x57,
    0xff, 0xff, 0x5f, 0xff, 0xfd, 0x2a, 0xbf, 0xce, 0x00, 0xee, 0x50, 0x04, 0x52, 0x0d, 0xdc, 0x1e,
    0xfc, 0xec, 0xa7, 0xeb, 0x67, 0x2b, 0xf3, 0xdf, 0xaf, 0xc2, 0xfe, 0xd0, 0x61, 0x92, 0x00, 0x37,
    0x28, 0x02, 0x09, 0x3a, 0xee, 0x0f, 0x99, 0x3e, 0x53, 0xf2, 0xd9, 0x95, 0xff, 0xff, 0xd7, 0xff,
    0xff, 0x5c, 0x23, 0xf0, 0x01, 0xb9, 0x40, 0x10, 0x49, 0xf7, 0x70, 0x7b, 0xcf, 0x32, 0x9f, 0x72,
    0x2c, 0xaf, 0xfa, 0xfe, 0xbf, 0x31, 0xfb, 0x68, 0x29, 0x00, 0x0d, 0xca, 0x00, 0x8a, 0x4c, 0xbb,
    0x83, 0xe0, 0xff, 0x94, 0xfc, 0x5a, 0xe5, 0x7f, 0xff, 0xf5, 0xff, 0xff, 0xd3, 0x27, 0xfc, 0xe0,
    0x0e, 0xe5, 0x00, 0x41, 0x29, 0xdd, 0xc1, 0xef, 0x21, 0xca, 0x7f, 0xab, 0x72, 0xbf, 0xff, 0xfa,
    0xff, 0xff, 0xeb, 0x84, 0x7e, 0x00, 0x37, 0x28, 0x02, 0x08, 0xee, 0xee, 0x0f, 0xa1, 0x7e, 0x53,
    0xf4, 0xb3, 0x96, 0x04, 0x77, 0xd8, 0x05, 0xff, 0x43, 0xc7, 0xf0, 0x01, 0xb9, 0x40, 0x10, 0x4a,
    0x57, 0x70, 0x7d, 0x51, 0x72, 0xa0, 0x16, 0x7c, 0xaf, 0xff, 0xbe, 0xc0, 0x3c, 0xfa, 0x73, 0x9f,
    0x80, 0x0d, 0xca, 0x00, 0x82, 0x55, 0xbb, 0x83, 0xde, 0x51, 0x94, 0xff, 0x94, 0x65, 0x7f, 0xff,
    0xf5, 0xff, 0xff, 0xd6, 0x01, 0xfc, 0x00, 0x6e, 0x50, 0x04, 0x51, 0x8d, 0xdc, 0x1f, 0x16, 0xbc,
    0xa7, 0xf0, 0xaf, 0x2c, 0x1a, 0x2f, 0xaf, 0x61, 0x3e, 0xe5, 0x0b, 0x12, 0x00, 0x37, 0x28, 0xd0,
];

#[test]
fn round_trip() {
    let decoded = decode(DENM, Headers::None).unwrap();
    assert!(matches![
        decoded,
        ItsMessage::DenmV2 {
            geonetworking: None,
            transport: None,
            ..
        }
    ]);
    let encoded = decoded.encode(EncodingRules::UPER).unwrap();
    assert_eq!(encoded.as_slice(), DENM);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_wasm() {
    let expected_gn = r#"{"Unsecured":{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":408,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}},"payload":[7,209,0,0,2,1,224,253,29,55,231,70,90,168,188,128,6,145,13,100,201,4,4,67,89,50,69,157,89,1,146,7,19,95,33,66,190,43,224,0,24,106,9,136,0,80,20,64,24,3,0,20,251,132,63,10,47,221,107,251,0,197,10,6,2,45,41,127,245,159,255,230,6,128,95,156,0,255,0,13,49,36,4,155,224,6,72,2,173,137,32,19,135,0,27,192,57,76,50,129,18,248,1,165,254,213,97,68,9,95,192,13,47,252,147,17,0,21,222,0,87,127,70,88,6,2,200,111,253,27,252,206,199,216,11,183,127,213,159,205,183,14,66,137,251,255,123,0,143,51,204,5,119,224,4,24,1,13,134,240,23,111,0,25,191,238,236,168,3,163,55,252,102,4,30,101,104,16,253,192,2,240,2,19,23,64,109,94,0,74,127,241,216,191,1,81,112,3,123,248,98,205,240,19,215,128,7,160,2,22,74,0,31,219,255,210,255,248,178,170,2,112,223,251,152,0,181,142,112,15,158,255,179,63,255,236,75,129,61,119,255,45,255,179,98,232,10,249,192,0,144,12,27,18,64,120,77,255,18,128,129,25,240,3,129,111,247,244,1,204,200,40,66,63,127,250,223,255,230,2,192,83,28,2,120,255,10,177,16,6,113,223,252,231,255,153,159,160,29,174,255,244,64,23,44,200,128,199,248,0,26,0,238,99,216,13,10,191,254,15,253,202,236,32,147,213,255,75,127,255,216,151,13,26,176,5,51,253,100,201,184,12,29,128,20,95,227,182,70,64,84,92,2,60,255,146,47,238,8,131,224,8,24,1,69,142,32,19,135,0,82,192,106,203,114,1,93,248,0,161,255,247,99,136,6,113,192,29,112,21,211,25,32,87,125,255,245,128,96,215,192,1,6,66,4,128,96,12]}}"#;
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":408,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<DENM>(DENM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_denm(&json, 211).unwrap();
    let decoded = decode(
        &encoded.to_vec(),
        Headers::GnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    assert_eq!(json.its, decoded.its);
    assert_eq!(json.transport, decoded.transport);
    assert_eq!(expected_gn, decoded.geonetworking.unwrap().as_str())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_denm_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":408,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<DENM>(DENM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_denm(&json, 211).unwrap();
    let decoded = decode(
        &encoded.to_vec(),
        Headers::GnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    println!("{}", decoded.its.as_ref().unwrap());
    assert_eq!(json.its, decoded.its);
    assert_eq!(json.transport, decoded.transport);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_cam_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":164,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<CAM>(CAM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_cam(&json, 141).unwrap();
    let decoded = decode(
        &encoded.to_vec(),
        Headers::GnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    assert_eq!(json.its, decoded.its);
    assert_eq!(json.transport, decoded.transport);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_mapem_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":540,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<MAPEM>(MAPEM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_mapem(&json, 131).unwrap();
    let decoded = decode(
        &encoded.to_vec(),
        Headers::GnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    assert_eq!(json.its, decoded.its);
    assert_eq!(json.transport, decoded.transport);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_spatem_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":207,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<SPATEM>(SPATEM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_spatem(&json, 131).unwrap();
    let decoded = decode(
        &encoded.to_vec(),
        Headers::GnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    assert_eq!(json.its, decoded.its);
    assert_eq!(json.transport, decoded.transport);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_ivim_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":77,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<IVIM>(IVIM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_ivim(&json, 221).unwrap();
    let decoded = decode(
        &encoded.to_vec(),
        Headers::GnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    assert_eq!(json.its, decoded.its);
    assert_eq!(json.transport, decoded.transport);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_srem_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":43,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some("{\"header\":{\"protocolVersion\":2,\"messageID\":9,\"stationID\":760129084},\"srm\":{\"timeStamp\":98917,\"second\":23692,\"sequenceNumber\":87,\"requests\":[{\"request\":{\"id\":{\"id\":0},\"requestID\":0,\"requestType\":\"priorityRequestUpdate\",\"inBoundLane\":{\"approach\":0},\"outBoundLane\":{\"approach\":0}}}],\"requester\":{\"id\":{\"stationID\":3919},\"type\":{\"role\":\"publicTransport\"},\"position\":{\"position\":{\"lat\":535485106,\"long\":99886480},\"speed\":{\"transmisson\":\"unavailable\",\"speed\":232}},\"transitStatus\":\"00\",\"transitOccupancy\":\"occupancyMed\",\"transitSchedule\":4}}}".into()),
        ..Default::default()
    };
    let encoded = encode_srem(&json, 131).unwrap();
    let decoded = decode(
        &encoded.to_vec(),
        Headers::GnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    assert_eq!(json.its, decoded.its);
    assert_eq!(json.transport, decoded.transport);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_cpm_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":628,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<CPM>(CPM).unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_cpm(&json, 131).unwrap();
    let decoded = decode(
        &encoded.to_vec(),
        Headers::GnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    assert_eq!(json.its, decoded.its);
    assert_eq!(json.transport, decoded.transport);
}

#[test]
fn decode_pcap_frame() {
    let exp_geonetworking = r#"{"Unsecured":{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":5,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[true,false,false,false,false,false,false,false],"payload_length":67,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"PassengerCar","reserved":[false,false,false,false,false,false,false,false,false,false],"address":[138,176,248,168,162,37]},"timestamp":1151018751,"latitude":535505166,"longitude":99353789,"position_accuracy":true,"speed":14,"heading":724},"media_dependent_data":[0,0,0,0]}},"payload":[7,209,0,0,2,2,156,107,199,147,38,255,64,90,178,2,65,206,38,186,215,161,134,24,96,0,54,204,208,72,45,79,160,5,168,130,152,138,127,51,255,1,255,250,0,40,51,0,0,44,2,121,2,217,173,240,3,121,96,26,104,51,205,99,240,67,44]}}"#;
    let exp_tp = r#"{"destination_port":2001,"destination_port_info":0}"#;
    let exp_etsi = r#"{"header":{"protocolVersion":2,"messageID":2,"stationID":2624309139},"cam":{"generationDeltaTime":9983,"camParameters":{"basicContainer":{"stationType":5,"referencePosition":{"latitude":535505166,"longitude":99353789,"positionConfidenceEllipse":{"semiMajorConfidence":195,"semiMinorConfidence":195,"semiMajorOrientation":0},"altitude":{"altitudeValue":12230,"altitudeConfidence":"alt-005-00"}}},"highFrequencyContainer":{"basicVehicleContainerHighFrequency":{"heading":{"headingValue":724,"headingConfidence":126},"speed":{"speedValue":11,"speedConfidence":41},"driveDirection":"unavailable","vehicleLength":{"vehicleLengthValue":42,"vehicleLengthConfidenceIndication":"unavailable"},"vehicleWidth":18,"longitudinalAcceleration":{"longitudinalAccelerationValue":-1,"longitudinalAccelerationConfidence":102},"curvature":{"curvatureValue":0,"curvatureConfidence":"onePerMeter-0-00002"},"curvatureCalculationMode":"yawRateUsed","yawRate":{"yawRateValue":0,"yawRateConfidence":"unavailable"},"accelerationControl":"00","lateralAcceleration":{"lateralAccelerationValue":0,"lateralAccelerationConfidence":102}}},"lowFrequencyContainer":{"basicVehicleContainerLowFrequency":{"vehicleRole":"default","exteriorLights":"00","pathHistory":[{"pathPosition":{"deltaLatitude":317,"deltaLongitude":1460,"deltaAltitude":-940},"pathDeltaTime":1779},{"pathPosition":{"deltaLatitude":423,"deltaLongitude":3316,"deltaAltitude":-1310},"pathDeltaTime":4300}]}}}}}"#;
    let hex = "0000480002000040000004e5480038000200c900f40200000000000000000a014cff42ff34ff34ff7f390000f7c52687ebeb050091000c177c3d3c95fa000000000000000000000088000000ffffffffffff8ab0f8a8a225ffffffffffff00882300aaaa03000000894711000501205002800043010014008ab0f8a8a225449b26ff1feb290e05ec04bd800e02d40000000007d1000002029c6bc79326ff405ab20241ce26bad7a18618600036ccd0482d4fa005a882988a7f33ff01fffa00283300002c027902d9adf00379601a6833cd63f0432ce5dc898b";
    let raw = (0..hex.len())
        .step_by(2)
        .map(|s| u8::from_str_radix(&hex[s..s + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();
    let decoded = decode(&raw, Headers::RadioTap802LlcGnBtp).unwrap();
    if let ItsMessage::Cam {
        geonetworking: Some(geo),
        transport: Some(tp),
        etsi,
    } = decoded
    {
        assert_eq!(geo.encode_to_json().unwrap(), exp_geonetworking);
        assert_eq!(tp.encode_to_json().unwrap(), exp_tp);
        assert_eq!(
            ItsMessage::Cam {
                geonetworking: None,
                transport: None,
                etsi
            }
            .encode(EncodingRules::JER)
            .unwrap(),
            exp_etsi.as_bytes()
        );
    } else {
        panic!("Failed decode_pcap_frame!");
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn decode_pcap_frame_wasm() {
    let expected = ItsMessage {
        geonetworking: Some(
            r#"{"Unsecured":{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":5,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[true,false,false,false,false,false,false,false],"payload_length":67,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"PassengerCar","reserved":[false,false,false,false,false,false,false,false,false,false],"address":[138,176,248,168,162,37]},"timestamp":1151018751,"latitude":535505166,"longitude":99353789,"position_accuracy":true,"speed":14,"heading":724},"media_dependent_data":[0,0,0,0]}},"payload":[7,209,0,0,2,2,156,107,199,147,38,255,64,90,178,2,65,206,38,186,215,161,134,24,96,0,54,204,208,72,45,79,160,5,168,130,152,138,127,51,255,1,255,250,0,40,51,0,0,44,2,121,2,217,173,240,3,121,96,26,104,51,205,99,240,67,44]}}"#.into(),
        ),
        transport: Some(
            r#"{"destination_port":2001,"destination_port_info":0}"#.into(),
        ),
        its: Some(
            r#"{"header":{"protocolVersion":2,"messageID":2,"stationID":2624309139},"cam":{"generationDeltaTime":9983,"camParameters":{"basicContainer":{"stationType":5,"referencePosition":{"latitude":535505166,"longitude":99353789,"positionConfidenceEllipse":{"semiMajorConfidence":195,"semiMinorConfidence":195,"semiMajorOrientation":0},"altitude":{"altitudeValue":12230,"altitudeConfidence":"alt-005-00"}}},"highFrequencyContainer":{"basicVehicleContainerHighFrequency":{"heading":{"headingValue":724,"headingConfidence":126},"speed":{"speedValue":11,"speedConfidence":41},"driveDirection":"unavailable","vehicleLength":{"vehicleLengthValue":42,"vehicleLengthConfidenceIndication":"unavailable"},"vehicleWidth":18,"longitudinalAcceleration":{"longitudinalAccelerationValue":-1,"longitudinalAccelerationConfidence":102},"curvature":{"curvatureValue":0,"curvatureConfidence":"onePerMeter-0-00002"},"curvatureCalculationMode":"yawRateUsed","yawRate":{"yawRateValue":0,"yawRateConfidence":"unavailable"},"accelerationControl":"00","lateralAcceleration":{"lateralAccelerationValue":0,"lateralAccelerationConfidence":102}}},"lowFrequencyContainer":{"basicVehicleContainerLowFrequency":{"vehicleRole":"default","exteriorLights":"00","pathHistory":[{"pathPosition":{"deltaLatitude":317,"deltaLongitude":1460,"deltaAltitude":-940},"pathDeltaTime":1779},{"pathPosition":{"deltaLatitude":423,"deltaLongitude":3316,"deltaAltitude":-1310},"pathDeltaTime":4300}]}}}}}"#.into(),
        ),
        message_type: 2,
    };
    let hex = "0000480002000040000004e5480038000200c900f40200000000000000000a014cff42ff34ff34ff7f390000f7c52687ebeb050091000c177c3d3c95fa000000000000000000000088000000ffffffffffff8ab0f8a8a225ffffffffffff00882300aaaa03000000894711000501205002800043010014008ab0f8a8a225449b26ff1feb290e05ec04bd800e02d40000000007d1000002029c6bc79326ff405ab20241ce26bad7a18618600036ccd0482d4fa005a882988a7f33ff01fffa00283300002c027902d9adf00379601a6833cd63f0432ce5dc898b";
    let raw = (0..hex.len())
        .step_by(2)
        .map(|s| u8::from_str_radix(&hex[s..s + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();
    let decoded = decode(
        &raw,
        Headers::RadioTap802LlcGnBtp,
        EncodingRules::UPER,
        EncodingRules::JER,
    )
    .unwrap();
    assert_eq!(expected, decoded)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn strip_headers_frame_wasm() {
    let expected = ItsMessage {
        geonetworking: Some(
            r#"{"Unsecured":{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":5,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[true,false,false,false,false,false,false,false],"payload_length":67,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"PassengerCar","reserved":[false,false,false,false,false,false,false,false,false,false],"address":[138,176,248,168,162,37]},"timestamp":1151018751,"latitude":535505166,"longitude":99353789,"position_accuracy":true,"speed":14,"heading":724},"media_dependent_data":[0,0,0,0]}},"payload":[7,209,0,0,2,2,156,107,199,147,38,255,64,90,178,2,65,206,38,186,215,161,134,24,96,0,54,204,208,72,45,79,160,5,168,130,152,138,127,51,255,1,255,250,0,40,51,0,0,44,2,121,2,217,173,240,3,121,96,26,104,51,205,99,240,67,44]}}"#.into(),
        ),
        transport: Some(
            r#"{"destination_port":2001,"destination_port_info":0}"#.into(),
        ),
        its: Some(
            r#"02029C6BC79326FF405AB20241CE26BAD7A18618600036CCD0482D4FA005A882988A7F33FF01FFFA00283300002C027902D9ADF00379601A6833CD63F0432C"#.into(),
        ),
        message_type: 2,
    };
    let hex = "0000480002000040000004e5480038000200c900f40200000000000000000a014cff42ff34ff34ff7f390000f7c52687ebeb050091000c177c3d3c95fa000000000000000000000088000000ffffffffffff8ab0f8a8a225ffffffffffff00882300aaaa03000000894711000501205002800043010014008ab0f8a8a225449b26ff1feb290e05ec04bd800e02d40000000007d1000002029c6bc79326ff405ab20241ce26bad7a18618600036ccd0482d4fa005a882988a7f33ff01fffa00283300002c027902d9adf00379601a6833cd63f0432ce5dc898b";
    let raw = (0..hex.len())
        .step_by(2)
        .map(|s| u8::from_str_radix(&hex[s..s + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();
    let decoded = decode(
        &raw,
        Headers::RadioTap802LlcGnBtp,
        EncodingRules::UPER,
        EncodingRules::UPER,
    )
    .unwrap();
    assert_eq!(expected, decoded)
}

#[test]
fn ivim_xer_impl() {
    assert_eq!(
        "<IVIM><header><protocolVersion>2</protocolVersion><messageID>6</messageID><stationID>1183964</stationID></header><ivi><mandatory><serviceProviderId><countryCode>1001010000</countryCode><providerIdentifier>0</providerIdentifier></serviceProviderId><iviIdentificationNumber>1</iviIdentificationNumber><iviStatus>1</iviStatus></mandatory><optional><IviContainer><glc><GeographicLocationContainer><referencePosition><latitude>535413205</latitude><longitude>99854436</longitude><positionConfidenceEllipse><semiMajorConfidence>4095</semiMajorConfidence><semiMinorConfidence>4095</semiMinorConfidence><semiMajorOrientation>3601</semiMajorOrientation></positionConfidenceEllipse><altitude><altitudeValue>800001</altitudeValue><altitudeConfidence><unavailable /></altitudeConfidence></altitude></referencePosition><parts><GlcPart><zoneId>1</zoneId><zoneHeading>3601</zoneHeading><zone><segment><Segment><line><deltaPositions><laneWidth><DeltaPosition><deltaLatitude>937</deltaLatitude><deltaLongitude>-917</deltaLongitude></DeltaPosition><DeltaPosition><deltaLatitude>933</deltaLatitude><deltaLongitude>-458</deltaLongitude></DeltaPosition><DeltaPosition><deltaLatitude>1375</deltaLatitude><deltaLongitude>-323</deltaLongitude></DeltaPosition></laneWidth></deltaPositions></line><IviLaneWidth>350</IviLaneWidth></Segment></segment></zone></GlcPart><GlcPart><zoneId>2</zoneId><zoneHeading>3601</zoneHeading><zone><segment><Segment><line><deltaPositions><laneWidth><DeltaPosition><deltaLatitude>-27</deltaLatitude><deltaLongitude>2200</deltaLongitude></DeltaPosition><DeltaPosition><deltaLatitude>595</deltaLatitude><deltaLongitude>11144</deltaLongitude></DeltaPosition></laneWidth></deltaPositions></line><IviLaneWidth>350</IviLaneWidth></Segment></segment></zone></GlcPart></parts></GeographicLocationContainer></glc></IviContainer><IviContainer><giv><GeneralIviContainer><GicPart><detectionZoneIds><its-Rrid>2</its-Rrid></detectionZoneIds><direction><driverAwarenessZoneIds>1</driverAwarenessZoneIds></direction><minimumAwarenessTime>0</minimumAwarenessTime><laneStatus>1</laneStatus><RoadSignCodes><RSCode><code><iso14823><ISO14823Code><pictogramCode><serviceCategoryCode><trafficSignPictogram><pictogramCategoryCode><dangerWarning /></pictogramCategoryCode></trafficSignPictogram></serviceCategoryCode><attributes><nature>3</nature><serialNumber>78</serialNumber></attributes></pictogramCode></ISO14823Code></iso14823></code></RSCode></RoadSignCodes></GicPart></GeneralIviContainer></giv></IviContainer></optional></ivi></IVIM>".as_bytes(),
        decode(
            &[
                0x02, 0x06, 0x00, 0x12, 0x10, 0xdc, 0x82, 0x50, 0x00, 0x00, 0x00, 0x00, 0x88, 0x05,
                0x58, 0xea, 0xad, 0x57, 0x13, 0xd7, 0xa6, 0x4f, 0xff, 0xff, 0xfe, 0x11, 0xdb, 0xba,
                0x1f, 0x08, 0xc0, 0xe1, 0x11, 0x01, 0x40, 0x75, 0x0f, 0xe3, 0x54, 0x07, 0x48, 0xff,
                0x1a, 0xc0, 0xab, 0xcf, 0xf5, 0xe2, 0xbc, 0x30, 0x78, 0x44, 0x40, 0x2f, 0xff, 0x24,
                0x11, 0x2f, 0x01, 0x29, 0x45, 0x70, 0xea, 0xf0, 0x81, 0x60, 0x00, 0x02, 0x00, 0x04,
                0x08, 0x01, 0x4e
            ],
            Headers::None
        )
        .unwrap().encode(EncodingRules::XER).unwrap()
    )
}

#[test]
fn xer_to_uper() {
    assert_eq!(
        &[0x01,0x06,0x00,0x00,0x00,0x00,0xba,0x50,0x00,0xc8,0x00,0x01,0x90,0x15,0xbd,0xce,0x3a,0x64,0x05,0x2a,0xc9,0x55,0xd9,0x01,0x5b,0xdc,0xdd,0x94,0x20,0x15,0x0d,0x3f,0xd0,0x5c,0x3b,0xc1,0x8b,0x40,0x04,0x00,0x40,0x03,0x6e,0xe8,0x40,0x41,0x00,0x02,0x57,0xfc,0x06,0x63,0x15,0x92,0xa3,0xe9,0x3f,0x1a,0x72,0xde,0xed,0xe9,0xae,0x9d,0xf4,0x03,0xba,0xbc,0x4f,0x70,0x29,0xa1,0x11,0x77,0xf8,0xb9,0x8e,0x97,0x52,0x33,0x95,0x38,0x71,0x6b,0xd9,0x52,0xb6,0xdf,0xc3,0x99,0xae,0x69,0xbb,0xd9,0xdf,0xde,0x80,0x07,0xa5,0x41,0x66,0x6d,0x7a,0xbd,0x76,0x5e,0x5f,0xb2,0x66,0xe6,0x00,0x9c,0x0d,0x36,0x66,0xed,0xcd,0x64,0x65,0xd7,0x1d,0xde,0x3e,0x3d,0xa7,0xed,0xca,0x62,0xcc,0x1f,0xf7,0x36,0x30,0xb6,0x10,0x40,0x18,0xff,0xff,0xbf,0xff,0xed,0xfc,0x89,0xa5,0xa8,0xe3,0xbd,0x9d,0x25,0x8e,0x77,0x0a,0x03,0xd2,0xe9,0x59,0x22,0x17,0xee,0xb3,0x42,0x41,0xfc,0xec,0xfb,0xa6,0x42,0xce,0xfb,0xb2,0x9e,0xa4,0xf4,0x6c,0x30,0x9a,0x6f,0x86,0x3b,0x65,0x4a,0xfb,0x76,0x3a,0xa9,0x8f,0xdd,0x43,0xda,0x89,0x7f,0xc0,0x66,0x31,0x41,0x08,0x00,0xfa,0x3a,0xaf,0x8f,0x45,0xa7,0x2d,0xf9,0x85,0xba,0xc8,0x8f,0x9f,0xbd,0xaf,0xe5,0xfa,0x59,0xdb,0x3b,0x1f,0xb0,0x9d,0xb7,0x50,0x7b,0xcb,0xbb,0xca,0xcf,0xd9,0x5d,0xbf,0xb3,0xfe,0xc8,0xe2,0x15,0x84,0x20,0x08,0x00,0x20,0x02,0xa2,0x22,0x82,0x64,0x04,0x4a,0x10,0x10,0x4a,0x8c,0xae,0x6e,0x84,0x05,0xe4,0x0a,0x6e,0x8e,0x4c,0x2e,0x8c,0xac,0xed,0x2c,0xa4,0x09,0xce,0x45,0xc4,0x08,0x25,0xa6,0x64,0x18,0x73,0x8c,0x4c,0xae,0x4d,0x8c,0x2e,0x6e,0x8e,0xad,0xcc,0xe4,0x0c,0x8c,0xae,0x44,0x08,0x48,0x28,0x44,0x06,0x87,0x24,0x08,0x2a,0x61,0x44,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x09,0x6a,0x65,0xaa,0xec,0x2d,0x8c,0x8c,0x2e,0xa1,0x44,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x03,0x08,0x00,0x21,0x08,0x15,0x11,0x14,0x13,0x1e],
        decode(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns11:iVIM xmlns="http://www.bast.de/RWS" xmlns:ns6="http://www.bast.de/CITSapplMgmtIDs"
    xmlns:ns5="http://www.bast.de/IVI" xmlns:ns8="http://www.bast.de/GDD"
    xmlns:ns7="http://www.bast.de/ElectronicRegistrationIdentificationVehicleDataModule"
    xmlns:ns9="http://www.bast.de/DSRC" xmlns:ns11="http://www.bast.de/IRS-ICS-Interface"
    xmlns:ns10="http://www.bast.de/DENM-PDU-Descriptions"
    xmlns:ns2="http://www.bast.de/ITS-Container"
    xmlns:ns4="http://www.bast.de/EfcDsrcApplication"
    xmlns:ns3="http://www.bast.de/IVIM-PDU-Descriptions">
    <ns3:header>
        <ns2:protocolVersion>1</ns2:protocolVersion>
        <ns2:messageID>6</ns2:messageID>
        <ns2:stationID>0</ns2:stationID>
    </ns3:header>
    <ns3:ivi>
        <ns5:mandatory>
            <ns5:serviceProviderId>
                <ns4:countryCode>1001010000</ns4:countryCode>
                <ns4:providerIdentifier>50</ns4:providerIdentifier>
            </ns5:serviceProviderId>
            <ns5:iviIdentificationNumber>1</ns5:iviIdentificationNumber>
            <ns5:timeStamp>1718351679034</ns5:timeStamp>
            <ns5:validFrom>1718333678935</ns5:validFrom>
            <ns5:validTo>1718351678937</ns5:validTo>
            <ns5:iviStatus>2</ns5:iviStatus>
        </ns5:mandatory>
        <ns5:optional>
            <ns5:IviContainer>
                <ns5:glc>
                    <ns5:referencePosition>
                        <ns2:latitude>512759361</ns2:latitude>
                        <ns2:longitude>94712877</ns2:longitude>
                        <ns2:positionConfidenceEllipse>
                            <ns2:semiMajorConfidence>1</ns2:semiMajorConfidence>
                            <ns2:semiMinorConfidence>1</ns2:semiMinorConfidence>
                            <ns2:semiMajorOrientation>0</ns2:semiMajorOrientation>
                        </ns2:positionConfidenceEllipse>
                        <ns2:altitude>
                            <ns2:altitudeValue>800001</ns2:altitudeValue>
                            <ns2:altitudeConfidence>alt-000-01</ns2:altitudeConfidence>
                        </ns2:altitude>
                    </ns5:referencePosition>
                    <ns5:parts>
                        <ns5:GlcPart>
                            <ns5:zoneId>1</ns5:zoneId>
                            <ns5:zone>
                                <ns5:segment>
                                    <ns5:line>
                                        <ns5:deltaPositions>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>65409</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>78219</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>75080</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>84473</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>85596</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>95989</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>89404</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>106526</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>87946</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>112661</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>82467</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>114630</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>78291</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>108826</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>76401</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>101215</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>76375</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>94178</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>78685</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>85471</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>80892</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>81924</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>84611</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>78700</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>87983</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>77616</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>91342</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>77829</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>98727</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>78711</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>105161</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>77497</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>113608</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>73428</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>121749</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>71265</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>130791</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>71772</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                        </ns5:deltaPositions>
                                    </ns5:line>
                                </ns5:segment>
                            </ns5:zone>
                        </ns5:GlcPart>
                        <ns5:GlcPart>
                            <ns5:zoneId>2</ns5:zoneId>
                            <ns5:zone>
                                <ns5:segment>
                                    <ns5:line>
                                        <ns5:deltaPositions>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>0</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>0</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-16494</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-77099</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-14468</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-71379</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-12574</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-65046</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-11597</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-61248</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-10647</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-57089</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-9736</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-52713</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-8329</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-45229</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-5927</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-31532</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-3896</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-19802</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-2323</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-10931</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>-1111</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>-4795</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>65409</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>78219</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                        </ns5:deltaPositions>
                                    </ns5:line>
                                </ns5:segment>
                            </ns5:zone>
                        </ns5:GlcPart>
                        <ns5:GlcPart>
                            <ns5:zoneId>3</ns5:zoneId>
                            <ns5:zone>
                                <ns5:segment>
                                    <ns5:line>
                                        <ns5:deltaPositions>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>83798</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>116643</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>85596</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>117806</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>88338</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>118751</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>90060</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>119503</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>92004</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>120911</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>93857</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>122462</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>96602</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>126127</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                            <ns5:DeltaPosition>
                                                <ns5:deltaLatitude>98152</ns5:deltaLatitude>
                                                <ns5:deltaLongitude>128584</ns5:deltaLongitude>
                                            </ns5:DeltaPosition>
                                        </ns5:deltaPositions>
                                    </ns5:line>
                                </ns5:segment>
                            </ns5:zone>
                        </ns5:GlcPart>
                    </ns5:parts>
                </ns5:glc>
            </ns5:IviContainer>
            <ns5:IviContainer>
                <ns5:giv>
                    <ns5:GicPart>
                        <ns5:detectionZoneIds>2</ns5:detectionZoneIds>
                        <ns5:relevanceZoneIds>1</ns5:relevanceZoneIds>
                        <ns5:direction>0</ns5:direction>
                        <ns5:iviType>2</ns5:iviType>
                        <ns5:laneStatus>0</ns5:laneStatus>
                        <ns5:roadSignCodes>
                            <ns5:RSCode>
                                <ns5:code>
                                    <ns5:iso14823>
                                        <ns5:pictogramCode>
                                            <ns5:countryCode>4445</ns5:countryCode>
                                            <ns5:serviceCategoryCode>
                                                <ns5:trafficSignPictogram>regulatory</ns5:trafficSignPictogram>
                                            </ns5:serviceCategoryCode>
                                            <ns5:pictogramCategoryCode>
                                                <ns5:nature>4</ns5:nature>
                                                <ns5:serialNumber>16</ns5:serialNumber>
                                            </ns5:pictogramCategoryCode>
                                        </ns5:pictogramCode>
                                    </ns5:iso14823>
                                </ns5:code>
                            </ns5:RSCode>
                        </ns5:roadSignCodes>
                        <ns5:extraText>
                            <ns5:Text>
                                <ns5:layoutComponentId>1</ns5:layoutComponentId>
                                <ns5:language>1001010000</ns5:language>
                                <ns5:textContent>Test / Strategie Nr. A-3 Überlastung der BAB 49 AS
                                    KS-Waldau
                                </ns5:textContent>
                            </ns5:Text>
                        </ns5:extraText>
                    </ns5:GicPart>
                    <ns5:GicPart>
                        <ns5:relevanceZoneIds>3</ns5:relevanceZoneIds>
                        <ns5:direction>0</ns5:direction>
                        <ns5:iviType>2</ns5:iviType>
                        <ns5:laneStatus>1</ns5:laneStatus>
                        <ns5:roadSignCodes>
                            <ns5:RSCode>
                                <ns5:code>
                                    <ns5:iso14823>
                                        <ns5:pictogramCode>
                                            <ns5:countryCode>4445</ns5:countryCode>
                                            <ns5:serviceCategoryCode>
                                                <ns5:trafficSignPictogram>regulatory</ns5:trafficSignPictogram>
                                            </ns5:serviceCategoryCode>
                                            <ns5:pictogramCategoryCode>
                                                <ns5:nature>4</ns5:nature>
                                                <ns5:serialNumber>15</ns5:serialNumber>
                                            </ns5:pictogramCategoryCode>
                                        </ns5:pictogramCode>
                                    </ns5:iso14823>
                                </ns5:code>
                            </ns5:RSCode>
                        </ns5:roadSignCodes>
                    </ns5:GicPart>
                </ns5:giv>
            </ns5:IviContainer>
        </ns5:optional>
    </ns3:ivi>
</ns11:iVIM>"#.as_bytes(),
            Headers::None
        )
        .unwrap().encode(EncodingRules::UPER).unwrap().as_slice()
    )
}

#[test]
fn meise() {
    let trace = &[
        (50.94422230828326, 4.319819885031933, 14.0),
        (50.94433187141461, 4.319715144999749, 14.421098860177215),
        (50.94444597095791, 4.3196161245273235, 14.49320767775803),
        (50.944559638977594, 4.31951469123179, 14.514811321797245),
        (50.94467131673817, 4.3194062874606445, 14.6189470632111),
        (50.94478556407996, 4.3193016659797046, 14.682561147322852),
        (50.944900460304325, 4.319196738977789, 14.802954005088964),
        (50.94501652833029, 4.319091253024284, 15.0),
        (50.945133701768256, 4.318985164664194, 15.0),
        (50.945251023007096, 4.318878871394504, 15.0168848902918),
        (50.94536915616482, 4.318771452488812, 15.126524050696252),
        (50.94548728932255, 4.3186640335831195, 15.236163211100706),
        (50.945604831072174, 4.3185550020315, 15.186542505347113),
        (50.945722011023356, 4.318444983931371, 15.039493355613738),
        (50.945838567922685, 4.318339047030385, 14.923424323493593),
        (50.94595489604393, 4.31823460870277, 14.81873086450616),
        (50.946071042806274, 4.31813029111573, 14.714436502483725),
        (50.94618505113332, 4.318027397202462, 14.614847964382214),
        (50.94629905946037, 4.317924503289194, 14.515259426280704),
        (50.94641161205896, 4.317822188154519, 14.327560481027687),
        (50.94652058194486, 4.317721297459534, 13.92301184140601),
        (50.94662815722099, 4.317622020240405, 13.538721389721076),
        (50.94673103752013, 4.317528174814629, 13.222630465233616),
        (50.94683391781927, 4.317434329388853, 12.906539540746156),
        (50.946936545860275, 4.317355522567653, 12.350665240545771),
        (50.9470333145962, 4.3172772925351275, 11.805377385949486),
        (50.94712084174537, 4.317187983219879, 11.387249883754215),
        (50.947203862259464, 4.317097490783578, 11.009065026333412),
        (50.94728368040497, 4.317008129515364, 10.705354904822993),
        (50.9473641625404, 4.316932081872727, 10.098687722522307),
        (50.94743965722933, 4.316862970870509, 9.371451965682459),
        (50.947516335846224, 4.316811043750338, 9.166666666666666),
        (50.94759239612578, 4.316762400652654, 9.039595416898765),
        (50.94766795655601, 4.31671864463064, 8.88888888888889),
        (50.9477427066701, 4.316678352466695, 8.676609946240504),
        (50.94781734014829, 4.316650262901493, 8.361922973563011),
        (50.94789185969071, 4.316636472748873, 8.333333333333334),
        (50.94797038387738, 4.316627583208264, 9.170845859519616),
        (50.9480552430112, 4.316621851648526, 9.713873196432283),
        (50.94814386288949, 4.316620367269, 9.992592251580433),
        (50.94823274368591, 4.316641448755879, 10.0),
        (50.94831540967479, 4.3166867193602485, 9.562363238969166),
        (50.94838872921343, 4.316752345004003, 9.246116512149998),
        (50.94845141882431, 4.316835479871176, 8.96655452985179),
        (50.94850886391968, 4.316922414699823, 8.713783602372512),
        (50.94855987385879, 4.317015550028569, 8.61111111111111),
        (50.948601458083935, 4.317117472442676, 8.44349133439683),
        (50.94863393184565, 4.317225219285284, 8.333333333333334),
        (50.94865954234557, 4.3173362199112155, 8.267855979851417),
        (50.94868143448961, 4.317447911617591, 8.074656537548032),
        (50.94868592893993, 4.317562509210526, 8.055555555555555),
        (50.94868445382767, 4.317678201854447, 8.216243928558843),
        (50.948661989492166, 4.3177979066608385, 9.207489351369068),
        (50.948630000501055, 4.317926590365898, 10.13682114908728),
        (50.948584957212645, 4.318060618026053, 11.270783309759247),
        (50.94851835153094, 4.318194194278128, 12.828237010149655),
        (50.94842650034292, 4.318324428461302, 14.61717292102932),
        (50.94831370799732, 4.3184583361603135, 16.897127820391358),
        (50.94818091218048, 4.318607516480925, 19.419020740483052),
        (50.948033413708515, 4.318784567909814, 21.57083033222572),
        (50.94787098504141, 4.318982611432205, 23.90441409876436),
        (50.94769120035218, 4.319194802250152, 25.8092065550834),
        (50.947507524348254, 4.3194361945687305, 27.555162372103297),
        (50.94731892403834, 4.319702134754763, 28.583396109402113),
        (50.9471192568704, 4.31996549440512, 29.103699512791223),
        (50.946908502439555, 4.320216105961631, 29.463289165837057),
        (50.94669776011728, 4.320473847537651, 29.616804710471854),
        (50.946486262503505, 4.32073206726176, 29.779259556117655),
        (50.94627310950077, 4.320991334953504, 29.961306863509908),
        (50.94605929807922, 4.321251803751785, 30.0),
        (50.94584562603829, 4.3215126110854944, 29.966833295645692),
        (50.94563325039016, 4.321773477051128, 29.798070338700256),
        (50.94542020429054, 4.322032956297863, 29.808327852984032),
        (50.94520661088888, 4.32229130354011, 29.964723115397618),
    ];
    for i in 0..trace.len() {
        let (lat, lon, speed_ms): (f64, f64, f64) = *trace.get(i).unwrap();
        let (prev_lat, prev_lon, _): (f64, f64, f64) = *if i != 0 {
            trace.get(i - 1).unwrap()
        } else {
            &(0., 0., 0.)
        };
        let bearing = (((lon.to_radians() - prev_lon.to_radians()).sin() * lat.to_radians().cos())
            .atan2(
                prev_lat.to_radians().cos() * lat.to_radians().sin()
                    - prev_lat.to_radians().sin()
                        * lat.to_radians().cos()
                        * (lon.to_radians() - prev_lon.to_radians()).cos(),
            )
            .to_degrees()
            + 360.0)
            % 360.0;
        let heading_etsi = (bearing * 10.) as u16;
        let lat_etsi = (lat * 10_000_000.) as i32;
        let lon_etsi = (lon * 10_000_000.) as i32;
        let speed_etsi = (speed_ms * 10.0) as u16;

        println!(
            r#"{{"header":{{"protocolVersion":2,"messageID":2}},"cam":{{"camParameters":{{"basicContainer":{{"stationType":10,"referencePosition":{{"latitude":{lat_etsi},"longitude":{lon_etsi}}}}},"highFrequencyContainer":{{"basicVehicleContainerHighFrequency":{{"heading":{{"headingValue":{heading_etsi}}},"speed":{{"speedValue":{speed_etsi}}}}}}}}}}}}},"#
        );
        if i < 34 {
            let id = i + 1;
            println!(
                r#"{{"header":{{"protocolVersion":2,"messageID":9}},"srm":{{"sequenceNumber": {id},"requests":[{{"request":{{"id":{{"region":31132,"id":11}},"requestType":"priorityRequestUpdate",}}}}]}}}},"#
            );
        } else {
            println!("null,")
        }
    }
}

#[test]
fn autobahn_debug() {
    let hx = hex::decode("12001A0203810040038082014D204001000119020050D500003C00005056BE08134D8F4A9D1C94B9A0082CB144800000001C95E94F0830468202BC00000000000007D207D20201000FB70DE30007CA2400689389B1E953A4E26C7A54ED23AD24F737A1882320320E1125C98F400F01E700C104805F604BAC73993093E7F2F8E7326611D4AF31CE62280FC4C3D34ADFA9E39C3C974DEC5639C3BA8EDF7EE39C39FBCD9ED639C39F22D64B639C3CC22E3DFE39C3D85CE426E39C3DDB8E050E39C3E742E1CE639C3E8BECC41639C3EAD6CC7A639C3CD54C456E39C44F4D2B7EA78E70F50C394A38E70E14D328D38E70F8EBBE1BF8E70F9253FF198E70FAC841AAB8E70FF0841E798E71010AC13E38E710536C1AD18E71036D4036D8E710523BF9BD8E7109AF3D4098E710608BCCBD8E71030DBC42B8E70FFFFBC17D8E70FD773CFDD8E70F9E3BDB318E70229494F7FFFDFFFFC7380001F289001A005001250002628EF81C46701C94B9A0082CB144616581010180030080FB9FE6571F7CE7F9108300000000002801C0938400A8010280012581050401FFFFFF80018B81070601C04001FFF8808182454E517CDB18A0C6C4BF854F8BA248C0D6ED06C4F380DDF6AB3E2B1F1182192A8080A0B3C6C4E16E28F9650AF4859405FC1D2ABEB7C944C2D02F762AD2C7851AD5F0A01300A14FF14E7DF6A61FD5E1D1637647E82F6CD4CABCF5E41B4F98C8D7076281809AA39EA8A0053DA119ECBC03E5B156A6324618C0652EB61DE8081143A4E987AC296AE742C32C92D916DDE0167A58F391AF90114ECC5DC4A9B619992642608145").unwrap();
    let cam = decode(&hx, Headers::GnBtp);
    println!("{:?}", cam.unwrap())
    // if let Ok(ItsMessage::Cam { etsi, .. }) = cam {
    //     println!("{}", hex::encode(ItsMessage::Cam { geonetworking: None, transport: None, etsi }.encode(EncodingRules::UPER).unwrap()))
    // }
}
