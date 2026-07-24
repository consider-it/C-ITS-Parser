// Copyright (c) 2025 consider it GmbH

//! Conversions between ETSI ASN.1 values and common (SI) units
//!
//! Take a look at the individual data types in [`crate::standards`] to discover available conversion methods and initialization functions.

pub const MPS_TO_KMH_FACTOR: f32 = 3.6;

#[cfg(feature = "_cdd_1_3_1_1")]
use crate::standards::cdd_1_3_1_1;
#[cfg(feature = "_cdd_2_2_1")]
use crate::standards::cdd_2_2_1;
#[cfg(feature = "cpm_1")]
use crate::standards::cpm_1;
#[cfg(feature = "_dsrc_2_2_1")]
use crate::standards::dsrc_2_2_1;

/// Create conversions for ETSI type `t` and some "unavailable" value
macro_rules! latlon_to_deg {
    ($t:ty, $unavailable:expr) => {
        impl $t {
            /// convert ETSI Latitude/ Longitude to degrees
            #[must_use]
            pub fn as_deg(&self) -> f64 {
                f64::from(self.0) / 10_000_000.
            }

            /// convert ETSI Latitude/ Longitude to degrees or `None` if "unavailable"
            #[must_use]
            pub fn try_as_deg(&self) -> Option<f64> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_deg())
                }
            }

            /// convert ETSI Latitude/ Longitude to degrees
            #[must_use]
            pub fn from_deg(other: f64) -> Self {
                Self((other * 10_000_000.) as i32)
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }
    };
}

#[cfg(feature = "_cdd_1_3_1_1")]
latlon_to_deg!(cdd_1_3_1_1::its_container::Longitude, 1_800_000_001);
#[cfg(feature = "_cdd_1_3_1_1")]
latlon_to_deg!(cdd_1_3_1_1::its_container::Latitude, 900_000_001);
#[cfg(feature = "_cdd_2_2_1")]
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::Longitude, 1_800_000_001);
#[cfg(feature = "_cdd_2_2_1")]
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::Latitude, 900_000_001);

#[cfg(feature = "_cdd_1_3_1_1")]
latlon_to_deg!(cdd_1_3_1_1::its_container::DeltaLongitude, 131_072);
#[cfg(feature = "_cdd_1_3_1_1")]
latlon_to_deg!(cdd_1_3_1_1::its_container::DeltaLatitude, 131_072);
#[cfg(feature = "_cdd_2_2_1")]
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::DeltaLongitude, 131_072);
#[cfg(feature = "_cdd_2_2_1")]
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::DeltaLatitude, 131_072);

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv`
#[cfg(feature = "cpm_1")]
macro_rules! etsi_to_meters {
    ($t:ty, $tt:ty, $conv:expr) => {
        impl $t {
            /// convert ETSI data to meters
            #[must_use]
            pub fn as_meters(&self) -> f32 {
                self.0 as f32 / $conv
            }

            /// create ETSI data from meters
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_meters(value: f32) -> Result<Self, alloc::string::String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(alloc::format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_meters()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_meters()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = alloc::string::String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_meters(value)
            }
        }
    };
}

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv` and some "unavailable" value
#[cfg(any(
    feature = "_cdd_1_3_1_1",
    feature = "_cdd_2_2_1",
    feature = "_dsrc_2_2_1",
    feature = "cpm_1"
))]
macro_rules! etsi_to_meters_unavailable {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI data to meters
            #[must_use]
            pub fn as_meters(&self) -> f32 {
                self.0 as f32 / $conv
            }

            /// convert ETSI data to meters or `None` if "unavailable"
            #[must_use]
            pub fn try_as_meters(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_meters())
                }
            }

            /// create ETSI data from meters
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_meters(value: f32) -> Result<Self, alloc::string::String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(alloc::format!("Value out of bounds"));
                    }
                }

                // Not all "unavailable" values are positive, but always at the very edge of the allowed value range.
                // So by checking for constraints first, we can use a strict equals condition.
                if etsi_val == $unavailable {
                    return Err(alloc::format!("Value out of bounds"));
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_meters()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_meters()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = alloc::string::String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_meters(value)
            }
        }
    };
}

#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters_unavailable!(dsrc_2_2_1::etsi_its_dsrc::OffsetB09, i16, 100., -256);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters_unavailable!(dsrc_2_2_1::etsi_its_dsrc::OffsetB10, i16, 100., -512);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters_unavailable!(dsrc_2_2_1::etsi_its_dsrc::OffsetB11, i16, 100., -1024);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters_unavailable!(dsrc_2_2_1::etsi_its_dsrc::OffsetB12, i16, 100., -2048);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters_unavailable!(dsrc_2_2_1::etsi_its_dsrc::OffsetB13, i16, 100., -4096);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters_unavailable!(dsrc_2_2_1::etsi_its_dsrc::OffsetB14, i16, 100., -8192);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters_unavailable!(dsrc_2_2_1::etsi_its_dsrc::OffsetB16, i16, 100., -32768);

