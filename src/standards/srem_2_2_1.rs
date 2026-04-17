#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod srem_pdu_descriptions {
    extern crate alloc;
    use super::super::cdd_2_2_1::etsi_its_cdd::ItsPduHeader;
    use super::super::dsrc_2_2_1::etsi_its_dsrc::SignalRequestMessage;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;

    #[doc = "Signal request extended Message Message"]
    #[doc = "This DF includes DEs for the SREM: protocolVersion, the SREM message type identifier `messageID`,"]
    #[doc = "the station identifier `stationID` of the originating ITS-S and the signal request data from ETSI-ITS-DSRC."]
    #[doc = "\n@field header: The DE `protocolVersion` is used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "               It shall be set to 2."]
    #[doc = "               The DE `messageID` shall be srem(9)."]
    #[doc = "\n@field srm:    contains the Signal request data as defined in ETSI-ITS-DSRC."]
    #[doc = "\n@category: Basic Information"]
    #[doc = "\n@revision: V1.3.1"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SREM {
        pub header: ItsPduHeader,
        pub srm: SignalRequestMessage,
    }
    impl SREM {
        pub fn new(header: ItsPduHeader, srm: SignalRequestMessage) -> Self {
            Self { header, srm }
        }
    }
}
