use etsi_web::{de::decode, EncodingRules, Headers, ItsMessage};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
#[cfg(not(target_arch = "wasm32"))]
use geonetworking::Encode;
#[cfg(not(target_arch = "wasm32"))]
use pretty_assertions::assert_eq;
#[cfg(not(target_arch = "wasm32"))]
use xmltree::Element;

#[cfg(target_arch = "wasm32")]
use etsi_web::en::{
    encode_cam, encode_cpm, encode_denm, encode_ivim, encode_mapem, encode_spatem, encode_srem,
};
#[cfg(not(target_arch = "wasm32"))]
use etsi_web::standards::ivim_2_2_1::ivim_pdu_descriptions::{
    DeltaLatitude, DeltaLongitude, DeltaPosition, DeltaPositions, IviLaneWidth, PolygonalLine,
    Segment,
};

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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_wasm() {
    let expected_gn = r#"{"Unsecured":{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":408,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}},"payload":[7,209,0,0,2,1,224,253,29,55,231,70,90,168,188,128,6,145,13,100,201,4,4,67,89,50,69,157,89,1,146,7,19,95,33,66,190,43,224,0,24,106,9,136,0,80,20,64,24,3,0,20,251,132,63,10,47,221,107,251,0,197,10,6,2,45,41,127,245,159,255,230,6,128,95,156,0,255,0,13,49,36,4,155,224,6,72,2,173,137,32,19,135,0,27,192,57,76,50,129,18,248,1,165,254,213,97,68,9,95,192,13,47,252,147,17,0,21,222,0,87,127,70,88,6,2,200,111,253,27,252,206,199,216,11,183,127,213,159,205,183,14,66,137,251,255,123,0,143,51,204,5,119,224,4,24,1,13,134,240,23,111,0,25,191,238,236,168,3,163,55,252,102,4,30,101,104,16,253,192,2,240,2,19,23,64,109,94,0,74,127,241,216,191,1,81,112,3,123,248,98,205,240,19,215,128,7,160,2,22,74,0,31,219,255,210,255,248,178,170,2,112,223,251,152,0,181,142,112,15,158,255,179,63,255,236,75,129,61,119,255,45,255,179,98,232,10,249,192,0,144,12,27,18,64,120,77,255,18,128,129,25,240,3,129,111,247,244,1,204,200,40,66,63,127,250,223,255,230,2,192,83,28,2,120,255,10,177,16,6,113,223,252,231,255,153,159,160,29,174,255,244,64,23,44,200,128,199,248,0,26,0,238,99,216,13,10,191,254,15,253,202,236,32,147,213,255,75,127,255,216,151,13,26,176,5,51,253,100,201,184,12,29,128,20,95,227,182,70,64,84,92,2,60,255,146,47,238,8,131,224,8,24,1,69,142,32,19,135,0,82,192,106,203,114,1,93,248,0,161,255,247,99,136,6,113,192,29,112,21,211,25,32,87,125,255,245,128,96,215,192,1,6,66,4,128,96,12]}}"#;
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":408,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<etsi_web::standards::denm_2_1_1::denm_pdu_description::DENM>(DENM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_denm(&json, 211).unwrap();
    let decoded = decode(&encoded.to_vec(), Headers::GnBtp, EncodingRules::JER).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(expected_gn).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.geonetworking.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_denm_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":408,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<etsi_web::standards::denm_2_1_1::denm_pdu_description::DENM>(DENM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_denm(&json, 211).unwrap();
    let decoded = decode(&encoded.to_vec(), Headers::GnBtp, EncodingRules::JER).unwrap();
    // Ignore Geonetworking header, because it will get wrapped in an Unsecured header, and have the payload from the rest of the message (like in [`round_trip_wasm`])
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_cam_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":164,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<etsi_web::standards::cam_1_4_1::cam_pdu_descriptions::CAM>(CAM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_cam(&json, 141).unwrap();
    let decoded = decode(&encoded.to_vec(), Headers::GnBtp, EncodingRules::JER).unwrap();
    // Ignore Geonetworking header, because it will get wrapped in an Unsecured header, and have the payload from the rest of the message (like in [`round_trip_wasm`])
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_mapem_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":540,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<etsi_web::standards::is_1_3_1::etsi_schema::MAPEM>(MAPEM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_mapem(&json, 131).unwrap();
    let decoded = decode(&encoded.to_vec(), Headers::GnBtp, EncodingRules::JER).unwrap();
    // Ignore Geonetworking header, because it will get wrapped in an Unsecured header, and have the payload from the rest of the message (like in [`round_trip_wasm`])
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_spatem_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":207,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<etsi_web::standards::is_1_3_1::etsi_schema::SPATEM>(SPATEM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_spatem(&json, 131).unwrap();
    let decoded = decode(&encoded.to_vec(), Headers::GnBtp, EncodingRules::JER).unwrap();
    // Ignore Geonetworking header, because it will get wrapped in an Unsecured header, and have the payload from the rest of the message (like in [`round_trip_wasm`])
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_ivim_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":77,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<etsi_web::standards::is_1_3_1::etsi_schema::IVIM>(IVIM)
            .unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_ivim(&json, 221).unwrap();
    let decoded = decode(&encoded.to_vec(), Headers::GnBtp, EncodingRules::JER).unwrap();
    // Ignore Geonetworking header, because it will get wrapped in an Unsecured header, and have the payload from the rest of the message (like in [`round_trip_wasm`])
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap()
    );
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
    let decoded = decode(&encoded.to_vec(), Headers::GnBtp, EncodingRules::JER).unwrap();
    // Ignore Geonetworking header, because it will get wrapped in an Unsecured header, and have the payload from the rest of the message (like in [`round_trip_wasm`])
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn round_trip_cpm_wasm() {
    let json = ItsMessage {
        geonetworking: Some(r#"{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":80,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[false,false,false,false,false,false,false,false],"payload_length":628,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"Unknown","reserved":[false,true,false,false,false,false,false,true,true,false],"address":[0,96,224,105,87,141]},"timestamp":542947520,"latitude":535574568,"longitude":99765648,"position_accuracy":false,"speed":680,"heading":2122},"media_dependent_data":[127,0,184,0]}}}"#.into()),
        transport: Some(r#"{"destination_port":2001,"destination_port_info":0}"#.into()),
        its: Some(rasn::jer::encode(
            &rasn::uper::decode::<etsi_web::standards::is_1_3_1::etsi_schema::CPM>(CPM).unwrap()
        ).unwrap()),
        ..Default::default()
    };
    let encoded = encode_cpm(&json, 131).unwrap();
    let decoded = decode(&encoded.to_vec(), Headers::GnBtp, EncodingRules::JER).unwrap();
    // Ignore Geonetworking header, because it will get wrapped in an Unsecured header, and have the payload from the rest of the message (like in [`round_trip_wasm`])
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn decode_pcap_frame_wasm() {
    let exp_geonetworking = r#"{"Unsecured":{"basic":{"version":1,"next_header":"CommonHeader","reserved":[false,false,false,false,false,false,false,false],"lifetime":5,"remaining_hop_limit":1},"common":{"next_header":"BTPB","reserved_1":[false,false,false,false],"header_type_and_subtype":{"TopologicallyScopedBroadcast":"SingleHop"},"traffic_class":{"store_carry_forward":false,"channel_offload":false,"traffic_class_id":2},"flags":[true,false,false,false,false,false,false,false],"payload_length":67,"maximum_hop_limit":1,"reserved_2":[false,false,false,false,false,false,false,false]},"extended":{"SHB":{"source_position_vector":{"gn_address":{"manually_configured":false,"station_type":"PassengerCar","reserved":[false,false,false,false,false,false,false,false,false,false],"address":[138,176,248,168,162,37]},"timestamp":1151018751,"latitude":535505166,"longitude":99353789,"position_accuracy":true,"speed":14,"heading":724},"media_dependent_data":[0,0,0,0]}},"payload":[7,209,0,0,2,2,156,107,199,147,38,255,64,90,178,2,65,206,38,186,215,161,134,24,96,0,54,204,208,72,45,79,160,5,168,130,152,138,127,51,255,1,255,250,0,40,51,0,0,44,2,121,2,217,173,240,3,121,96,26,104,51,205,99,240,67,44]}}"#;
    let exp_tp = r#"{"destination_port":2001,"destination_port_info":0}"#;
    let exp_etsi = r#"{"header":{"protocolVersion":2,"messageID":2,"stationID":2624309139},"cam":{"generationDeltaTime":9983,"camParameters":{"basicContainer":{"stationType":5,"referencePosition":{"latitude":535505166,"longitude":99353789,"positionConfidenceEllipse":{"semiMajorConfidence":195,"semiMinorConfidence":195,"semiMajorOrientation":0},"altitude":{"altitudeValue":12230,"altitudeConfidence":"alt-005-00"}}},"highFrequencyContainer":{"basicVehicleContainerHighFrequency":{"heading":{"headingValue":724,"headingConfidence":126},"speed":{"speedValue":11,"speedConfidence":41},"driveDirection":"unavailable","vehicleLength":{"vehicleLengthValue":42,"vehicleLengthConfidenceIndication":"unavailable"},"vehicleWidth":18,"longitudinalAcceleration":{"longitudinalAccelerationValue":-1,"longitudinalAccelerationConfidence":102},"curvature":{"curvatureValue":0,"curvatureConfidence":"onePerMeter-0-00002"},"curvatureCalculationMode":"yawRateUsed","yawRate":{"yawRateValue":0,"yawRateConfidence":"unavailable"},"accelerationControl":"00","lateralAcceleration":{"lateralAccelerationValue":0,"lateralAccelerationConfidence":102}}},"lowFrequencyContainer":{"basicVehicleContainerLowFrequency":{"vehicleRole":"default","exteriorLights":"00","pathHistory":[{"pathPosition":{"deltaLatitude":317,"deltaLongitude":1460,"deltaAltitude":-940},"pathDeltaTime":1779},{"pathPosition":{"deltaLatitude":423,"deltaLongitude":3316,"deltaAltitude":-1310},"pathDeltaTime":4300}]}}}}}"#;
    let hex = "0000480002000040000004e5480038000200c900f40200000000000000000a014cff42ff34ff34ff7f390000f7c52687ebeb050091000c177c3d3c95fa000000000000000000000088000000ffffffffffff8ab0f8a8a225ffffffffffff00882300aaaa03000000894711000501205002800043010014008ab0f8a8a225449b26ff1feb290e05ec04bd800e02d40000000007d1000002029c6bc79326ff405ab20241ce26bad7a18618600036ccd0482d4fa005a882988a7f33ff01fffa00283300002c027902d9adf00379601a6833cd63f0432ce5dc898b";
    let raw = (0..hex.len())
        .step_by(2)
        .map(|s| u8::from_str_radix(&hex[s..s + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();
    let decoded = decode(&raw, Headers::RadioTap802LlcGnBtp, EncodingRules::JER).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&decoded.geonetworking.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(exp_geonetworking).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&decoded.its.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(exp_etsi).unwrap()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(exp_tp).unwrap()
    );
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
    let decoded = decode(&raw, Headers::RadioTap802LlcGnBtp, EncodingRules::UPER).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&expected.geonetworking.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.geonetworking.unwrap()).unwrap()
    );
    assert_eq!(expected.its, decoded.its);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&expected.transport.unwrap()).unwrap(),
        serde_json::from_str::<serde_json::Value>(&decoded.transport.unwrap()).unwrap()
    );
}

#[cfg(not(target_arch = "wasm32"))]
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
            serde_json::from_str::<serde_json::Value>(
                &String::from_utf8(
                    ItsMessage::Cam {
                        geonetworking: None,
                        transport: None,
                        etsi
                    }
                    .encode(EncodingRules::JER)
                    .unwrap()
                )
                .unwrap()
            )
            .unwrap(),
            serde_json::from_str::<serde_json::Value>(exp_etsi).unwrap()
        );
    } else {
        panic!("Failed decode_pcap_frame!");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn round_trip() {
    let messages = [DENM, CAM, MAPEM, SPATEM, IVIM, CPM];
    for message in messages {
        let uper_decoded = decode(message, Headers::None).unwrap();

        // Encode to all encoding rules
        let _uper_encoded = uper_decoded.clone().encode(EncodingRules::UPER).unwrap();
        let xer_encoded = uper_decoded.clone().encode(EncodingRules::XER).unwrap();
        let jer_encoded = uper_decoded.clone().encode(EncodingRules::JER).unwrap();

        // Decode from XER and JER (UPER done already)
        let xer_decoded = decode(&xer_encoded, Headers::None).unwrap();
        let jer_decoded = decode(&jer_encoded, Headers::None).unwrap();

        assert_eq!(uper_decoded, xer_decoded);
        assert_eq!(uper_decoded, jer_decoded);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn segment() {
    let expected = Element::parse(include_str!("files/segment.xml").as_bytes()).unwrap();
    let decoded = Segment::new(
        PolygonalLine::deltaPositions(DeltaPositions(vec![
            DeltaPosition::new(DeltaLatitude(937), DeltaLongitude(-917)),
            DeltaPosition::new(DeltaLatitude(933), DeltaLongitude(-458)),
            DeltaPosition::new(DeltaLatitude(1375), DeltaLongitude(-323)),
        ])),
        Some(IviLaneWidth(350)),
    );
    let re_encoded = Element::parse(rasn::xer::encode(&decoded).unwrap().as_slice()).unwrap();
    assert_eq!(expected, re_encoded);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ivim_xer_impl() {
    let expected = Element::parse(include_str!("files/ivim_xer_impl_ivi.xml").as_bytes()).unwrap();
    let decoded = decode(
        &[
            0x02, 0x06, 0x00, 0x12, 0x10, 0xdc, 0x82, 0x50, 0x00, 0x00, 0x00, 0x00, 0x88, 0x05,
            0x58, 0xea, 0xad, 0x57, 0x13, 0xd7, 0xa6, 0x4f, 0xff, 0xff, 0xfe, 0x11, 0xdb, 0xba,
            0x1f, 0x08, 0xc0, 0xe1, 0x11, 0x01, 0x40, 0x75, 0x0f, 0xe3, 0x54, 0x07, 0x48, 0xff,
            0x1a, 0xc0, 0xab, 0xcf, 0xf5, 0xe2, 0xbc, 0x30, 0x78, 0x44, 0x40, 0x2f, 0xff, 0x24,
            0x11, 0x2f, 0x01, 0x29, 0x45, 0x70, 0xea, 0xf0, 0x81, 0x60, 0x00, 0x02, 0x00, 0x04,
            0x08, 0x01, 0x4e,
        ],
        Headers::None,
    )
    .unwrap();
    let re_encoded =
        Element::parse(decoded.encode(EncodingRules::XER).unwrap().as_slice()).unwrap();
    pretty_assertions::assert_eq!(expected, re_encoded);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn xer_to_xer() {
    let expected_bytes = include_str!("files/xer_to_xer_ivi.xml").as_bytes();
    let expected_xml = Element::parse(expected_bytes).unwrap();
    let decoded = decode(expected_bytes, Headers::None).unwrap();
    let re_encoded =
        Element::parse(decoded.encode(EncodingRules::XER).unwrap().as_slice()).unwrap();
    assert_eq!(expected_xml, re_encoded);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn xer_to_uper() {
    let expected = &[
        0x01, 0x06, 0x00, 0x00, 0x00, 0x00, 0xba, 0x50, 0x00, 0xc8, 0x00, 0x01, 0x90, 0x15, 0xbd,
        0xce, 0x3a, 0x64, 0x05, 0x2a, 0xc9, 0x55, 0xd9, 0x01, 0x5b, 0xdc, 0xdd, 0x94, 0x20, 0x15,
        0x0d, 0x3f, 0xd0, 0x5c, 0x3b, 0xc1, 0x8b, 0x40, 0x04, 0x00, 0x40, 0x03, 0x6e, 0xe8, 0x40,
        0x41, 0x00, 0x02, 0x57, 0xfc, 0x06, 0x63, 0x15, 0x92, 0xa3, 0xe9, 0x3f, 0x1a, 0x72, 0xde,
        0xed, 0xe9, 0xae, 0x9d, 0xf4, 0x03, 0xba, 0xbc, 0x4f, 0x70, 0x29, 0xa1, 0x11, 0x77, 0xf8,
        0xb9, 0x8e, 0x97, 0x52, 0x33, 0x95, 0x38, 0x71, 0x6b, 0xd9, 0x52, 0xb6, 0xdf, 0xc3, 0x99,
        0xae, 0x69, 0xbb, 0xd9, 0xdf, 0xde, 0x80, 0x07, 0xa5, 0x41, 0x66, 0x6d, 0x7a, 0xbd, 0x76,
        0x5e, 0x5f, 0xb2, 0x66, 0xe6, 0x00, 0x9c, 0x0d, 0x36, 0x66, 0xed, 0xcd, 0x64, 0x65, 0xd7,
        0x1d, 0xde, 0x3e, 0x3d, 0xa7, 0xed, 0xca, 0x62, 0xcc, 0x1f, 0xf7, 0x36, 0x30, 0xb6, 0x10,
        0x40, 0x18, 0xff, 0xff, 0xbf, 0xff, 0xed, 0xfc, 0x89, 0xa5, 0xa8, 0xe3, 0xbd, 0x9d, 0x25,
        0x8e, 0x77, 0x0a, 0x03, 0xd2, 0xe9, 0x59, 0x22, 0x17, 0xee, 0xb3, 0x42, 0x41, 0xfc, 0xec,
        0xfb, 0xa6, 0x42, 0xce, 0xfb, 0xb2, 0x9e, 0xa4, 0xf4, 0x6c, 0x30, 0x9a, 0x6f, 0x86, 0x3b,
        0x65, 0x4a, 0xfb, 0x76, 0x3a, 0xa9, 0x8f, 0xdd, 0x43, 0xda, 0x89, 0x7f, 0xc0, 0x66, 0x31,
        0x41, 0x08, 0x00, 0xfa, 0x3a, 0xaf, 0x8f, 0x45, 0xa7, 0x2d, 0xf9, 0x85, 0xba, 0xc8, 0x8f,
        0x9f, 0xbd, 0xaf, 0xe5, 0xfa, 0x59, 0xdb, 0x3b, 0x1f, 0xb0, 0x9d, 0xb7, 0x50, 0x7b, 0xcb,
        0xbb, 0xca, 0xcf, 0xd9, 0x5d, 0xbf, 0xb3, 0xfe, 0xc8, 0xe2, 0x15, 0x84, 0x20, 0x08, 0x00,
        0x20, 0x02, 0xa2, 0x22, 0x82, 0x64, 0x04, 0x4a, 0x10, 0x10, 0x4a, 0x8c, 0xae, 0x6e, 0x84,
        0x05, 0xe4, 0x0a, 0x6e, 0x8e, 0x4c, 0x2e, 0x8c, 0xac, 0xed, 0x2c, 0xa4, 0x09, 0xce, 0x45,
        0xc4, 0x08, 0x25, 0xa6, 0x64, 0x18, 0x73, 0x8c, 0x4c, 0xae, 0x4d, 0x8c, 0x2e, 0x6e, 0x8e,
        0xad, 0xcc, 0xe4, 0x0c, 0x8c, 0xae, 0x44, 0x08, 0x48, 0x28, 0x44, 0x06, 0x87, 0x24, 0x08,
        0x2a, 0x61, 0x44, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
        0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
        0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x09, 0x6a, 0x65, 0xaa, 0xec, 0x2d, 0x8c,
        0x8c, 0x2e, 0xa1, 0x44, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
        0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
        0x04, 0x04, 0x04, 0x04, 0x04, 0x03, 0x08, 0x00, 0x21, 0x08, 0x15, 0x11, 0x14, 0x13, 0x1e,
    ];
    // This string can't be in one line, because of new lines in the free text
    let decoded = decode(
        include_str!("files/xer_to_uper_ivi.xml").as_bytes(),
        Headers::None,
    )
    .unwrap();
    let re_encoded = decoded.encode(EncodingRules::UPER).unwrap();

    assert_eq!(expected, re_encoded.as_slice())
}