#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::DistanceValue, i32, 100.);
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue, u16, 10., 256);
#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::ObjectDimensionValue, u16, 10.);
#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::Radius, u16, 10.);
#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::Range, u16, 10.);
#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::SemiRangeLength, u16, 10.);

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::VehicleWidth, u8, 10., 62); // Unit: 0,1 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::VehicleWidth, u8, 10., 62); // Unit: 0,1 metre
#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(
    cdd_1_3_1_1::its_container::VehicleLengthValue,
    u16,
    10.,
    1023
); // Unit: 0,1 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::VehicleLengthValue, u16, 10., 1023); // Unit: 0,1 metre

#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters_unavailable!(dsrc_2_2_1::etsi_its_dsrc::VehicleHeight, u8, 20., 127); // Unit: 0,05 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::VehicleHeight, u8, 20., 127); // Unit: 0,05 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::SemiAxisLength, u16, 100., 4095); // Unit: 0,01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::SemiAxisLength, u16, 100., 4095); // Unit: 0,01 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::AltitudeValue, i32, 100., 800001); // Unit: 0,01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::AltitudeValue, i32, 100., 800001); // Unit: 0,01 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::DeltaAltitude, i16, 100., 12800); // Unit: 0,01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::DeltaAltitude, i16, 100., 12800); // Unit: 0,01 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::HeightLonCarr, u8, 100., 100); // Unit: 0,01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::HeightLonCarr, u8, 100., 100); // Unit: 0,01 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::PosLonCarr, u8, 100., 127); // Unit: 0,01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::PosLonCarr, u8, 100., 127); // Unit: 0,01 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::PosFrontAx, u8, 100., 20); // Unit: 0,01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::PosFrontAx, u8, 100., 20); // Unit: 0,01 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::PosPillar, u8, 100., 30); // Unit: 0,01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::PosPillar, u8, 100., 30); // Unit: 0,01 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::WheelBaseVehicle, u8, 100., 127); // Unit: 0,01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::WheelBaseVehicle, u8, 100., 127); // Unit: 0,01 metre

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::TurningRadius, u8, 2.5, 255); // Unit: 0,4 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::TurningRadius, u8, 2.5, 255); // Unit: 0,4 metre

#[cfg(feature = "_cdd_2_2_1")] // CPM v2
etsi_to_meters_unavailable!(
    cdd_2_2_1::etsi_its_cdd::CoordinateConfidence,
    u16,
    100.,
    4096
); // Unit: 0,01 metre

#[cfg(feature = "cpm_1")]
etsi_to_meters!(
    cpm_1::cpm_pdu_descriptions::LongitudinalLanePositionValue,
    u16,
    10.
); // Unit: 0,1 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(
    cdd_2_2_1::etsi_its_cdd::LongitudinalLanePositionValue,
    u16,
    10.,
    32767
); // Unit: 0,1 metre

#[cfg(feature = "cpm_1")]
etsi_to_meters_unavailable!(
    cpm_1::cpm_pdu_descriptions::LongitudinalLanePositionConfidence,
    u8,
    100.,
    102
); // Unit: 0.01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(
    cdd_2_2_1::etsi_its_cdd::LongitudinalLanePositionConfidence,
    u16,
    10.,
    1023
); // Unit: 0,1 metre

#[cfg(feature = "cpm_1")]
etsi_to_meters_unavailable!(
    cpm_1::cpm_pdu_descriptions::ObjectDimensionConfidence,
    u8,
    100.,
    102
); // Unit: 0.01 metre
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(
    cdd_2_2_1::etsi_its_cdd::ObjectDimensionConfidence,
    u8,
    10.,
    32
); // Unit: 0,1 metre

