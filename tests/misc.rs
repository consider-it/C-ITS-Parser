#[cfg(all(not(target_arch = "wasm32"), feature = "spatem_2_2_1"))]
// protbuf:
// TAG: xyyy.yzzz: y = field num, z = wire type
// LENGTH: as VARINT (0x2d = 45 here)
const ALBERDING_SPAT_PROTO: &[u8] = &[
    0x0a, 0x2d, 0x01, 0x02, 0x12, 0x09, 0x5b, 0x72, 0xf3, 0x70, 0x40, 0x5a, 0x95, 0x2f, 0xbb, 0xcd,
    0xbe, 0xdc, 0xc8, 0xdf, 0xff, 0xff, 0xff, 0xc2, 0x22, 0xe8, 0x75, 0x80, 0x00, 0x00, 0xfc, 0x02,
    0xf7, 0xd8, 0x2c, 0x08, 0x50, 0x73, 0x75, 0x30, 0xf5, 0xff, 0xfb, 0x00, 0x00, 0x00, 0x00, 0x10,
    0x0a, 0x18, 0xc8, 0x01, 0x20, 0x07,
];

#[cfg(all(not(target_arch = "wasm32"), feature = "spatem_2_2_1"))]
#[test]
fn decode_customer_message() {
    let alberding_spat = &ALBERDING_SPAT_PROTO[2..48];

    let header = rasn::Codec::Uper
        .decode_from_binary::<c_its_parser::standards::cdd_2_2_1::etsi_its_cdd::ItsPduHeader>(
            alberding_spat,
        );
    println!("{header:?}");

    // let message = rasn::Codec::Uper.decode_from_binary::<c_its_parser::standards::spatem_2_2_1::spatem_pdu_descriptions::SPATEM>(alberding_spat);
    let message = rasn::Codec::Uper
        .decode_from_binary::<c_its_parser::standards::dsrc_2_2_1::etsi_its_dsrc::SPAT>(
            alberding_spat,
        );

    println!("{message:#?}");

    assert!(message.is_ok());
}
