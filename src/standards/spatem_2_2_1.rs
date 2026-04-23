#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod spatem_pdu_descriptions {
    extern crate alloc;
    use core::borrow::Borrow;

    use rasn::prelude::*;

    use super::super::cdd_2_2_1::etsi_its_cdd::ItsPduHeader;
    use super::super::dsrc_2_2_1::etsi_its_dsrc::*;

    #[doc = "Signal phase and timing extended Message"]
    #[doc = "Signal phase and timing extended Message Root"]
    #[doc = "This DF includes DEs for the SPATEM: protocolVersion, the SPATEM message type identifier `messageID`,"]
    #[doc = "the station identifier `stationID` of the originating ITS-S and the SPaT data from ETSI-ITS-DSRC module."]
    #[doc = "\n@field header:  The DE `protocolVersion` used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "                It shall be set to 2."]
    #[doc = "                The DE `messageID` shall be spatem(4)."]
    #[doc = "\n@field spat:    contains the SPaT data as defined in ETSI-ITS-DSRC."]
    #[doc = "\n@category: Basic Information"]
    #[doc = "\n@revision: V1.3.1"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SPATEM {
        pub header: ItsPduHeader,
        pub spat: SPAT,
    }
    impl SPATEM {
        pub fn new(header: ItsPduHeader, spat: SPAT) -> Self {
            Self { header, spat }
        }
    }
}