#[cfg(feature = "_cdd_2_2_1")] // DENM v2
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::Position1d, i16, 1., 8191); // Unit: 1 metre

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv` and some "unavailable" value
macro_rules! etsi_to_mps {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI speed to m/s
            #[must_use]
            pub fn as_mps(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// convert ETSI speed to m/s or `None` if "unavailable"
            #[must_use]
            pub fn try_as_mps(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_mps())
                }
            }

            /// create ETSI speed from m/s
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_mps(value: f32) -> Result<Self, alloc::string::String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(alloc::format!("Value out of bounds"));
                    }
                }

                // Not all "unavailable" values are positive, but always at the very edge of the allowed value range.
                // So by checking for constraints first, we can use a strict equals condition.
                if etsi_val == $unavailable {
                    return Err(alloc::format!("Value out of bounds"));
                }

                Ok(Self(etsi_val))
            }

            /// convert ETSI speed to km/h
            #[must_use]
            pub fn as_kmh(&self) -> f32 {
                self.as_mps() * MPS_TO_KMH_FACTOR
            }

            /// convert ETSI speed to km/h or `None` if "unavailable"
            #[must_use]
            pub fn try_as_kmh(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_kmh())
                }
            }

            /// create ETSI speed from km/h
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_kmh(value: f32) -> Result<Self, alloc::string::String> {
                Self::from_mps(value / MPS_TO_KMH_FACTOR)
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_mps()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_mps()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = alloc::string::String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_mps(value)
            }
        }
    };
}

#[cfg(feature = "cpm_1")]
etsi_to_mps!(
    cpm_1::cpm_pdu_descriptions::SpeedValueExtended,
    i16,
    100.,
    16_383
); // Unit: 0,01 m/s
#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mps!(cdd_1_3_1_1::its_container::SpeedValue, u16, 100., 16_383); // Unit: 0,01 m/s
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mps!(cdd_2_2_1::etsi_its_cdd::SpeedValue, u16, 100., 16_383); // Unit: 0,01 m/s

#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mps!(
    cdd_2_2_1::etsi_its_cdd::VelocityComponentValue,
    i16,
    100.,
    16_383
); // Unit: 0,01 m/s

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mps!(cdd_1_3_1_1::its_container::SpeedConfidence, u8, 100., 127); // Unit: 0,01 m/s
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mps!(cdd_2_2_1::etsi_its_cdd::SpeedConfidence, u8, 100., 127); // Unit: 0,01 m/s

#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_mps!(dsrc_2_2_1::etsi_its_dsrc::Velocity, u16, 50., 8191); // Unit: 0.02 m/s

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv` and some "unavailable" value
#[cfg(any(feature = "_cdd_1_3_1_1", feature = "_cdd_2_2_1"))]
macro_rules! etsi_to_mpss {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI acceleration to m/s/s
            #[must_use]
            pub fn as_mpss(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// convert ETSI acceleration to m/s/s or `None` if "unavailable"
            #[must_use]
            pub fn try_as_mpss(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_mpss())
                }
            }

            /// create ETSI acceleration from m/s/s
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_mpss(value: f32) -> Result<Self, alloc::string::String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(alloc::format!("Value out of bounds"));
                    }
                }

                // Not all "unavailable" values are positive, but always at the very edge of the allowed value range.
                // So by checking for constraints first, we can use a strict equals condition.
                if etsi_val == $unavailable {
                    return Err(alloc::format!("Value out of bounds"));
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_mpss()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_mpss()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = alloc::string::String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_mpss(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mpss!(
    cdd_1_3_1_1::its_container::LongitudinalAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(
    cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mpss!(
    cdd_1_3_1_1::its_container::LateralAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(
    cdd_2_2_1::etsi_its_cdd::LateralAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(cdd_2_2_1::etsi_its_cdd::AccelerationValue, i16, 10., 161); // Unit: 0,1 m/s^2

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mpss!(
    cdd_1_3_1_1::its_container::VerticalAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(
    cdd_2_2_1::etsi_its_cdd::VerticalAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2

#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(
    cdd_2_2_1::etsi_its_cdd::AccelerationMagnitudeValue,
    u8,
    10.,
    161
); // Unit: 0,1 m/s^2

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mpss!(
    cdd_1_3_1_1::its_container::AccelerationConfidence,
    u8,
    10.,
    102
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(
    cdd_2_2_1::etsi_its_cdd::AccelerationConfidence,
    u8,
    10.,
    102
); // Unit: 0,1 m/s^2

/// Check for unavailable data of ETSI type `t` (which has underlying data type `tt`)
#[cfg(any(feature = "_cdd_1_3_1_1", feature = "_cdd_2_2_1"))]
macro_rules! etsi_raw_unavailable {
    ($t:ty, $tt:ty, $unavailable:expr) => {
        impl $t {
            /// convert ETSI acceleration to m/s/s or `None` if "unavailable"
            #[must_use]
            pub fn try_as_raw(&self) -> Option<$tt> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.0)
                }
            }

            /// create ETSI acceleration from raw value
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_raw(value: $tt) -> Result<Self, alloc::string::String> {
                use rasn::AsnType;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&value) {
                        return Err(alloc::format!("Value out of bounds"));
                    }
                }

                // Not all "unavailable" values are positive, but always at the very edge of the allowed value range.
                // So by checking for constraints first, we can use a strict equals condition.
                if value == $unavailable {
                    return Err(alloc::format!("Value out of bounds"));
                }

                Ok(Self(value))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for $tt {
            fn from(other: &$t) -> $tt {
                other.0
            }
        }
        impl From<$t> for $tt {
            fn from(other: $t) -> $tt {
                other.0
            }
        }

        impl TryFrom<$tt> for $t {
            type Error = alloc::string::String;

            fn try_from(value: $tt) -> Result<Self, Self::Error> {
                Self::from_raw(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_raw_unavailable!(cdd_1_3_1_1::its_container::CurvatureValue, i16, 1023);
#[cfg(feature = "_cdd_2_2_1")]
etsi_raw_unavailable!(cdd_2_2_1::etsi_its_cdd::CurvatureValue, i16, 1023);

#[cfg(feature = "_cdd_2_2_1")]
etsi_raw_unavailable!(cdd_2_2_1::etsi_its_cdd::ConfidenceLevel, u8, 101); // Unit: percent
#[cfg(feature = "_cdd_2_2_1")]
etsi_raw_unavailable!(cdd_2_2_1::etsi_its_cdd::CorrelationCellValue, i8, 101); // Unit: the value is scaled by 100
#[cfg(feature = "_cdd_2_2_1")]
etsi_raw_unavailable!(cdd_2_2_1::etsi_its_cdd::NumberOfOccupants, u8, 127); // Unit: 1 person

#[cfg(feature = "_cdd_2_2_1")]
etsi_raw_unavailable!(cdd_2_2_1::etsi_its_cdd::StabilityLossProbability, u8, 63); // Unit: 2 %
#[cfg(feature = "_cdd_2_2_1")]
etsi_raw_unavailable!(
    cdd_2_2_1::etsi_its_cdd::TrajectoryInterceptionProbability,
    u8,
    63
); // Unit: 2 %

/// Create conversions for ETSI type `t` with conversion factor `conv` and some "unavailable" value
#[cfg(any(
    feature = "cpm_1",
    feature = "_cdd_1_3_1_1",
    feature = "_cdd_2_2_1",
    feature = "_dsrc_2_2_1"
))]
macro_rules! angle_to_deg {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI WGS84AngleValue/ CartesianAngleValue to degrees
            #[must_use]
            pub fn as_deg(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// convert ETSI WGS84AngleValue/ CartesianAngleValue to degrees or `None` if "unavailable"
            #[must_use]
            pub fn try_as_deg(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_deg())
                }
            }

            /// create ETSI WGS84AngleValue/ CartesianAngleValue from degrees
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_deg(value: f32) -> Result<Self, alloc::string::String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(alloc::format!("Value out of bounds"));
                    }
                }

                // Not all "unavailable" values are positive, but always at the very edge of the allowed value range.
                // So by checking for constraints first, we can use a strict equals condition.
                if etsi_val == $unavailable {
                    return Err(alloc::format!("Value out of bounds"));
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_deg()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_deg()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = alloc::string::String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_deg(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(cdd_2_2_1::etsi_its_cdd::CartesianAngleValue, u16, 10., 3601); // Unit: 0,1 degrees
#[cfg(feature = "cpm_1")]
angle_to_deg!(
    cpm_1::cpm_pdu_descriptions::CartesianAngleValue,
    u16,
    10.,
    3601
); // Unit: 0,1 degrees
#[cfg(feature = "cpm_1")]
angle_to_deg!(cpm_1::cpm_pdu_descriptions::WGS84AngleValue, u16, 10., 3601); // Unit: 0,1 degrees
#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(cdd_2_2_1::etsi_its_cdd::Wgs84AngleValue, u16, 10., 3601); // Unit: 0,1 degrees
#[cfg(feature = "_dsrc_2_2_1")]
angle_to_deg!(dsrc_2_2_1::etsi_its_dsrc::Angle, u16, 80., 28800); // Unit: 0.0125 degrees
#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(cdd_2_2_1::etsi_its_cdd::HeadingValue, u16, 10., 3601); // Unit: 0,1 degree
#[cfg(feature = "_cdd_1_3_1_1")]
angle_to_deg!(cdd_1_3_1_1::its_container::HeadingValue, u16, 10., 3601); // Unit: 0,1 degree
#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(cdd_2_2_1::etsi_its_cdd::HeadingConfidence, u8, 10., 127); // Unit: 0,1 degree
#[cfg(feature = "_cdd_1_3_1_1")]
angle_to_deg!(cdd_1_3_1_1::its_container::HeadingConfidence, u8, 10., 127); // Unit: 0,1 degree

#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(cdd_2_2_1::etsi_its_cdd::AngleConfidence, u8, 10., 127); // Unit: 0,1 degrees
#[cfg(feature = "cpm_1")]
angle_to_deg!(cpm_1::cpm_pdu_descriptions::AngleConfidence, u8, 10., 127); // Unit: 0,1 degrees

#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(
    cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue,
    i16,
    (1. / 1.5),
    512
); // Unit: 1,5 degree
#[cfg(feature = "_cdd_1_3_1_1")]
angle_to_deg!(
    cdd_1_3_1_1::its_container::SteeringWheelAngleValue,
    i16,
    (1. / 1.5),
    512
); // Unit: 1,5 degree

#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(
    cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleConfidence,
    u8,
    (1. / 1.5),
    127
); // Unit: 1,5 degree
#[cfg(feature = "_cdd_1_3_1_1")]
angle_to_deg!(
    cdd_1_3_1_1::its_container::SteeringWheelAngleConfidence,
    u8,
    (1. / 1.5),
    127
); // Unit: 1,5 degree

/// Create conversions for ETSI type `t` with conversion factor `conv` and some "unavailable" value
#[cfg(any(feature = "_cdd_2_2_1", feature = "_cdd_1_3_1_1"))]
macro_rules! angle_to_degrate {
    ($t:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI YawRateValue to degrees per second
            #[must_use]
            pub fn as_deg_rate(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// convert ETSI YawRateValue to degrees per second or `None` if "unavailable"
            #[must_use]
            pub fn try_as_deg_rate(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_deg_rate())
                }
            }

            /// create ETSI YawRateValue from degrees per second
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_deg_rate(value: f32) -> Result<Self, alloc::string::String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as i16;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(alloc::format!("Value out of bounds"));
                    }
                }

                // Not all "unavailable" values are positive, but always at the very edge of the allowed value range.
                // So by checking for constraints first, we can use a strict equals condition.
                if etsi_val == $unavailable {
                    return Err(alloc::format!("Value out of bounds"));
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_deg_rate()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_deg_rate()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = alloc::string::String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_deg_rate(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_2_2_1")]
angle_to_degrate!(cdd_2_2_1::etsi_its_cdd::YawRateValue, 100., 32767); // Unit: 0,01 degree per second
#[cfg(feature = "_cdd_1_3_1_1")]
angle_to_degrate!(cdd_1_3_1_1::its_container::YawRateValue, 100., 32767); // Unit: 0,01 degree per second

#[cfg(feature = "_cdd_2_2_1")]
angle_to_degrate!(
    cdd_2_2_1::etsi_its_cdd::CartesianAngularVelocityComponentValue,
    1.,
    256
); // Unit: degree/s

// DeltaTime: unit 10 seconds, clamping to -121 for <-20 minutes and +120 for >+20 minutes, -122 for unavailable
#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::DeltaTime {
    /// convert ETSI DeltaTime to seconds
    #[must_use]
    pub fn as_sec(&self) -> i16 {
        i16::from(self.0) * 10
    }

    /// convert ETSI DeltaTime to seconds or `None` if "unavailable"
    #[must_use]
    pub fn try_as_sec(&self) -> Option<i16> {
        if self.is_unavailable() {
            None
        } else {
            Some(self.as_sec())
        }
    }

    /// create ETSI DeltaTime from seconds, clamping at min. and max. bounds
    #[must_use]
    pub fn from_sec(value: i16) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let etsi_val = (value / 10) as i8;

        Self(etsi_val.clamp(-121, 120))
    }

    /// create ETSI type with "unavailable" value
    pub fn unavailable() -> Self {
        Self(-122)
    }

    /// determines if the ETSI value is special "unavailable" value
    pub fn is_unavailable(&self) -> bool {
        self.0 == -122
    }
}

#[cfg(feature = "_dsrc_2_2_1")]
impl From<&dsrc_2_2_1::etsi_its_dsrc::DeltaTime> for i16 {
    fn from(other: &dsrc_2_2_1::etsi_its_dsrc::DeltaTime) -> i16 {
        other.as_sec()
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<dsrc_2_2_1::etsi_its_dsrc::DeltaTime> for i16 {
    fn from(other: dsrc_2_2_1::etsi_its_dsrc::DeltaTime) -> i16 {
        other.as_sec()
    }
}

// DSecond: unit milliseconds, 65535 for unavailable
#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::DSecond {
    /// convert ETSI DeltaTime to milliseconds
    #[must_use]
    pub fn as_millis(&self) -> u16 {
        self.0
    }

    /// convert ETSI DeltaTime to milliseconds or `None` if "unavailable"
    #[must_use]
    pub fn try_as_millis(&self) -> Option<u16> {
        if self.is_unavailable() {
            None
        } else {
            Some(self.as_millis())
        }
    }

    /// create ETSI DSecond from milliseconds
    ///
    /// # Errors
    /// human-readable string when input value is out of bounds
    pub fn from_millis(value: u16) -> Result<Self, alloc::string::String> {
        // ASN.1 bounds are bigger than allowed values (0..59999 for normal values, 60000..60999 for leap seconds)

        if value > 60999 {
            return Err(alloc::format!("Value out of bounds"));
        }

        Ok(Self(value))
    }

    /// create ETSI type with "unavailable" value
    pub fn unavailable() -> Self {
        Self(65535)
    }

    /// determines if the ETSI value is special "unavailable" value
    pub fn is_unavailable(&self) -> bool {
        self.0 == 65535
    }
}

#[cfg(feature = "_dsrc_2_2_1")]
impl From<&dsrc_2_2_1::etsi_its_dsrc::DSecond> for u16 {
    fn from(other: &dsrc_2_2_1::etsi_its_dsrc::DSecond) -> u16 {
        other.as_millis()
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<dsrc_2_2_1::etsi_its_dsrc::DSecond> for u16 {
    fn from(other: dsrc_2_2_1::etsi_its_dsrc::DSecond) -> u16 {
        other.as_millis()
    }
}

// TimeMark: unit 1/10 of a second, 36001 for unknown, 36000 for out-of-range
#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::TimeMark {
    const CONV_FACTOR: u32 = 100;
    const UNKNOWN: u16 = 36001;
    const OUT_OF_RANGE: u16 = 36000;

    /// convert ETSI TimeMark to milliseconds
    #[must_use]
    pub fn as_millis(&self) -> u32 {
        self.0 as u32 * Self::CONV_FACTOR
    }

    /// convert ETSI TimeMark to milliseconds or `None` if "unknown"
    #[must_use]
    pub fn try_as_millis(&self) -> Option<u32> {
        if self.is_unknown() {
            None
        } else {
            Some(self.as_millis())
        }
    }

    /// create ETSI TimeMark from milliseconds
    ///
    /// # Errors
    /// human-readable string when input value is out of bounds
    pub fn from_millis(value: u32) -> Result<Self, alloc::string::String> {
        #[allow(clippy::cast_possible_truncation)]
        let etsi_val = (value / Self::CONV_FACTOR) as u16;

        // ASN.1 bounds are bigger than allowed values (0..35990 for normal values, 35991..35999 for leap seconds)
        if etsi_val > 35999 {
            return Err(alloc::format!("Value out of bounds"));
        }

        Ok(Self(etsi_val))
    }

    /// create ETSI type with "unknown" value
    pub fn unknown() -> Self {
        Self(Self::UNKNOWN)
    }

    /// determines if the ETSI value is special "unknown" value
    pub fn is_unknown(&self) -> bool {
        self.0 == Self::UNKNOWN
    }

    /// create ETSI type with "out-of-range" value
    pub fn out_of_range() -> Self {
        Self(Self::OUT_OF_RANGE)
    }

    /// determines if the ETSI value is special "out-of-range" value
    pub fn is_out_of_range(&self) -> bool {
        self.0 == Self::OUT_OF_RANGE
    }
}

#[cfg(feature = "_dsrc_2_2_1")]
impl From<&dsrc_2_2_1::etsi_its_dsrc::TimeMark> for u32 {
    fn from(other: &dsrc_2_2_1::etsi_its_dsrc::TimeMark) -> u32 {
        other.as_millis()
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<dsrc_2_2_1::etsi_its_dsrc::TimeMark> for u32 {
    fn from(other: dsrc_2_2_1::etsi_its_dsrc::TimeMark) -> u32 {
        other.as_millis()
    }
}

/// Create conversions for PathDeltaTime
///
/// Value range is 1..65535 (extensible), so u16 fits currently, but u32 should have enough space for extensions
#[cfg(any(feature = "_cdd_2_2_1", feature = "_cdd_1_3_1_1"))]
macro_rules! path_delta_time_to_millis {
    ($t:ty) => {
        impl $t {
            const ETSI_TO_MS_FACTOR: u32 = 10; // unit: 10 milliseconds

            /// convert ETSI PathDeltaTime to milliseconds
            #[must_use]
            pub fn as_millis(&self) -> u32 {
                let etsi_val: i64 = (&self.0).try_into().unwrap_or_default();

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = etsi_val as u32;
                etsi_val * Self::ETSI_TO_MS_FACTOR
            }

            /// create ETSI PathDeltaTime from milliseconds
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_millis(value: u32) -> Result<Self, alloc::string::String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = value / Self::ETSI_TO_MS_FACTOR;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(alloc::format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val.into()))
            }
        }
    };
}

#[cfg(feature = "_cdd_1_3_1_1")]
path_delta_time_to_millis!(cdd_1_3_1_1::its_container::PathDeltaTime);
#[cfg(feature = "_cdd_2_2_1")]
path_delta_time_to_millis!(cdd_2_2_1::etsi_its_cdd::PathDeltaTime);

// MinuteOfTheYear: unit minute, 527040 for invalid
#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::MinuteOfTheYear {
    /// create ETSI MinuteOfTheYear with "invalid" value
    pub fn invalid() -> Self {
        Self(527040)
    }

    /// determines if the ETSI value is special "invalid" value
    pub fn is_invalid(&self) -> bool {
        self.0 == 527040
    }
}

// MsgCount 0..127
#[cfg(feature = "_dsrc_2_2_1")]
impl crate::standards::dsrc_2_2_1::etsi_its_dsrc::MsgCount {
    pub fn increment(&self) -> Self {
        Self((self.0 + 1) % 128)
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<u8> for dsrc_2_2_1::etsi_its_dsrc::MsgCount {
    fn from(value: u8) -> Self {
        Self(value % 128)
    }
}

// RequestID 0..255
#[cfg(feature = "_dsrc_2_2_1")]
impl crate::standards::dsrc_2_2_1::etsi_its_dsrc::RequestID {
    pub fn increment(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<u8> for dsrc_2_2_1::etsi_its_dsrc::RequestID {
    // for convenience and interface unification only
    fn from(value: u8) -> Self {
        Self(value)
    }
}

// convenience getters

#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::SpeedLimitList {
    /// Extracts a certain speed limit in m/s, if existing
    pub fn get_speed_limit_mps(
        &self,
        limit_type: dsrc_2_2_1::etsi_its_dsrc::SpeedLimitType,
    ) -> Option<f32> {
        self.0.iter().find_map(|item| {
            if item.r_type == limit_type {
                Some(item.speed.as_mps())
            } else {
                None
            }
        })
    }
}

#[cfg(feature = "_cdd_2_2_1")]
impl cdd_2_2_1::etsi_its_cdd::CauseCodeChoice {
    /// Converts a Cause Code to an integer tuple of cause code and sub cause code ID
    pub fn to_u8_tuple(&self) -> (u8, u8) {
        match self {
            Self::reserved0(scc) => (0, scc.0),
            Self::trafficCondition1(scc) => (1, scc.0),
            Self::accident2(scc) => (2, scc.0),
            Self::roadworks3(scc) => (3, scc.0),
            Self::reserved4(scc) => (4, scc.0),
            Self::impassability5(scc) => (5, scc.0),
            Self::adverseWeatherCondition_Adhesion6(scc) => (6, scc.0),
            Self::aquaplaning7(scc) => (7, scc.0),
            Self::reserved8(scc) => (8, scc.0),
            Self::hazardousLocation_SurfaceCondition9(scc) => (9, scc.0),
            Self::hazardousLocation_ObstacleOnTheRoad10(scc) => (10, scc.0),
            Self::hazardousLocation_AnimalOnTheRoad11(scc) => (11, scc.0),
            Self::humanPresenceOnTheRoad12(scc) => (12, scc.0),
            Self::reserved13(scc) => (13, scc.0),
            Self::wrongWayDriving14(scc) => (14, scc.0),
            Self::rescueAndRecoveryWorkInProgress15(scc) => (15, scc.0),
            Self::reserved16(scc) => (16, scc.0),
            Self::adverseWeatherCondition_ExtremeWeatherCondition17(scc) => (17, scc.0),
            Self::adverseWeatherCondition_Visibility18(scc) => (18, scc.0),
            Self::adverseWeatherCondition_Precipitation19(scc) => (19, scc.0),
            Self::violence20(scc) => (20, scc.0),
            Self::reserved21(scc) => (21, scc.0),
            Self::reserved22(scc) => (22, scc.0),
            Self::reserved23(scc) => (23, scc.0),
            Self::reserved24(scc) => (24, scc.0),
            Self::reserved25(scc) => (25, scc.0),
            Self::slowVehicle26(scc) => (26, scc.0),
            Self::dangerousEndOfQueue27(scc) => (27, scc.0),
            Self::publicTransportVehicleApproaching28(scc) => (28, scc.0),
            Self::reserved29(scc) => (29, scc.0),
            Self::reserved30(scc) => (30, scc.0),
            Self::reserved31(scc) => (31, scc.0),
            Self::reserved32(scc) => (32, scc.0),
            Self::reserved33(scc) => (33, scc.0),
            Self::reserved34(scc) => (34, scc.0),
            Self::reserved35(scc) => (35, scc.0),
            Self::reserved36(scc) => (36, scc.0),
            Self::reserved37(scc) => (37, scc.0),
            Self::reserved38(scc) => (38, scc.0),
            Self::reserved39(scc) => (39, scc.0),
            Self::reserved40(scc) => (40, scc.0),
            Self::reserved41(scc) => (41, scc.0),
            Self::reserved42(scc) => (42, scc.0),
            Self::reserved43(scc) => (43, scc.0),
            Self::reserved44(scc) => (44, scc.0),
            Self::reserved45(scc) => (45, scc.0),
            Self::reserved46(scc) => (46, scc.0),
            Self::reserved47(scc) => (47, scc.0),
            Self::reserved48(scc) => (48, scc.0),
            Self::reserved49(scc) => (49, scc.0),
            Self::reserved50(scc) => (50, scc.0),
            Self::reserved51(scc) => (51, scc.0),
            Self::reserved52(scc) => (52, scc.0),
            Self::reserved53(scc) => (53, scc.0),
            Self::reserved54(scc) => (54, scc.0),
            Self::reserved55(scc) => (55, scc.0),
            Self::reserved56(scc) => (56, scc.0),
            Self::reserved57(scc) => (57, scc.0),
            Self::reserved58(scc) => (58, scc.0),
            Self::reserved59(scc) => (59, scc.0),
            Self::reserved60(scc) => (60, scc.0),
            Self::reserved61(scc) => (61, scc.0),
            Self::reserved62(scc) => (62, scc.0),
            Self::reserved63(scc) => (63, scc.0),
            Self::reserved64(scc) => (64, scc.0),
            Self::reserved65(scc) => (65, scc.0),
            Self::reserved66(scc) => (66, scc.0),
            Self::reserved67(scc) => (67, scc.0),
            Self::reserved68(scc) => (68, scc.0),
            Self::reserved69(scc) => (69, scc.0),
            Self::reserved70(scc) => (70, scc.0),
            Self::reserved71(scc) => (71, scc.0),
            Self::reserved72(scc) => (72, scc.0),
            Self::reserved73(scc) => (73, scc.0),
            Self::reserved74(scc) => (74, scc.0),
            Self::reserved75(scc) => (75, scc.0),
            Self::reserved76(scc) => (76, scc.0),
            Self::reserved77(scc) => (77, scc.0),
            Self::reserved78(scc) => (78, scc.0),
            Self::reserved79(scc) => (79, scc.0),
            Self::reserved80(scc) => (80, scc.0),
            Self::reserved81(scc) => (81, scc.0),
            Self::reserved82(scc) => (82, scc.0),
            Self::reserved83(scc) => (83, scc.0),
            Self::reserved84(scc) => (84, scc.0),
            Self::reserved85(scc) => (85, scc.0),
            Self::reserved86(scc) => (86, scc.0),
            Self::reserved87(scc) => (87, scc.0),
            Self::reserved88(scc) => (88, scc.0),
            Self::reserved89(scc) => (89, scc.0),
            Self::reserved90(scc) => (90, scc.0),
            Self::vehicleBreakdown91(scc) => (91, scc.0),
            Self::postCrash92(scc) => (92, scc.0),
            Self::humanProblem93(scc) => (93, scc.0),
            Self::stationaryVehicle94(scc) => (94, scc.0),
            Self::emergencyVehicleApproaching95(scc) => (95, scc.0),
            Self::hazardousLocation_DangerousCurve96(scc) => (96, scc.0),
            Self::collisionRisk97(scc) => (97, scc.0),
            Self::signalViolation98(scc) => (98, scc.0),
            Self::dangerousSituation99(scc) => (99, scc.0),
            Self::railwayLevelCrossing100(scc) => (100, scc.0),
            Self::reserved101(scc) => (101, scc.0),
            Self::reserved102(scc) => (102, scc.0),
            Self::reserved103(scc) => (103, scc.0),
            Self::reserved104(scc) => (104, scc.0),
            Self::reserved105(scc) => (105, scc.0),
            Self::reserved106(scc) => (106, scc.0),
            Self::reserved107(scc) => (107, scc.0),
            Self::reserved108(scc) => (108, scc.0),
            Self::reserved109(scc) => (109, scc.0),
            Self::reserved110(scc) => (110, scc.0),
            Self::reserved111(scc) => (111, scc.0),
            Self::reserved112(scc) => (112, scc.0),
            Self::reserved113(scc) => (113, scc.0),
            Self::reserved114(scc) => (114, scc.0),
            Self::reserved115(scc) => (115, scc.0),
            Self::reserved116(scc) => (116, scc.0),
            Self::reserved117(scc) => (117, scc.0),
            Self::reserved118(scc) => (118, scc.0),
            Self::reserved119(scc) => (119, scc.0),
            Self::reserved120(scc) => (120, scc.0),
            Self::reserved121(scc) => (121, scc.0),
            Self::reserved122(scc) => (122, scc.0),
            Self::reserved123(scc) => (123, scc.0),
            Self::reserved124(scc) => (124, scc.0),
            Self::reserved125(scc) => (125, scc.0),
            Self::reserved126(scc) => (126, scc.0),
            Self::reserved127(scc) => (127, scc.0),
            Self::reserved128(scc) => (128, scc.0),
        }
    }
}
