#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod mapem_pdu_descriptions {
    extern crate alloc;
    use super::super::cdd_2_2_1::etsi_its_cdd::ItsPduHeader;
    use super::super::dsrc_2_2_1::etsi_its_dsrc::MapData;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = "*"]
    #[doc = "* Map (lane topology) extended Message"]
    #[doc = "* This DF includes DEs for the MAPEM: protocolVersion, the MAPEM message type identifier `messageID`, "]
    #[doc = "* the station identifier `stationID` of the originating ITS-S and the Map data from ETSI-ITS-DSRC."]
    #[doc = "* "]
    #[doc = "* @field header:  The DE `protocolVersion` is used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "*                 It shall be set to 2."]
    #[doc = "*                 The DE `messageID` shall be mapem(5)."]
    #[doc = "* @field map:     contains the MAP data as defined in ETSI-ITS-DSRC."]
    #[doc = "* "]
    #[doc = "* @category: Basic Information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MAPEM {
        pub header: ItsPduHeader,
        pub map: MapData,
    }
    impl MAPEM {
        pub fn new(header: ItsPduHeader, map: MapData) -> Self {
            Self { header, map }
        }
    }
}
