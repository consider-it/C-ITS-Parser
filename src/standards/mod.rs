//! C-ITS ASN.1 Message Definitions and Utilities
//!
//! Note: Most modules are auto-generated code from the rasn-compiler.

#[cfg(feature = "_cdd_1_3_1_1")]
/// CDD v1.3.1
pub mod cdd_1_3_1_1;
#[cfg(feature = "cpm_1")]
/// ETSI TR 103 562 v2.1.1 CPM
pub mod cpm_1;
#[cfg(feature = "denm_1_3_1")]
/// ETSI EN 302 637-3 v1.3.1 DENM
pub mod denm_1_3_1;
#[cfg(feature = "ivim_2_1_1")]
/// ETSI TS 103 301 v2.1.1 IVIM (compatible with v1.3.1)
pub mod ivim_2_1_1;

#[cfg(feature = "cam_1_4_1")]
/// ETSI EN 302 637-2 v1.4.1 CAM
pub mod cam_1_4_1;
#[cfg(any(feature = "_cdd_2_2_1", feature = "etsi"))]
/// CDD v2.2.1
pub mod cdd_2_2_1;
#[cfg(feature = "cpm_2_1_1")]
/// ETSI TS 103 324 v2.1.1 CPM
pub mod cpm_2_1_1;
#[cfg(feature = "denm_2_2_1")]
/// ETSI TS 103 831 v2.2.1 DENM (compatible with v2.1.1)
pub mod denm_2_2_1;
#[cfg(feature = "_dsrc_2_2_1")]
/// DSRC v2.2.1
pub mod dsrc_2_2_1;
#[cfg(feature = "ivim_2_2_1")]
/// ETSI TS 103 301 v2.2.1 IVIM
pub mod ivim_2_2_1;
#[cfg(feature = "mapem_2_2_1")]
/// ETSI TS 103 301 v2.2.1 MAPEM (compatible with v1.3.1 and v2.1.1)
pub mod mapem_2_2_1;
#[cfg(feature = "spatem_2_2_1")]
/// ETSI TS 103 301 v2.2.1 SPATEM (compatible with v1.3.1 and v2.1.1)
pub mod spatem_2_2_1;
#[cfg(feature = "srem_2_2_1")]
/// ETSI TS 103 301 v2.2.1 SREM (compatible with v1.3.1 and v2.1.1)
pub mod srem_2_2_1;
#[cfg(feature = "ssem_2_2_1")]
/// ETSI TS 103 301 v2.2.1 SSEM (compatible with v1.3.1 and v2.1.1)
pub mod ssem_2_2_1;

pub mod extensions;

pub mod conversions;
