#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod ssem_pdu_descriptions {
    extern crate alloc;
    use core::borrow::Borrow;

    use rasn::prelude::*;

    use super::super::cdd_2_2_1::etsi_its_cdd::ItsPduHeader;
    use super::super::dsrc_2_2_1::etsi_its_dsrc::SignalStatusMessage;

    #[doc = "Signal status extended Message"]
    #[doc = "This DF includes DEs for the SSEM: protocolVersion, the SSEM message type identifier `messageID` and"]
    #[doc = "the station identifier `stationID` of the originating ITS-S and the signal status data from ETSI-ITS-DSRC."]
    #[doc = "- @field header: The DE `protocolVersion` is used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "               It shall be set to 2."]
    #[doc = "               The DE `messageID` shall be ssem(10)."]
    #[doc = "- @field ssm:    contains the Signal status data as defined in ETSI-ITS-DSRC."]
    #[doc = "\n\n@category: Basic Information"]
    #[doc = "\n\n@revision: V1.3.1"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SSEM {
        pub header: ItsPduHeader,
        pub ssm: SignalStatusMessage,
    }
    impl SSEM {
        pub fn new(header: ItsPduHeader, ssm: SignalStatusMessage) -> Self {
            Self { header, ssm }
        }
    }
}
